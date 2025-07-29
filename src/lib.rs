use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{DeriveInput, parse_macro_input};

mod types;
use types::{DataLangFile, DataLangItem, FieldReference};

impl Parse for DataLangFile {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items = Vec::new();

        while !input.is_empty() {
            if input.peek(syn::Ident) {
                let _lookahead = input.lookahead1();
                let first_ident: syn::Ident = input.fork().parse()?;

                match first_ident.to_string().as_str() {
                    "dictionary" => {
                        input.parse::<syn::Ident>()?; // consume "dictionary"
                        let name: syn::Ident = input.parse()?;
                        items.push(DataLangItem::Dictionary {
                            name: name.to_string(),
                        });
                    }
                    "term" => {
                        input.parse::<syn::Ident>()?; // consume "term"
                        let name: syn::Ident = input.parse()?;

                        if input.peek(syn::Ident) && input.peek2(syn::token::Brace) {
                            // term Name has { ... }
                            let has_keyword: syn::Ident = input.parse()?;
                            if has_keyword.to_string() != "has" {
                                return Err(input.error("Expected 'has' after term name"));
                            }

                            let content;
                            syn::braced!(content in input);
                            let mut fields = Vec::new();

                            while !content.is_empty() {
                                let field: FieldReference = content.parse()?;
                                fields.push(field);
                            }

                            items.push(DataLangItem::Term {
                                name: name.to_string(),
                                fields,
                            });
                        } else {
                            // term Name { }
                            let _content;
                            syn::braced!(_content in input);
                            items.push(DataLangItem::Term {
                                name: name.to_string(),
                                fields: Vec::new(),
                            });
                        }
                    }
                    "import" => {
                        input.parse::<syn::Ident>()?; // consume "import"
                        let module: syn::Ident = input.parse()?;
                        items.push(DataLangItem::Import {
                            module: module.to_string(),
                        });
                    }
                    _ => {
                        // Assume it's a struct definition
                        let name: syn::Ident = input.parse()?;
                        let content;
                        syn::braced!(content in input);
                        let mut fields = Vec::new();

                        while !content.is_empty() {
                            let field: FieldReference = content.parse()?;
                            fields.push(field);
                        }

                        items.push(DataLangItem::Struct {
                            name: name.to_string(),
                            fields,
                        });
                    }
                }
            } else {
                break;
            }
        }

        Ok(DataLangFile { items })
    }
}

impl Parse for FieldReference {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse + or -
        let is_included = if input.peek(syn::Token![+]) {
            input.parse::<syn::Token![+]>()?;
            true
        } else if input.peek(syn::Token![-]) {
            input.parse::<syn::Token![-]>()?;
            false
        } else {
            return Err(input.error("Expected + or - before field reference"));
        };

        // Parse field reference (Name or Base::Name)
        let first_part: syn::Ident = input.parse()?;

        if input.peek(syn::Token![::]) {
            input.parse::<syn::Token![::]>()?;
            let second_part: syn::Ident = input.parse()?;
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

// Let's try to use our own macro (this will fail)
/*
datalang! {
    term SelfTest {
    }
}
*/

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
                    let name_ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
                    let snake_name = name.to_lowercase();
                    let field_name = syn::Ident::new(&snake_name, proc_macro2::Span::call_site());

                    generated_code.push(quote! {
                        #[derive(Debug, Clone)]
                        pub struct #name_ident {
                            pub #field_name: String,
                        }

                        impl #name_ident {
                            pub fn new() -> Self {
                                Self {
                                    #field_name: String::new(),
                                }
                            }
                        }
                    });
                } else {
                    // Composite term - generate struct with referenced fields
                    let name_ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
                    let field_names: Vec<syn::Ident> = fields
                        .iter()
                        .filter(|f| f.is_included)
                        .map(|f| {
                            // Use the namespace if present for future namespacing logic
                            let field_name = if let Some(_namespace) = &f.namespace {
                                // For now, just use the name part, but namespace is available
                                f.name.to_lowercase()
                            } else {
                                f.name.to_lowercase()
                            };
                            syn::Ident::new(&field_name, proc_macro2::Span::call_site())
                        })
                        .collect();

                    generated_code.push(quote! {
                        #[derive(Debug, Clone)]
                        pub struct #name_ident {
                            #(pub #field_names: String,)*
                        }

                        impl #name_ident {
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
                let name_ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
                let included_fields: Vec<String> = fields
                    .iter()
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

                let field_idents: Vec<syn::Ident> = included_fields
                    .iter()
                    .map(|f| syn::Ident::new(f, proc_macro2::Span::call_site()))
                    .collect();

                generated_code.push(quote! {
                    #[derive(Debug, Clone)]
                    pub struct #name_ident {
                        #(pub #field_idents: String,)*
                    }

                    impl #name_ident {
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
