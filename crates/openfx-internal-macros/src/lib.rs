mod common;
mod low_macros;

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn low_make_property_enums(tokens: TokenStream) -> TokenStream {
    low_macros::make_property_enums::make_property_enums(tokens)
}

#[proc_macro]
pub fn low_make_property_set_structs(tokens: TokenStream) -> TokenStream {
    low_macros::make_property_set_structs::make_property_set_structs(tokens)
}

#[proc_macro]
pub fn low_make_property_set_structs_v2(tokens: TokenStream) -> TokenStream {
    low_macros::make_property_set_structs_v2::make_property_set_structs(tokens)
}

#[proc_macro]
pub fn low_make_property_enums_from_c(tokens: TokenStream) -> TokenStream {
    low_macros::make_property_enums_from_c::make_property_enums_from_c(tokens)
}

#[proc_macro]
pub fn low_make_action_enum(tokens: TokenStream) -> TokenStream {
    low_macros::make_action_enum::make_action_enum(tokens)
}

#[proc_macro]
pub fn low_make_suite_struct(tokens: TokenStream) -> TokenStream {
    low_macros::make_suite_struct::make_suite_struct(tokens)
}

#[proc_macro]
pub fn low_make_object_struct(tokens: TokenStream) -> TokenStream {
    low_macros::make_object_struct::make_object_struct(tokens)
}

#[proc_macro]
pub fn low_plugin_impl_host_for_fetch_suite(tokens: TokenStream) -> TokenStream {
    low_macros::plugin_impl_host_for_fetch_suite::plugin_impl_host_for_fetch_suite(tokens)
}

#[proc_macro_attribute]
pub fn low_impl_suite(attr: TokenStream, item: TokenStream) -> TokenStream {
    low_macros::impl_suite::impl_suite(attr, item)
}
