#![doc = include_str!("../README.md")]

use dll_proxy_core::proxy_dll_core;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro]
pub fn proxy_dll(input: TokenStream) -> TokenStream {
    proxy_dll_core(input.into()).into()
}