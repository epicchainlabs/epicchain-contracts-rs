// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use proc_macro2::{Span, TokenStream};

const NEP11_TOKEN: &str = "Nep11Token";
const NEP17_TOKEN: &str = "Nep17Token";

pub(crate) fn expand_contract_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut src_impl: TokenStream = input.clone().into();
    let input = syn::parse::<syn::Item>(input);
    match input {
        Ok(syn::Item::Impl(item)) => src_impl.extend(expand_impl_item(&item)),
        Ok(_) => {
            return syn::Error::new(Span::call_site(), "`#[contract]` can only be applied to `impl` block")
                .to_compile_error()
                .into()
        }
        Err(err) => return err.to_compile_error().into(),
    }

    src_impl.into()
}

fn expand_impl_item(item: &syn::ItemImpl) -> TokenStream {
    let self_type = item.self_ty.as_ref();
    let mut methods: TokenStream = item
        .items
        .iter()
        .filter_map(|x| match x {
            syn::ImplItem::Fn(method) => Some(method),
            _ => None,
        })
        .filter(|method| {
            // Only process methods marked with #[method] attribute
            has_method_attribute(method)
        })
        .map(|method| {
            let name = &method.sig.ident;
            let returns = &method.sig.output;

            // Check if the method has a #[safe] attribute
            let is_safe = has_safe_attribute(method);

            // If the method is marked as safe, add a comment that can be parsed by the WASM to NEF converter
            let safe_comment = if is_safe {
                quote::quote! { /* @safe */ }
            } else {
                quote::quote! {}
            };

            // Extract parameters excluding &self
            let params: Vec<_> = method.sig.inputs
                .iter()
                .filter_map(|arg| match arg {
                    syn::FnArg::Typed(pat_type) => Some(pat_type),
                    syn::FnArg::Receiver(_) => None, // Skip &self
                })
                .collect();

            // Create parameter list for function signature
            let param_list = params.iter().map(|p| {
                let pat = &p.pat;
                let ty = &p.ty;
                quote::quote! { #pat: #ty }
            });

            // Create argument list for method call (just parameter names)
            let arg_list = params.iter().map(|p| &p.pat);

            quote::quote! {
                #[no_mangle]
                #safe_comment
                pub fn #name(#(#param_list),*) #returns {
                    let contract = #self_type::init();
                    contract.#name(#(#arg_list),*)
                }
            }
        })
        .collect();

    if let Some((None, path, _for)) = &item.trait_ {
        // check path is Nep17Token or not
        if path.segments.last().map(|x| x.ident == NEP17_TOKEN).unwrap_or(false) {
            methods.extend(expand_nep17_methods(item));
        }

        if path.segments.last().map(|x| x.ident == NEP11_TOKEN).unwrap_or(false) {
            methods.extend(expand_nep11_methods(item));
        }
    }

    methods
}

// Check if the method has a #[method] attribute
fn has_method_attribute(method: &syn::ImplItemFn) -> bool {
    method.attrs.iter().any(|attr| {
        if let Some(ident) = attr.path().get_ident() {
            ident.to_string() == "method"
        } else {
            false
        }
    })
}

// Check if the method has a #[safe] attribute
fn has_safe_attribute(method: &syn::ImplItemFn) -> bool {
    method.attrs.iter().any(|attr| {
        if let Some(ident) = attr.path().get_ident() {
            ident.to_string() == "safe"
        } else {
            false
        }
    })
}

fn expand_nep17_methods(item: &syn::ItemImpl) -> TokenStream {
    let self_type = item.self_ty.as_ref();
    let mut methods: TokenStream = quote::quote! {};

    // `_initialize` has default implementation
    if !has_method(item, "_initialize") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn _initialize() {
                #self_type::_initialize()
            }
        });
    }

    // total_supply has default implementation
    if !has_method(item, "total_supply") {
        methods.extend(quote::quote! {
            #[no_mangle]
            /* @safe */
            pub fn total_supply() -> epicchain_contract::types::Int256 {
                #self_type::total_supply()
            }
        });
    }

    // `balance_of` has default implementation
    if !has_method(item, "balance_of") {
        methods.extend(quote::quote! {
            #[no_mangle]
            /* @safe */
            pub fn balance_of(owner: epicchain_contract::types::H160) -> epicchain_contract::types::Int256 {
                #self_type::balance_of(owner)
            }
        });
    }

    // `transfer` has default implementation
    if !has_method(item, "transfer") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn transfer(
                from: epicchain_contract::types::H160,
                to: epicchain_contract::types::H160,
                amount: epicchain_contract::types::Int256,
                data: epicchain_contract::types::Array<epicchain_contract::types::Any>,
            ) -> bool {
                #self_type::transfer(from, to, amount, data)
            }
        });
    }

    methods
}

fn expand_nep11_methods(item: &syn::ItemImpl) -> TokenStream {
    let self_type = item.self_ty.as_ref();
    let mut methods: TokenStream = quote::quote! {};

    // symbol method
    if !has_method(item, "symbol") {
        methods.extend(quote::quote! {
            #[no_mangle]
            /* @safe */
            pub fn symbol() -> epicchain_contract::types::ByteString {
                #self_type::symbol()
            }
        });
    }

    // decimals method
    if !has_method(item, "decimals") {
        methods.extend(quote::quote! {
            #[no_mangle]
            /* @safe */
            pub fn decimals() -> u8 {
                #self_type::decimals()
            }
        });
    }

    // totalSupply method
    if !has_method(item, "totalSupply") {
        methods.extend(quote::quote! {
            #[no_mangle]
            /* @safe */
            pub fn totalSupply() -> epicchain_contract::types::Int256 {
                #self_type::total_supply()
            }
        });
    }

    // balanceOf method
    if !has_method(item, "balanceOf") {
        methods.extend(quote::quote! {
            #[no_mangle]
            /* @safe */
            pub fn balanceOf(owner: epicchain_contract::types::H160) -> epicchain_contract::types::Int256 {
                #self_type::balance_of(owner)
            }
        });
    }

    // tokensOf method
    if !has_method(item, "tokensOf") {
        methods.extend(quote::quote! {
            #[no_mangle]
            /* @safe */
            pub fn tokensOf(owner: epicchain_contract::types::H160) -> epicchain_contract::types::Array<epicchain_contract::types::ByteString> {
                #self_type::tokens_of(owner)
            }
        });
    }

    // ownerOf method
    if !has_method(item, "ownerOf") {
        methods.extend(quote::quote! {
            #[no_mangle]
            /* @safe */
            pub fn ownerOf(tokenId: epicchain_contract::types::ByteString) -> epicchain_contract::types::H160 {
                #self_type::owner_of(tokenId)
            }
        });
    }

    // transfer method
    if !has_method(item, "transfer") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn transfer(to: epicchain_contract::types::H160, tokenId: epicchain_contract::types::ByteString, data: epicchain_contract::types::Any) -> bool {
                #self_type::transfer(to, tokenId, data)
            }
        });
    }

    methods
}

fn has_method(item: &syn::ItemImpl, name: &str) -> bool {
    item.items.iter().any(|item| match item {
        syn::ImplItem::Fn(method) => method.sig.ident == name,
        _ => false,
    })
}
