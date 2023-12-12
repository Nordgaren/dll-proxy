include!("../../src/utils.rs");

pub fn find_dll_on_disk(dll_name: &str) -> Option<Vec<u8>> {
    match fs::read(dll_name) {
        Ok(v) => return Some(v),
        Err(_) => {}
    }

    if let Some(path) = get_dll_path_from_search_paths(dll_name) {
        return Some(fs::read(path).expect("Found DLL in System or Windows directory, but could not read it from Disk."));
    }

    None
}