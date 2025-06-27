// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use proc_macro2::{Span, TokenStream};
use quote::format_ident;

pub(crate) fn expand_structs_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse::<syn::Item>(input);
    match input {
        Ok(syn::Item::Struct(item)) => expand_struct_item(item).into(),
        Ok(_) => {
            return syn::Error::new(Span::call_site(), "`#[epicchain::structs]` can only be applied to `struct`")
                .to_compile_error()
                .into()
        }
        Err(err) => return err.to_compile_error().into(),
    }
}

fn expand_struct_item(item: syn::ItemStruct) -> TokenStream {
    let attrs = &item.attrs;
    let vis = &item.vis;
    let ident = &item.ident;
    let generics = &item.generics;
    let semi_token = &item.semi_token;
    let (impls, types, wheres) = &item.generics.split_for_impl();

    let mut expanded = match &item.fields {
        syn::Fields::Named(_fields) => {
            quote::quote! {
                #[cfg(target_family = "wasm")]
                #(#attrs)*
                #vis struct #ident #generics {
                    placeholder: epicchain_contract::types::placeholder::Placeholder,
                } #semi_token

                #[cfg(target_family = "wasm")]
                impl #impls epicchain_contract::types::placeholder::FromPlaceholder for #ident #types #wheres {
                    #[inline(always)]
                    fn from_placeholder(placeholder: epicchain_contract::types::placeholder::Placeholder) -> Self {
                        Self { placeholder }
                    }
                }

                #[cfg(target_family = "wasm")]
                impl #impls epicchain_contract::types::placeholder::IntoPlaceholder for #ident #types #wheres {
                    #[inline(always)]
                    fn into_placeholder(self) -> epicchain_contract::types::placeholder::Placeholder {
                        self.placeholder
                    }
                }
            }
        }
        syn::Fields::Unnamed(_fields) => {
            quote::quote! {
                #[cfg(target_family = "wasm")]
                #(#attrs)*
                #vis struct #ident #generics (epicchain_contract::types::placeholder::Placeholder) #semi_token

                #[cfg(target_family = "wasm")]
                impl #impls epicchain_contract::types::placeholder::FromPlaceholder for #ident #types #wheres {
                    #[inline(always)]
                    fn from_placeholder(placeholder: epicchain_contract::types::placeholder::Placeholder) -> Self {
                        Self(placeholder)
                    }
                }

                #[cfg(target_family = "wasm")]
                impl #impls epicchain_contract::types::placeholder::IntoPlaceholder for #ident #types #wheres {
                    #[inline(always)]
                    fn into_placeholder(self) -> epicchain_contract::types::placeholder::Placeholder {
                        self.0
                    }
                }
            }
        }
        syn::Fields::Unit => {
            quote::quote! {
                #[cfg(target_family = "wasm")]
                #item
            }
        }
    };

    // impl getters for fields
    let fields = expand_fields(&item.fields);
    if !fields.is_empty() {
        expanded.extend(quote::quote! {
            #[cfg(target_family = "wasm")]
            impl #impls #ident #types #wheres {
                #fields
            }
        });
    }

    expanded.extend(quote::quote! {
        #[cfg(not(target_family = "wasm"))]
        #item
    });

    expanded
}

fn expand_fields(fields: &syn::Fields) -> TokenStream {
    let mut expanded = quote::quote! {};
    match fields {
        syn::Fields::Named(fields) => {
            expanded.extend(expand_named_field(fields));
        }
        syn::Fields::Unnamed(fields) => {
            expanded.extend(expand_unamed_field(fields));
        }
        syn::Fields::Unit => {}
    };

    expanded
}

fn expand_named_field(fields: &syn::FieldsNamed) -> TokenStream {
    let mut expanded = quote::quote! {};
    for (index, field) in fields.named.iter().enumerate() {
        let Some(name) = &field.ident else {
            continue;
        };
        let ty = &field.ty;
        if let Some(get) = find_attr(field, "get") {
            let vis = get_vis(get);
            expanded.extend(quote::quote! {
                #vis fn #name(&self) -> #ty {
                    epicchain_contract::types::structs::internal_struct_get::<#index, #ty>(self.placeholder)
                }
            });
        } else {
            // getter in default
            expanded.extend(quote::quote! {
                // #[cfg(not(target_family = "wasm"))]
                fn #name(&self) -> #ty {
                    epicchain_contract::types::structs::internal_struct_get::<#index, #ty>(self.placeholder)
                }
            });
        }

        let name = &format_ident!("set_{}", name);
        if let Some(set) = find_attr(field, "set") {
            let vis = get_vis(set);
            expanded.extend(quote::quote! {
                #vis fn #name(&mut self, value: #ty) {
                    epicchain_contract::types::structs::internal_struct_set::<#index, #ty>(self.placeholder, value);
                }
            });
        } // no set in default
    }

    expanded
}

fn expand_unamed_field(fields: &syn::FieldsUnnamed) -> TokenStream {
    let mut expanded = quote::quote! {};
    for (index, field) in fields.unnamed.iter().enumerate() {
        let ty = &field.ty;
        let name = &format_ident!("get_{}", index);
        if let Some(get) = find_attr(field, "get") {
            let vis = get_vis(get);
            expanded.extend(quote::quote! {
                #vis fn #name(&self) -> #ty {
                    epicchain_contract::types::structs::internal_struct_get::<#index, #ty>(self.0)
                }
            });
        } else {
            // getter in default
            expanded.extend(quote::quote! {
                // #[cfg(target_family = "wasm")]
                fn #name(&self) -> #ty {
                    epicchain_contract::types::structs::internal_struct_get::<#index, #ty>(self.0)
                }
            });
        }

        let name = &format_ident!("set_{}", index);
        if let Some(set) = find_attr(field, "set") {
            let vis = get_vis(set);
            expanded.extend(quote::quote! {
                #vis fn #name(&mut self, value: #ty) {
                    epicchain_contract::types::structs::internal_struct_set::<#index, #ty>(self.0, value);
                }
            });
        } // no set in default
    }

    expanded
}

fn find_attr<'a>(field: &'a syn::Field, name: &str) -> Option<&'a syn::Attribute> {
    field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident(name) && matches!(attr.style, syn::AttrStyle::Outer))
}

fn get_vis(attr: &syn::Attribute) -> Option<TokenStream> {
    if let syn::Meta::List(vis) = &attr.meta {
        Some(vis.tokens.clone())
    } else {
        None
    }
}
