use builder::BuilderField;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput};

mod builder;
mod generate;

#[proc_macro_derive(
    Builder,
    attributes(builder_skip, builder_default, builder_use_default)
)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Check if the input is a struct.
    let Data::Struct(DataStruct { fields, .. }) = &input.data else {
        return TokenStream::from(quote! {
            compile_error!("Builder derive only works with structs.");
        });
    };

    let name = &input.ident;

    // Create a new builder struct containing information about the struct being built.
    let mut builder = builder::Builder::new(name.clone());

    // Iterate over the fields of the struct and add them to the builder.
    for field in fields {
        // Check if the field is named and return an error if it is not.
        let Some(field_name) = field.ident.as_ref() else {
            return TokenStream::from(quote! {
                compile_error!("Builder derive only works with named fields.");
            });
        };

        let field_ty = &field.ty;

        let mut default = None;
        let mut use_default = false;
        let mut skip = false;

        // Check for builder_skip and builder_default attributes on the field.
        for attribute in field.attrs.iter() {
            let path = attribute.path();

            if path.is_ident("builder_skip") {
                // Make sure the attribute has no arguments.
                match &attribute.meta {
                    syn::Meta::Path(_) => {
                        skip = true;
                    }
                    _ => {
                        return TokenStream::from(quote! {
                            compile_error!("builder_skip expected no arguments, e.g. `#[builder_skip]`.");
                        });
                    }
                }
            } else if path.is_ident("builder_default") {
                // Make sure the attribute has a single argument.
                match &attribute.meta {
                    syn::Meta::NameValue(named) => {
                        default = Some(named.value.clone());
                    }
                    _ => {
                        return TokenStream::from(quote! {
                            compile_error!("builder_default expected default value, e.g. `#[builder_default = \"42\"]`.");
                        });
                    }
                }
            } else if path.is_ident("builder_use_default") {
                // Make sure the attribute has no arguments.
                match &attribute.meta {
                    syn::Meta::Path(_) => {
                        use_default = true;
                    }
                    _ => {
                        return TokenStream::from(quote! {
                            compile_error!("builder_use_default expected no arguments, e.g. `#[builder_use_default]`.");
                        });
                    }
                }
            }
        }

        // Create a new field and add it to the builder.
        let field = BuilderField::new(
            field_name.clone(),
            field_ty.clone(),
            default,
            use_default,
            skip,
        );
        builder.add_field(field);
    }

    // Generate the builder code.
    builder.generate()
}
