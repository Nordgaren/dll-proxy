use std::path::PathBuf;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

#[link(name = "kernel32", kind = "raw-dylib")]
extern "system" {
    #[allow(non_snake_case)]
    pub fn LoadLibraryA(lpLibFileName: *const u8) -> usize;
}

pub fn dll_is_known_dll(dll_name: &str) -> bool {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let known_dlls = hklm
        .open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\KnownDLLs")
        .expect("Could not find KnownDLLs subkey");
    let lowercase = dll_name.to_lowercase();
    let path = PathBuf::from(lowercase);
    // I hate this
    let lowercase_dll_name = path.file_name()
        .expect("Could not get file name from supplied dll")
        .to_str()
        .expect("Utf8Error when trying to parse dll name to string");
    for dll in known_dlls.enum_values() {
        match dll {
            Ok(name) => {
                if name.1.to_string().to_lowercase() == lowercase_dll_name {
                    return true;
                }
            }
            Err(_) => continue,
        }
    }

    false
}
