mod utils;
mod winternals;
pub use dll_proxy_derive::dll_proxy;
use crate::winternals::LoadLibraryA;

pub unsafe fn init_dll() -> bool {
    let path = match crate::utils::get_dll_path_from_search_paths("") {
        Some(p) => p,
        None => return false,
    };

    let dll = LoadLibraryA(&path);
    if dll == 0 {
        return false;
    }



    true
}
