use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Token, braced, Attribute, Lit, BinOp};
use syn::parse::{Parse, ParseStream, Result};

#[derive(Debug, Clone)]
struct FieldAttribute {
    name: String,
    constraint: Option<String>,
}

#[derive(Debug, Clone)]
struct DataLangField {
    attributes: Vec<FieldAttribute>,
    name: Ident,
}

struct DataLangStruct {
    name: Ident,
    parent_trait: Option<Ident>,
    fields: Vec<DataLangField>,
}

impl Parse for DataLangStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        
        let content;
        braced!(content in input);
        
        let mut parent_trait = None;
        let mut fields = Vec::new();
        
        while !content.is_empty() {
            // Check for trait inheritance
            if content.peek(Token![trait]) {
                content.parse::<Token![trait]>()?;
                parent_trait = Some(content.parse::<Ident>()?);
                continue;
            }
            
            // Parse field attributes
            let mut attributes = Vec::new();
            while content.peek(Token![#]) {
                content.parse::<Token![#]>()?;
                let bracketed;
                syn::bracketed!(bracketed in content);
                
                let attr_name: Ident = bracketed.parse()?;
                let mut constraint = None;
                
                // Check for constraint (e.g., < 10)
                if bracketed.peek(Token![<]) {
                    bracketed.parse::<Token![<]>()?;
                    let value: Lit = bracketed.parse()?;
                    constraint = Some(format!("< {}", quote!(#value)));
                }
                
                attributes.push(FieldAttribute {
                    name: attr_name.to_string(),
                    constraint,
                });
            }
            
            // Parse field name
            if content.peek(Ident) {
                let field_name: Ident = content.parse()?;
                fields.push(DataLangField {
                    attributes,
                    name: field_name,
                });
            }
            
            // Optional comma or newline handling
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }
        
        Ok(DataLangStruct { name, parent_trait, fields })
    }
}

#[proc_macro]
pub fn datalang(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DataLangStruct);
    
    let struct_name = &parsed.name;
    let fields = &parsed.fields;
    let parent_trait = &parsed.parent_trait;
    
    // Extract field names for struct definition
    let field_names: Vec<&Ident> = fields.iter().map(|f| &f.name).collect();
    
    // Generate validation methods for fields with attributes
    let validation_methods: Vec<_> = fields.iter().filter_map(|field| {
        if field.attributes.is_empty() {
            return None;
        }
        
        let field_name = &field.name;
        let method_name = syn::Ident::new(&format!("validate_{}", field_name), field_name.span());
        
        let validations: Vec<_> = field.attributes.iter().map(|attr| {
            match (attr.name.as_str(), &attr.constraint) {
                ("length", Some(constraint)) => {
                    if constraint.starts_with("< ") {
                        let limit = constraint.trim_start_matches("< ").parse::<usize>().unwrap_or(0);
                        quote! {
                            if self.#field_name.len() >= #limit {
                                return Err(format!("Field '{}' length must be < {}", stringify!(#field_name), #limit));
                            }
                        }
                    } else {
                        quote! {}
                    }
                }
                _ => quote! {}
            }
        }).collect();
        
        Some(quote! {
            pub fn #method_name(&self) -> Result<(), String> {
                #(#validations)*
                Ok(())
            }
        })
    }).collect();
    
    // Generate trait implementation if parent trait exists
    let trait_impl = if let Some(parent) = parent_trait {
        // Inherit fields from parent trait (assuming it's another DataLang struct)
        quote! {
            // Note: In a real implementation, you'd need to track parent struct fields
            // For now, we'll just add a marker
            impl #struct_name {
                pub fn as_parent(&self) -> &dyn std::fmt::Debug {
                    self
                }
            }
        }
    } else {
        quote! {}
    };
    
    // Generate Rust struct with String type for all fields
    let expanded = quote! {
        #[derive(Debug, Clone)]
        pub struct #struct_name {
            #(pub #field_names: String,)*
        }
        
        impl #struct_name {
            pub fn new() -> Self {
                Self {
                    #(#field_names: String::new(),)*
                }
            }
            
            pub fn validate(&self) -> Result<(), String> {
                Ok(())
            }
            
            #(#validation_methods)*
        }
        
        #trait_impl
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
                todo!("Implement DataLang parsing")
            }
        }
    };
    
    TokenStream::from(expanded)
}
