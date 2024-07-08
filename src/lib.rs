use darling::{ast, util::parse_expr, FromDeriveInput, FromField};
use quote::{quote, ToTokens};

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

        // Generate the fields of the builder struct.
        let builder_fields: Vec<proc_macro2::TokenStream> = fields
            .iter()
            .map(|field| {
                let BuilderFieldReceiver {
                    ident,
                    ty,
                    skip,
                    default,
                } = field;

                if *skip && default.is_none() {
                    panic!("Cannot use `#[builder(skip)]` without having `#[builder(default = \"\")]` on the same field.");
                }
                
                quote! {
                    #ident: Option<#ty>,
                }
            })
            .collect();

        // Generate the builder struct.
        tokens.extend(quote! {
            #[allow(missing_docs)]
            pub struct #builder_ident #ty #wher {
                #(
                    #builder_fields
                )*
            }
        });

        // Generate the builder defaults.
        let builder_field_defaults: Vec<proc_macro2::TokenStream> = fields.iter().map(|field| {
            let BuilderFieldReceiver {
                ident,
                skip,
                default,
                ..
            } = field;

            if *skip {
                let default = default.as_ref().unwrap();

                quote! {
                    #ident: Some(#default),
                }
            } else {
                match default {
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
                }
            }
        }).collect();

        // Generate the builder default implementation.
        tokens.extend(quote! {
            impl Default for #builder_ident #ty #wher {
                fn default() -> Self {
                    Self {
                        #(
                            #builder_field_defaults
                        )*
                    }
                }
            }
        });

        // Generate the with methods.
        let builder_methods: Vec<proc_macro2::TokenStream> = fields.iter().map(|field| {
            let BuilderFieldReceiver {
                ident,
                ty,
                skip,
                ..
            } = field;

            let ident = ident.as_ref().unwrap();
            let with_ident = syn::Ident::new(&format!("with_{}", ident), ident.span());

            if *skip {
                quote! {}
            } else {
                quote! {
                    #[allow(missing_docs)]
                    pub fn #with_ident<INT: Into<#ty> + Sized>(mut self, #ident: INT) -> Self {
                        self.#ident = Some(#ident.into());
                        self
                    }
                }
            }
        }).collect();

        // Generate the builder implementation.
        tokens.extend(quote! {
            impl #builder_ident #ty #wher {
                #(
                    #builder_methods
                )*
            }
        });

        // Generate the build method fields.
        let build_method_fields: Vec<proc_macro2::TokenStream> = fields.iter().map(|field| {
            let BuilderFieldReceiver {
                ident,
                skip,
                ..
            } = field;

            if *skip {
                quote! {
                    #ident: self.#ident.unwrap(),
                }
            } else {
                quote! {
                    #ident: self.#ident.ok_or(format!("Field '{}' is required.", stringify!(#ident)))?,
                }
            }
        }).collect();

        // Generate the build method.
        tokens.extend(quote! {
            impl #builder_ident #ty #wher {
                #[allow(missing_docs)]
                pub fn build(self) -> Result<#ident #ty, String> {
                    Ok(#ident {
                        #(
                            #build_method_fields
                        )*
                    })
                }
            }
        });

        // Generate the builder function.
        tokens.extend(quote! {
            impl #ident #ty #wher {
                /// This method will return a new builder instance.
                pub fn builder() -> #builder_ident #ty #wher {
                    #builder_ident::default()
                }
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