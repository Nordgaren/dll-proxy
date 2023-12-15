#![doc = include_str!("../README.md")]

mod tests;
mod utils;

use crate::utils::{dll_is_known_dll, LoadLibraryA};
use pe_util::PE;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::PathBuf;
use syn::{parse2, LitStr};

pub fn proxy_dll_core(input: TokenStream) -> TokenStream {
    // proc_marco2 version of "parse_macro_input!(input as LitStr)"
    let user_input = match parse2::<LitStr>(input) {
        Ok(syntax_tree) => syntax_tree.value(),
        Err(error) => return error.to_compile_error(),
    };

    if dll_is_known_dll(&user_input) {
        panic!("dll is known dll, and cannot be proxied. Please use a different dll.")
    }

    let mut path = user_input.clone();
    path.push('\0');

    let pe_addr = unsafe { LoadLibraryA(path.as_ptr()) };

    if pe_addr == 0 {
        panic!("Could not find PE file");
    }

    let pe = unsafe {
        PE::from_address(pe_addr).expect(&format!("Could not parse PE headers for {}", user_input))
    };

    let exports = unsafe { pe.get_exports() }.expect("Could not get exports");

    let mut token_stream = TokenStream::new();

    for export in &exports {
        let export_ptr = format_ident!("p{}", export);
        let export_name = format_ident!("{}", export);

        let q = quote! {
                    static mut #export_ptr: usize = 0;

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

    // Get the values we have our thunks jump to.
    let mut init_funcs = TokenStream::new();
    for export in &exports {
        let export_ptr = format_ident!("p{}", export);
        let export_terminated = format!("{}\0", export);
        let q = quote! {
            #export_ptr = dll_proxy::winternals::GetProcAddress(dll_addr, #export_terminated.as_ptr());
        };
        init_funcs.extend(q);
    }
    // If the path is absolute, we want to remove the file name.
    // I am just going to assume if the user gives an absolute path, then it will be in the targets search paths.
    let dll_name = if path.is_absolute() {
        // why
        path.file_name()
            .expect(&format!("No file name for user input {:?}", path))
            .to_str()
            .expect("Could not convert filename to str")
    } else {
        path.to_str().expect("Could not convert filename to str")
    };

    // Just in case the user passes in a relative path.
    // I should maybe also make relative paths assume the dll is in the search paths
    // but it could have legitimate use case? IDK
    let dll_name_lower = path
        .file_name()
        .expect(&format!("No file name for user input {:?}", path))
        .to_str()
        .expect("Could not convert filename to str")
        .to_lowercase();

    let func = quote! {
        pub unsafe fn init_proxy(hModule: usize) -> Result<String, String> {
                let name = dll_proxy::utils::get_path(hModule);
                if !name.to_lowercase().ends_with(#dll_name_lower) {
                    return Ok(name);
                }
                let path = match dll_proxy::utils::get_dll_path_from_search_paths(#dll_name) {
                    Some(mut p) => {
                        p.push('\0');
                        p
                    },
                    None => return Err(format!("Could not find dll from search paths. {}", #dll_name)),
                };

                let dll_addr = dll_proxy::winternals::LoadLibraryA(path.as_ptr());
                if dll_addr == 0 {
                    return Err("LoadLibraryA failed, last error: 0x{:X}", dll_proxy::winternals::GetLastError());
                }

                #init_funcs

                Ok(name)
        }
    };

    token_stream.extend(func);

    token_stream
}
