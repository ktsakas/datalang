use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Token, braced};
use syn::parse::{Parse, ParseStream, Result};

// Let's try to use our own macro (this will fail)
/*
datalang! {
    term SelfTest {
    }
}
*/

#[derive(Debug, Clone)]
struct FieldReference {
    is_included: bool, // true for +, false for -
    namespace: Option<String>,
    name: String,
}

#[derive(Debug, Clone)]
enum DataLangItem {
    Dictionary {
        name: Ident,
    },
    Term {
        name: Ident,
        fields: Vec<FieldReference>,
    },
    Import {
        module: Ident,
    },
    Struct {
        name: Ident,
        fields: Vec<FieldReference>,
    },
}

struct DataLangFile {
    items: Vec<DataLangItem>,
}

impl Parse for FieldReference {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse + or -
        let is_included = if input.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            true
        } else if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            false
        } else {
            return Err(input.error("Expected + or - before field reference"));
        };
        
        // Parse field reference (Name or Base::Name)
        let first_part: Ident = input.parse()?;
        
        if input.peek(Token![::]) {
            input.parse::<Token![::]>()?;
            let second_part: Ident = input.parse()?;
            Ok(FieldReference {
                is_included,
                namespace: Some(first_part.to_string()),
                name: second_part.to_string(),
            })
        } else {
            Ok(FieldReference {
                is_included,
                namespace: None,
                name: first_part.to_string(),
            })
        }
    }
}

impl Parse for DataLangFile {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items = Vec::new();
        
        while !input.is_empty() {
            if input.peek(syn::Ident) {
                let _lookahead = input.lookahead1();
                let first_ident: Ident = input.fork().parse()?;
                
                match first_ident.to_string().as_str() {
                    "dictionary" => {
                        input.parse::<syn::Ident>()?; // consume "dictionary"
                        let name: Ident = input.parse()?;
                        items.push(DataLangItem::Dictionary { name });
                    }
                    "term" => {
                        input.parse::<syn::Ident>()?; // consume "term"
                        let name: Ident = input.parse()?;
                        
                        if input.peek(syn::Ident) && input.peek2(syn::token::Brace) {
                            // term Name has { ... }
                            let has_keyword: Ident = input.parse()?;
                            if has_keyword.to_string() != "has" {
                                return Err(input.error("Expected 'has' after term name"));
                            }
                            
                            let content;
                            braced!(content in input);
                            let mut fields = Vec::new();
                            
                            while !content.is_empty() {
                                let field: FieldReference = content.parse()?;
                                fields.push(field);
                            }
                            
                            items.push(DataLangItem::Term { name, fields });
                        } else {
                            // term Name { }
                            let _content;
                            braced!(_content in input);
                            items.push(DataLangItem::Term { name, fields: Vec::new() });
                        }
                    }
                    "import" => {
                        input.parse::<syn::Ident>()?; // consume "import"
                        let module: Ident = input.parse()?;
                        items.push(DataLangItem::Import { module });
                    }
                    _ => {
                        // Assume it's a struct definition
                        let name: Ident = input.parse()?;
                        let content;
                        braced!(content in input);
                        let mut fields = Vec::new();
                        
                        while !content.is_empty() {
                            let field: FieldReference = content.parse()?;
                            fields.push(field);
                        }
                        
                        items.push(DataLangItem::Struct { name, fields });
                    }
                }
            } else {
                break;
            }
        }
        
        Ok(DataLangFile { items })
    }
}

#[proc_macro]
pub fn datalang(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DataLangFile);
    
    let mut generated_code = Vec::new();
    
    for item in parsed.items {
        match item {
            DataLangItem::Dictionary { name } => {
                // Dictionary declarations don't generate code directly
                // In a real implementation, you'd track these for namespace resolution
                println!("cargo:warning=Processing dictionary: {}", name);
            }
            DataLangItem::Term { name, fields } => {
                if fields.is_empty() {
                    // Simple term - generate a basic struct
                    let snake_name = name.to_string().to_lowercase();
                    let field_name = Ident::new(&snake_name, name.span());
                    
                    generated_code.push(quote! {
                        #[derive(Debug, Clone)]
                        pub struct #name {
                            pub #field_name: String,
                        }
                        
                        impl #name {
                            pub fn new() -> Self {
                                Self {
                                    #field_name: String::new(),
                                }
                            }
                        }
                    });
                } else {
                    // Composite term - generate struct with referenced fields
                    let field_names: Vec<Ident> = fields.iter()
                        .filter(|f| f.is_included)
                        .map(|f| {
                            // Use the namespace if present for future namespacing logic
                            let field_name = if let Some(_namespace) = &f.namespace {
                                // For now, just use the name part, but namespace is available
                                f.name.to_lowercase()
                            } else {
                                f.name.to_lowercase()
                            };
                            Ident::new(&field_name, name.span())
                        })
                        .collect();
                    
                    generated_code.push(quote! {
                        #[derive(Debug, Clone)]
                        pub struct #name {
                            #(pub #field_names: String,)*
                        }
                        
                        impl #name {
                            pub fn new() -> Self {
                                Self {
                                    #(#field_names: String::new(),)*
                                }
                            }
                        }
                    });
                }
            }
            DataLangItem::Import { module } => {
                // Import declarations don't generate code directly
                // In a real implementation, you'd use these for module resolution
                println!("cargo:warning=Processing import: {}", module);
            }
            DataLangItem::Struct { name, fields } => {
                // Regular struct - process field inclusions/exclusions
                let included_fields: Vec<String> = fields.iter()
                    .filter(|f| f.is_included)
                    .map(|f| {
                        // Use the namespace if present for future namespacing logic
                        if let Some(_namespace) = &f.namespace {
                            // For now, just use the name part, but namespace is available
                            f.name.to_lowercase()
                        } else {
                            f.name.to_lowercase()
                        }
                    })
                    .collect();
                
                let field_idents: Vec<Ident> = included_fields.iter()
                    .map(|f| Ident::new(f, name.span()))
                    .collect();
                
                generated_code.push(quote! {
                    #[derive(Debug, Clone)]
                    pub struct #name {
                        #(pub #field_idents: String,)*
                    }
                    
                    impl #name {
                        pub fn new() -> Self {
                            Self {
                                #(#field_idents: String::new(),)*
                            }
                        }
                    }
                });
            }
        }
    }
    
    let expanded = quote! {
        #(#generated_code)*
    };
    
    TokenStream::from(expanded)
}

#[proc_macro_derive(DataLang)]
pub fn derive_datalang(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    let expanded = quote! {
        impl #name {
            pub fn from_datalang() -> Self {
                Self::new()
            }
        }
    };
    
    TokenStream::from(expanded)
}
