#![doc = include_str!("../README.md")]

mod tests;
mod utils;

use crate::utils::{dll_is_known_dll, LoadLibraryA};
use pe_util::PE;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::PathBuf;
use syn::{parse2, LitStr};

pub fn dll_proxy_core(_: TokenStream, input: TokenStream) -> TokenStream {
    // proc_marco2 version of "parse_macro_input!(input as LitStr)"
    let mut user_input = match parse2::<LitStr>(input) {
        Ok(syntax_tree) => syntax_tree.value(),
        Err(error) => return error.to_compile_error(),
    };

    if dll_is_known_dll(&user_input) {
        panic!("dll is known dll, and cannot be proxied. Please use a different dll.")
    }

    user_input.push('\0');

    let pe_addr = unsafe { LoadLibraryA(user_input.as_ptr()) };

    if pe_addr == 0 {
        panic!("Could not find PE file");
    }

    let pe = unsafe {
        PE::from_address(pe_addr)
            .expect(&format!("Could not parse PE headers for {}", user_input))
    };

    let exports = unsafe { pe.get_exports() }.expect("Could not get exports");

    let mut token_stream = TokenStream::new();

    for export in &exports {
        let export_ptr = format_ident!("p{}", export);
        let export_name = format_ident!("{}", export);

        let q = quote! {
                    pub static mut #export_ptr: *const std::ffi::c_void = 0 as *const std::ffi::c_void;

                    #[no_mangle]
                    pub extern "system" fn #export_name() {
                        unsafe {
                            std::arch::asm!(
                            "jmp [{}]",
                            sym #export_ptr,
                            options(noreturn, att_syntax, nostack),
                            );
                        }
                    }
        };
        token_stream.extend(q);
    }

    let path = PathBuf::from(user_input);
    // why
    let dll_name_no_ext = path
        .file_stem()
        .expect(&format!(
            "No file name without extension for user input {:?}",
            path
        ))
        .to_str()
        .expect("Could not format dll name as str");
    let dll_ident = format_ident!("{dll_name_no_ext}");

    // Get the values we have our thunks jump to.
    let mut init_funcs = TokenStream::new();
    for export in &exports {
        let export_ptr = format_ident!("p{}", export);
        let q = quote! {
            #export_ptr = dll_proxy::winternals::GetProcAddress(#dll_ident, #export);
        };
        init_funcs.extend(q);
    }
    let dll_name = if path.is_absolute() {
        // also why
        path.file_name()
            .expect(&format!("No file name for user input {:?}", path))
            .to_str()
            .expect("Could not convert filename to str")
    } else {
        path.to_str().expect("Could not convert filename to str")
    };

    let func_name = format_ident!("init_{dll_name_no_ext}");

    let func = quote! {
        pub unsafe fn #func_name() {
                let path = match dll_proxy::utils::get_dll_path_from_search_paths(#dll_name) {
                    Some(p) => {
                        p.push('\0');
                        p
                    },
                    None => return false,
                };

                let #dll_ident = dll_proxy::winternals::LoadLibraryA(path.as_ptr());
                if #dll_ident == 0 {
                    return false;
                }

                #init_funcs

                true
        }
    };

    token_stream.extend(func);

    token_stream
}
