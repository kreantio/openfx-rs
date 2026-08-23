mod sys_helpers_macros;

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn sys_helpers_make_property_accessors(tokens: TokenStream) -> TokenStream {
    sys_helpers_macros::make_property_accessors::make_property_accessors(tokens)
}
