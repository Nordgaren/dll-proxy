#![cfg(test)]

use quote::quote;
use crate::proxy_dll_core;

#[test]
fn not_known_dll() {
    let after = proxy_dll_core(quote!("dinput8.dll"));
    assert_ne!(
        after.to_string(),
        ""
    );
}
#[test]
#[should_panic]
fn known_dll() {
    proxy_dll_core(quote!("kernel32.dll"));
}