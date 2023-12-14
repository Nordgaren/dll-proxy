use std::fs;
use crate::winternals::*;

pub const MAX_PATH: usize = 260;
#[allow(unused)]
pub fn get_dll_path_from_search_paths(dll_name: &str) -> Option<String> {
    let mut buffer = [0; MAX_PATH + 1];

    let len = get_system_directory(&mut buffer);
    let path = std::str::from_utf8(&buffer[..len]).expect("Utf8Error std::str::from_utf8 from get_system_directory");
    let full_path = format!("{}\\{}", path, dll_name);
    if is_file(&full_path) {
        return Some(full_path);
    }

    buffer.fill(0);
    let len = get_windows_directory(&mut buffer);
    let path = std::str::from_utf8(&buffer[..len]).expect("Utf8Error std::str::from_utf8 from get_windows_directory");
    let full_path = format!("{}\\{}", path, dll_name);
    if is_file(&full_path) {
        return Some(full_path);
    }

    // // I don't know if I want to do this. This will get the working directory. I THINK that most
    // // exes execute from C:\Windows, but I am not sure. I guess I can try and mess with this, later.
    // buffer.fill(0);
    // let len = get_current_directory(&mut buffer);
    // let path = std::str::from_utf8(&buffer[..len]).expect("Utf8Error std::str::from_utf8 from get_windows_directory");
    // println!("get_current_directory: {}", path);
    // let full_path = format!("{}\\{}", path, dll_name);
    // if is_file(&full_path) {
    //     return Some(full_path);
    // }

    let path_env = std::env::var("PATH").expect("Could not get PATH environment variable");
    let paths = path_env.split(";");
    for path in paths {
        if path.is_empty() {
            continue;
        }
        let dll_path = format!("{}\\{}", path, dll_name);
        if is_file(&dll_path) {
            return Some(dll_path);
        }
    }

    None
}
fn is_file(path: &str) -> bool {
    match fs::metadata(path) {
        Ok(f) => f.is_file(),
        Err(_) => false,
    }
}
#[allow(unused)]
fn get_current_directory(buffer: &mut [u8]) -> usize {
    unsafe {
        GetCurrentDirectoryA( (buffer.len() - 1) as u32, buffer.as_mut_ptr()) as usize
    }
}
fn get_system_directory(buffer: &mut [u8]) -> usize {
    unsafe {
        GetSystemDirectoryA(buffer.as_mut_ptr(), (buffer.len() - 1) as u32) as usize
    }
}

fn get_windows_directory(buffer: &mut [u8]) -> usize {
    unsafe {
        GetWindowsDirectoryA(buffer.as_mut_ptr(), (buffer.len() - 1) as u32) as usize
    }
}
pub unsafe fn get_path(module_address: usize) -> String {
    let mut buffer = [0u8; MAX_PATH + 1];
    let name_size = GetModuleFileNameA(
        module_address,
        buffer.as_mut_ptr(),
        buffer.len() as u32
    ) as usize;
    let name = &buffer[..name_size];
    std::str::from_utf8(name).unwrap_or_default().to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::{get_current_directory, MAX_PATH};

    #[test]
    fn current_dir() {
        let mut buffer = [0; MAX_PATH + 1];

        unsafe {
            let len = get_current_directory(&mut buffer);
            println!("{}", len);
            let path = std::str::from_utf8_unchecked(&buffer[..len]);
            println!("{}", path);
        }
    }
}
