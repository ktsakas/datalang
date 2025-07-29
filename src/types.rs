// DataLang types and parsing logic
// Contains shared types used by both build.rs (string parsing) and lib.rs (syn parsing)
// #[allow(dead_code)] is used throughout because different compilation contexts use different methods:
// - build.rs uses parse_from_str() for text processing
// - lib.rs uses Parse trait implementations for macro processing
// - Some fields/variants are only accessed in specific contexts

#[derive(Debug, Clone)]
pub struct FieldReference {
    #[allow(dead_code)]
    pub is_included: bool, // true for +, false for -
    #[allow(dead_code)]
    pub namespace: Option<String>,
    #[allow(dead_code)]
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum DataLangItem {
    Dictionary {
        name: String,
    },
    Term {
        name: String,
        #[allow(dead_code)]
        fields: Vec<FieldReference>,
    },
    Import {
        module: String,
    },
    Struct {
        name: String,
        #[allow(dead_code)]
        fields: Vec<FieldReference>,
    },
}

#[derive(Debug, Clone)]
pub struct DataLangFile {
    pub items: Vec<DataLangItem>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseError {
    InvalidSyntax(String),
    #[allow(dead_code)]
    UnexpectedToken(String),
    MissingIdentifier(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidSyntax(msg) => write!(f, "Invalid syntax: {msg}"),
            ParseError::UnexpectedToken(msg) => write!(f, "Unexpected token: {msg}"),
            ParseError::MissingIdentifier(msg) => write!(f, "Missing identifier: {msg}"),
        }
    }
}

impl std::error::Error for ParseError {}

impl FieldReference {
    #[allow(dead_code)]
    pub fn parse_from_str(input: &str) -> std::result::Result<Self, ParseError> {
        let trimmed = input.trim();

        // Parse + or -
        let (is_included, rest) = if let Some(rest) = trimmed.strip_prefix('+') {
            (true, rest.trim())
        } else if let Some(rest) = trimmed.strip_prefix('-') {
            (false, rest.trim())
        } else {
            return Err(ParseError::InvalidSyntax(
                "Expected + or - before field reference".to_string(),
            ));
        };

        // Parse field reference (Name or Base::Name)
        if let Some((namespace, name)) = rest.split_once("::") {
            Ok(FieldReference {
                is_included,
                namespace: Some(namespace.trim().to_string()),
                name: name.trim().to_string(),
            })
        } else {
            Ok(FieldReference {
                is_included,
                namespace: None,
                name: rest.to_string(),
            })
        }
    }
}

impl DataLangFile {
    #[allow(dead_code)]
    pub fn parse_from_str(input: &str) -> std::result::Result<Self, ParseError> {
        let mut items = Vec::new();
        let lines: Vec<&str> = input.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("//") {
                i += 1;
                continue;
            }

            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.is_empty() {
                i += 1;
                continue;
            }

            match tokens[0] {
                "dictionary" => {
                    if tokens.len() < 2 {
                        return Err(ParseError::MissingIdentifier(
                            "Expected dictionary name".to_string(),
                        ));
                    }
                    let name = tokens[1].to_string();
                    items.push(DataLangItem::Dictionary { name });
                    i += 1;
                }
                "import" => {
                    if tokens.len() < 2 {
                        return Err(ParseError::MissingIdentifier(
                            "Expected module name after import".to_string(),
                        ));
                    }
                    let module = tokens[1].to_string();
                    items.push(DataLangItem::Import { module });
                    i += 1;
                }
                "term" => {
                    if tokens.len() < 2 {
                        return Err(ParseError::MissingIdentifier(
                            "Expected term name".to_string(),
                        ));
                    }
                    let name = tokens[1].to_string();

                    // Look for "has" keyword and opening brace
                    let has_fields = tokens.len() > 2 && tokens[2] == "has";

                    // Find the opening brace
                    let mut brace_line = i;
                    let mut found_brace = false;

                    // Check if brace is on the same line
                    if line.contains('{') {
                        found_brace = true;
                    } else {
                        // Look for brace on subsequent lines
                        for (j, line) in lines.iter().enumerate().skip(i + 1) {
                            if line.trim().contains('{') {
                                brace_line = j;
                                found_brace = true;
                                break;
                            }
                            if !line.trim().is_empty() && line.trim() != "has" {
                                break;
                            }
                        }
                    }

                    if !found_brace {
                        return Err(ParseError::InvalidSyntax(format!(
                            "Expected opening brace after term {name}"
                        )));
                    }

                    // Parse fields if this is a composite term
                    let mut fields = Vec::new();
                    if has_fields {
                        let mut field_line = brace_line + 1;
                        while field_line < lines.len() {
                            let field_text = lines[field_line].trim();
                            if field_text == "}" {
                                break;
                            }
                            if !field_text.is_empty() && !field_text.starts_with("//") {
                                if field_text.starts_with('+') || field_text.starts_with('-') {
                                    fields.push(FieldReference::parse_from_str(field_text)?);
                                } else {
                                    return Err(ParseError::InvalidSyntax(format!(
                                        "Invalid field syntax: '{}'. Fields must start with + or -",
                                        field_text
                                    )));
                                }
                            }
                            field_line += 1;
                        }
                        i = field_line + 1;
                    } else {
                        // Simple term, find the closing brace
                        let mut close_line = brace_line + 1;
                        while close_line < lines.len() {
                            if lines[close_line].trim() == "}" {
                                break;
                            }
                            close_line += 1;
                        }
                        i = close_line + 1;
                    }

                    items.push(DataLangItem::Term { name, fields });
                }
                _ => {
                    // Only allow valid identifiers for struct definitions
                    let name = tokens[0].to_string();

                    // Reject known invalid keywords
                    if [
                        "function",
                        "fn",
                        "struct",
                        "impl",
                        "let",
                        "const",
                        "static",
                        "use",
                        "mod",
                        "var",
                        "class",
                        "interface",
                        "enum",
                        "type",
                        "pub",
                        "priv",
                        "private",
                        "public",
                        "return",
                        "if",
                        "else",
                        "while",
                        "for",
                        "match",
                        "loop",
                    ]
                    .contains(&tokens[0])
                    {
                        return Err(ParseError::InvalidSyntax(format!(
                            "Invalid DataLang syntax: '{}' is not a valid DataLang construct",
                            tokens[0]
                        )));
                    }

                    // Find the opening brace
                    let mut brace_line = i;
                    let mut found_brace = false;

                    if line.contains('{') {
                        found_brace = true;
                    } else {
                        for (j, line) in lines.iter().enumerate().skip(i + 1) {
                            if line.trim().contains('{') {
                                brace_line = j;
                                found_brace = true;
                                break;
                            }
                        }
                    }

                    if !found_brace {
                        return Err(ParseError::InvalidSyntax(format!(
                            "Expected opening brace after struct {name}"
                        )));
                    }

                    // Parse fields
                    let mut fields = Vec::new();
                    let mut field_line = brace_line + 1;
                    while field_line < lines.len() {
                        let field_text = lines[field_line].trim();
                        if field_text == "}" {
                            break;
                        }
                        if !field_text.is_empty() && !field_text.starts_with("//") {
                            if field_text.starts_with('+') || field_text.starts_with('-') {
                                fields.push(FieldReference::parse_from_str(field_text)?);
                            } else {
                                return Err(ParseError::InvalidSyntax(format!(
                                    "Invalid field syntax: '{}'. Fields must start with + or -",
                                    field_text
                                )));
                            }
                        }
                        field_line += 1;
                    }

                    items.push(DataLangItem::Struct { name, fields });
                    i = field_line + 1;
                }
            }
        }

        Ok(DataLangFile { items })
    }

    #[allow(dead_code)]
    pub fn validate(&self) -> std::result::Result<(), ParseError> {
        // Basic validation - ensure no empty names
        for item in &self.items {
            match item {
                DataLangItem::Dictionary { name } => {
                    if name.is_empty() {
                        return Err(ParseError::InvalidSyntax(
                            "Dictionary name cannot be empty".to_string(),
                        ));
                    }
                }
                DataLangItem::Term { name, .. } => {
                    if name.is_empty() {
                        return Err(ParseError::InvalidSyntax(
                            "Term name cannot be empty".to_string(),
                        ));
                    }
                }
                DataLangItem::Import { module } => {
                    if module.is_empty() {
                        return Err(ParseError::InvalidSyntax(
                            "Import module cannot be empty".to_string(),
                        ));
                    }
                }
                DataLangItem::Struct { name, .. } => {
                    if name.is_empty() {
                        return Err(ParseError::InvalidSyntax(
                            "Struct name cannot be empty".to_string(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
