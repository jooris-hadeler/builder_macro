use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TS};
use quote::{quote, quote_spanned};
use syn::Ident;

use crate::builder::{Builder, BuilderField};

impl Builder {
    /// Generates the builder code.
    pub fn generate(self) -> TokenStream {
        let builder_struct = self.generate_struct();
        let builder_constructor = self.generate_new_method();
        let builder_impl = self.generate_impl();

        TokenStream::from(quote! {
            #builder_struct
            #builder_constructor
            #builder_impl
        })
    }

    /// Generates the builder struct.
    fn generate_struct(&self) -> TS {
        let builder_name = Ident::new(&format!("{}Builder", &self.name), Span::call_site());

        let fields = self.fields.iter().map(|field| {
            let name = &field.name;
            let ty = &field.ty;

            quote! {
                #name: Option<#ty>,
            }
        });

        quote! {
            #[allow(missing_docs)]
            pub struct #builder_name {
                #(#fields)*
            }
        }
    }

    /// Generates a builder method for a field.
    fn generate_field_method(&self, field: &BuilderField) -> TS {
        let name = &field.name;
        let ty = &field.ty;

        let method_name = Ident::new(&format!("with_{}", name), Span::call_site());

        // Skip the field if it is marked as such.
        if field.skip {
            // Report an error if the field does not have a default value.
            if field.default.is_none() {
                return quote_spanned! {
                    name.span() => compile_error!("Cannot skip a field without a default value. Use `builder_default` to set a default value.");
                };
            }

            return quote! {};
        }

        quote! {
            #[allow(missing_docs)]
            pub fn #method_name<I: Into<#ty>>(mut self, #name: I) -> Self {
                self.#name = Some(#name.into());
                self
            }
        }
    }

    /// Generates the builder methods.
    fn generate_impl(&self) -> TS {
        let name = Ident::new(&format!("{}Builder", self.name), Span::call_site());

        let methods: Vec<TS> = self
            .fields
            .iter()
            .map(|field| self.generate_field_method(field))
            .collect();

        let build_method = self.generate_build_method();

        quote! {
            impl #name {
                #(#methods)*
                #build_method
            }
        }
    }

    /// Generates the build method.
    fn generate_build_method(&self) -> TS {
        let name = &self.name;

        let fields = self.fields.iter().map(|field| {
            let name = &field.name;

            if field.default.is_some() {
                quote! {
                    #name: self.#name.unwrap(),
                }
            } else {
                quote! {
                    #name: self.#name.ok_or_else(|| format!("{} is not set.", stringify!(#name)))?,
                }
            }
        });

        quote! {
            #[allow(missing_docs)]
            pub fn build(self) -> Result<#name, String> {
                Ok(#name {
                    #(#fields)*
                })
            }
        }
    }

    /// Generates the builder constructor.
    fn generate_new_method(&self) -> TS {
        let name = &self.name;
        let builder_name = Ident::new(&format!("{}Builder", self.name), Span::call_site());

        let fields = self.fields.iter().map(|field| {
            let name = &field.name;
            let default = self.generate_field_default(field);

            quote! {
                #name: #default,
            }
        });

        quote! {
            impl #name {
                #[allow(missing_docs)]
                pub fn builder() -> #builder_name {
                    #builder_name {
                        #(#fields)*
                    }
                }
            }
        }
    }

    /// Generates default values for the builder fields.
    fn generate_field_default(&self, field: &BuilderField) -> TS {
        match &field.default {
            Some(default) => quote! { Some(#default.into()) },
            None => quote! { None },
        }
    }
}
