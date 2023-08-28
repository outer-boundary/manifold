mod auth;

use auth::roles::require_role_internal;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn require_role(args: TokenStream, input: TokenStream) -> TokenStream {
    require_role_internal(args, input)
}
