use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr, ExprLit, ItemFn, Lit, Meta, MetaNameValue};

struct RoleArgs {
    role: String,
}

impl Parse for RoleArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let meta = input.parse::<Meta>()?;
        if let Meta::NameValue(MetaNameValue { value, .. }) = meta {
            if let Expr::Lit(ExprLit { lit, .. }) = value {
                if let Lit::Str(lit_str) = lit {
                    return Ok(RoleArgs {
                        role: lit_str.value(),
                    });
                } else {
                    return Err(syn::Error::new_spanned(lit, "Expected a string literal"));
                }
            } else {
                return Err(syn::Error::new_spanned(
                    value,
                    "Expected a literal expression",
                ));
            }
        }
        Err(syn::Error::new_spanned(meta, "Expected a name-value meta"))
    }
}

pub fn require_role_internal(args: TokenStream, input: TokenStream) -> TokenStream {
    let fn_input = parse_macro_input!(input as ItemFn);
    let RoleArgs { role } = parse_macro_input!(args as RoleArgs);

    let stmts = &fn_input.block.stmts;
    let sig = &fn_input.sig;

    let expanded = quote! {
        #sig {
            if current_user.0.account_role != AccountRole::from_str(#role).unwrap() {
                tracing::warn!("User '{}' does not have role '{}'.", current_user.0.id, stringify!(#role));
                return actix_web::HttpResponse::Forbidden().json(ErrorResponse::new(
                    0,
                    format!("User '{}' does not have role '{}'", current_user.0.id, stringify!(#role)),
                ));
            }

            #(#stmts)*
        }
    };

    expanded.into()
}
