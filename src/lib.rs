use darling::{ast, util::parse_expr, FromDeriveInput, FromField};
use quote::{quote, quote_spanned, ToTokens};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(builder), supports(struct_named))]
struct BuilderStructReceiver {
    /// The struct ident.
    ident: syn::Ident,
    /// The type's generics. You'll need these any time your trait is expected
    /// to work with types that declare generics.
    generics: syn::Generics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    data: ast::Data<(), BuilderFieldReceiver>,
}

impl ToTokens for BuilderStructReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let BuilderStructReceiver {
            ident,
            generics,
            data,
        } = self;

        let builder_ident = syn::Ident::new(&format!("{}Builder", ident), ident.span());

        let (_imp, ty, wher) = generics.split_for_impl();
        let fields = data.as_ref().take_struct().unwrap().fields;

        // Make sure that if a field is skipped, it has a default value.
        let mut error = false;
        for field in fields.iter() {
            if field.skip && field.default.is_none() {
                let span = field.ident.as_ref().unwrap().span();

                tokens.extend(quote_spanned! {
                    span => compile_error!("Cannot use `#[builder(skip)]` without having `#[builder(default = \"\")]` on the same field."); 
                });
                error = true;
            }
        }

        if error {
            return;
        }

        // Generate the fields of the builder struct.
        let mut builder_fields = proc_macro2::TokenStream::new();
        fields
            .iter()
            .for_each(|BuilderFieldReceiver { ident, ty, .. }| {
                builder_fields.extend(quote! {
                    #ident: Option<#ty>,
                });
            });

        // Generate the builder defaults.
        let mut builder_field_defaults = proc_macro2::TokenStream::new();
        fields
            .iter()
            .for_each(|BuilderFieldReceiver { ident, default, .. }| {
                builder_field_defaults.extend(match default {
                    Some(default) => {
                        quote! {
                            #ident: Some(#default),
                        }
                    }
                    None => {
                        quote! {
                            #ident: None,
                        }
                    }
                });
            });

        // Generate the with methods.
        let mut builder_methods = proc_macro2::TokenStream::new();
        fields.iter().for_each(|BuilderFieldReceiver { ident, ty, skip, .. }| {
            let ident = ident.as_ref().unwrap();
            let with_ident = syn::Ident::new(&format!("with_{}", ident), ident.span());

            if !*skip {
                builder_methods.extend(quote! {
                    #[allow(missing_docs)]
                    pub fn #with_ident<BUILDER_TYPE_INTO: Into<#ty> + Sized>(mut self, #ident: BUILDER_TYPE_INTO) -> Self {
                        self.#ident = Some(#ident.into());
                        self
                    }
                });
            }
        });

        // Generate the build method fields.
        let mut build_method_fields = proc_macro2::TokenStream::new();
        fields
            .iter()
            .for_each(|BuilderFieldReceiver { ident, .. }| {
                build_method_fields.extend(quote! {
                    #ident: self.#ident.ok_or(format!("Field '{}' is required.", stringify!(#ident)))?,
                });
            });

        tokens.extend(quote! {
            #[allow(missing_docs)]
            pub struct #builder_ident #ty #wher {
                #builder_fields
            }

            impl #ty Default for #builder_ident #ty #wher {
                fn default() -> Self {
                    Self {
                        #builder_field_defaults
                    }
                }
            }

            impl #ty #builder_ident #ty #wher {
                #[allow(missing_docs)]
                pub fn build(self) -> Result<#ident #ty, String> {
                    Ok(#ident {
                        #build_method_fields
                    })
                }
            }

            impl #ty #ident #ty #wher {
                /// This method will return a new builder instance.
                pub fn builder() -> #builder_ident #ty {
                    #builder_ident::default()
                }
            }

            impl #ty #builder_ident #ty #wher {
                #builder_methods
            }
        });
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(builder))]
struct BuilderFieldReceiver {
    /// The ident of the field.
    ident: Option<syn::Ident>,
    /// The type of the field.
    ty: syn::Type,
    #[darling(default)]
    /// Whether the field should be skipped.
    skip: bool,
    #[darling(with = parse_expr::preserve_str_literal, map = Some)]
    /// The default value of the field.
    default: Option<syn::Expr>,
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let receiver = BuilderStructReceiver::from_derive_input(&input).unwrap();
    receiver.to_token_stream().into()
}
