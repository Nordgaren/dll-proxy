#![cfg(test)]

use crate::dll_proxy_core;
use quote::quote;

#[test]
fn not_known_dll() {
    let after = dll_proxy_core(quote!(), quote!("dinput8.dll"));
    assert_ne!(
        after.to_string(),
        ""
    );
}
#[test]
#[should_panic]
fn known_dll() {
    dll_proxy_core(quote!(), quote!("kernel32.dll"));
}