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
    #[allow(dead_code)]
    InvalidFieldReference { field: String, reason: String },
    #[allow(dead_code)]
    InvalidKeyword { keyword: String, suggestion: Option<String> },
    #[allow(dead_code)]
    StructuralError { context: String, issue: String },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidSyntax(msg) => write!(f, "Invalid syntax: {msg}"),
            ParseError::UnexpectedToken(msg) => write!(f, "Unexpected token: {msg}"),
            ParseError::MissingIdentifier(msg) => write!(f, "Missing identifier: {msg}"),
            ParseError::InvalidFieldReference { field, reason } => {
                write!(f, "Invalid field reference '{field}': {reason}")
            }
            ParseError::InvalidKeyword { keyword, suggestion } => {
                if let Some(suggestion) = suggestion {
                    write!(f, "Invalid keyword '{keyword}'. Did you mean '{suggestion}'?")
                } else {
                    write!(f, "Invalid keyword '{keyword}' is not a valid DataLang construct")
                }
            }
            ParseError::StructuralError { context, issue } => {
                write!(f, "Structural error in {context}: {issue}")
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl FieldReference {
    /// Check if this field is included (+)
    #[allow(dead_code)]
    pub fn is_included(&self) -> bool {
        self.is_included
    }

    /// Check if this field is excluded (-)
    #[allow(dead_code)]
    pub fn is_excluded(&self) -> bool {
        !self.is_included
    }

    /// Get the field name without namespace
    #[allow(dead_code)]
    pub fn field_name(&self) -> &str {
        &self.name
    }

    /// Get the full field reference (namespace::name or just name)
    #[allow(dead_code)]
    pub fn full_name(&self) -> String {
        match &self.namespace {
            Some(ns) => format!("{}::{}", ns, self.name),
            None => self.name.clone(),
        }
    }

    /// Check if this field has a namespace
    #[allow(dead_code)]
    pub fn has_namespace(&self) -> bool {
        self.namespace.is_some()
    }

    /// Get the namespace if present
    #[allow(dead_code)]
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    #[allow(dead_code)]
    pub fn parse_from_str(input: &str) -> std::result::Result<Self, ParseError> {
        let trimmed = input.trim();

        // Parse + or -
        let (is_included, rest) = if let Some(rest) = trimmed.strip_prefix('+') {
            (true, rest.trim())
        } else if let Some(rest) = trimmed.strip_prefix('-') {
            (false, rest.trim())
        } else {
            return Err(ParseError::InvalidFieldReference {
                field: input.to_string(),
                reason: "Field references must start with + (include) or - (exclude)".to_string(),
            });
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
    /// Get all dictionary items
    #[allow(dead_code)]
    pub fn dictionaries(&self) -> impl Iterator<Item = &str> {
        self.items.iter().filter_map(|item| match item {
            DataLangItem::Dictionary { name } => Some(name.as_str()),
            _ => None,
        })
    }

    /// Get all term items
    #[allow(dead_code)]
    pub fn terms(&self) -> impl Iterator<Item = (&str, &[FieldReference])> {
        self.items.iter().filter_map(|item| match item {
            DataLangItem::Term { name, fields } => Some((name.as_str(), fields.as_slice())),
            _ => None,
        })
    }

    /// Get all struct items
    #[allow(dead_code)]
    pub fn structs(&self) -> impl Iterator<Item = (&str, &[FieldReference])> {
        self.items.iter().filter_map(|item| match item {
            DataLangItem::Struct { name, fields } => Some((name.as_str(), fields.as_slice())),
            _ => None,
        })
    }

    /// Get all import items
    #[allow(dead_code)]
    pub fn imports(&self) -> impl Iterator<Item = &str> {
        self.items.iter().filter_map(|item| match item {
            DataLangItem::Import { module } => Some(module.as_str()),
            _ => None,
        })
    }

    /// Get all items of a specific type
    #[allow(dead_code)]
    pub fn items_by_type(&self, item_type: &str) -> Vec<&DataLangItem> {
        self.items.iter().filter(|item| {
            match (item_type, item) {
                ("dictionary", DataLangItem::Dictionary { .. }) => true,
                ("term", DataLangItem::Term { .. }) => true,
                ("struct", DataLangItem::Struct { .. }) => true,
                ("import", DataLangItem::Import { .. }) => true,
                _ => false,
            }
        }).collect()
    }

    /// Check if file contains any items of a specific type
    #[allow(dead_code)]
    pub fn has_item_type(&self, item_type: &str) -> bool {
        !self.items_by_type(item_type).is_empty()
    }

    /// Get field references for a specific term or struct
    #[allow(dead_code)]
    pub fn get_fields(&self, name: &str) -> Option<&[FieldReference]> {
        self.items.iter().find_map(|item| match item {
            DataLangItem::Term { name: item_name, fields } if item_name == name => Some(fields.as_slice()),
            DataLangItem::Struct { name: item_name, fields } if item_name == name => Some(fields.as_slice()),
            _ => None,
        })
    }

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
                                    return Err(ParseError::InvalidFieldReference {
                                        field: field_text.to_string(),
                                        reason: "Fields must start with + (include) or - (exclude)".to_string(),
                                    });
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
                    // In DataLang, we only allow valid keywords: dictionary, term, import
                    // Everything else should be treated as a struct definition ONLY if it starts with uppercase
                    let keyword = tokens[0];
                    let name = keyword.to_string();

                    // Suggestions and invalid keywords are a bad idea we should fix this
                    let suggestion = match keyword {
                        "function" | "fn" => Some("term"),
                        "struct" | "class" | "interface" => Some("term with struct-like syntax"),
                        "enum" => Some("term with variants"),
                        "use" => Some("import"),
                        "type" => Some("term"),
                        "test" | "describe" | "it" => Some("term"),
                        _ => None,
                    };

                    let invalid_keywords = [
                        "function", "fn", "struct", "impl", "let", "const", "static", "use", "mod",
                        "var", "class", "interface", "enum", "type", "pub", "priv", "private", 
                        "public", "return", "if", "else", "while", "for", "match", "loop",
                        "test", "describe", "it", "expect", "assert", "should", "spec"
                    ];

                    if invalid_keywords.contains(&keyword) {
                        return Err(ParseError::InvalidKeyword {
                            keyword: keyword.to_string(),
                            suggestion: suggestion.map(|s| s.to_string()),
                        });
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
                                return Err(ParseError::InvalidFieldReference {
                                    field: field_text.to_string(),
                                    reason: "Fields must start with + (include) or - (exclude)".to_string(),
                                });
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
