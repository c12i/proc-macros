use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(Builder)]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree as a DeriveInput
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name and fields
    let struct_name = input.ident;
    let fields = if let Data::Struct(syn::DataStruct { fields, .. }) = input.data {
        fields
    } else {
        panic!("Builder macro only supports structs.");
    };

    // Generate the builder struct and its methods
    let builder_struct = generate_builder_struct(&struct_name, &fields);
    let builder_name = format!("{}Builder", struct_name);
    let builder_name = syn::Ident::new(&builder_name, struct_name.span());

    // Generate the builder struct and the annotated struct name builder method
    let expanded = quote! {
        #builder_struct

        impl #struct_name {
            pub fn builder() -> #builder_name {
                #builder_name::default()
            }
        }
    };

    TokenStream::from(expanded)
}

// Helper function to generate the builder struct
fn generate_builder_struct(struct_name: &Ident, fields: &Fields) -> proc_macro2::TokenStream {
    let builder_name = format!("{}Builder", struct_name);
    let builder_name = syn::Ident::new(&builder_name, struct_name.span());

    let field_assignments = match fields {
        Fields::Named(named_fields) => {
            let field_names = named_fields.named.iter().map(|field| &field.ident);
            let field_types = named_fields.named.iter().map(|field| &field.ty);

            quote! {
                #(pub #field_names: Option<#field_types>),*
            }
        }
        _ => panic!("Builder macro only supports named fields in the struct."),
    };

    let field_setters = match fields {
        syn::Fields::Named(named_fields) => {
            let builder_field_names = named_fields.named.iter().map(|field| &field.ident);
            let builder_field_types = named_fields.named.iter().map(|field| &field.ty);

            quote! {
                #(pub fn #builder_field_names(&mut self, value: #builder_field_types) -> &mut Self {
                    self.#builder_field_names = Some(value);
                    self
                })*
            }
        }
        _ => quote! {},
    };

    let build_method = match fields {
        syn::Fields::Named(named_fields) => {
            let builder_field_names = named_fields.named.iter().map(|field| &field.ident);

            quote! {
                pub fn build(&mut self) -> Result<#struct_name, Box<dyn std::error::Error>> {
                    Ok(#struct_name {
                        #(
                            #builder_field_names: self.#builder_field_names.take().ok_or("Missing field")?,
                        )*
                    })
                }
            }
        }
        _ => quote! {},
    };

    quote! {
        #[derive(Default)]
        pub struct #builder_name {
            #field_assignments
        }

        impl #builder_name {
            #field_setters

            #build_method
        }
    }
}
