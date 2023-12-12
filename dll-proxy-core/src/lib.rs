#![doc = include_str!("../README.md")]

mod tests;
mod utils;
mod winternals;

use crate::utils::{dll_is_known_dll, find_dll_on_disk};
use pe_util::PE;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::PathBuf;
use syn::{parse2, LitStr};

pub fn dll_proxy_core(_: TokenStream, input: TokenStream) -> TokenStream {
    // proc_marco2 version of "parse_macro_input!(input as LitStr)"
    let user_input = match parse2::<LitStr>(input) {
        Ok(syntax_tree) => syntax_tree.value(),
        Err(error) => return error.to_compile_error(),
    };

    if dll_is_known_dll(user_input.as_str()) {
        panic!("Cannot proxy dll in KnownDlls list {}", user_input)
    }

    let pe_file = match find_dll_on_disk(user_input.as_str()) {
        Some(v) => v,
        None => panic!("Could not find {}", user_input),
    };

    let pe = PE::from_slice(pe_file.as_slice())
        .expect(&format!("Could not parse PE headers for {}", user_input));

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
                            "jmpq  *{}(%rip)",
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
            #export_ptr = dll_proxy::winternals::GetProcAddressInternal(#dll_ident, #export);
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
                    Some(p) => p,
                    None => return false,
                };

                let #dll_ident = dll_proxy::winternals::LoadLibraryA(&path);
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
