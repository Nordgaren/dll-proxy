#![doc = include_str!("../README.md")]

use dll_proxy_core::dll_proxy_core;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn dll_proxy(args: TokenStream, input: TokenStream) -> TokenStream {
    dll_proxy_core(args.into(), input.into()).into()
}