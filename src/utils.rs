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

    clear_buffer(&mut buffer);
    let len = get_windows_directory(&mut buffer);
    let path = std::str::from_utf8(&buffer[..len]).expect("Utf8Error std::str::from_utf8 from get_windows_directory");
    let full_path = format!("{}\\{}", path, dll_name);
    if is_file(&full_path) {
        return Some(full_path);
    }

    // // I don't know if I want to do this. This will get the working directory. I THINK that most
    // // exes execute from C:\Windows, but I am not sure. I guess I can try and mess with this, later.
    // clear_buffer(&mut buffer);
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

fn clear_buffer(buffer: &mut [u8]) {
    for b in buffer.iter_mut() {
        *b = 0;
    }
}

fn is_file(path: &str) -> bool {
    match fs::metadata(path) {
        Ok(f) => f.is_file(),
        Err(_) => false,
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

fn get_current_directory(buffer: &mut [u8]) -> usize {
    unsafe {
        GetCurrentDirectoryA( (buffer.len() - 1) as u32, buffer.as_mut_ptr()) as usize
    }
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

// pub unsafe fn GetModuleHandleInternal(module_name: &str) -> usize {
//     let peb = get_peb();
//
//     if module_name.is_empty() {
//         return peb.ImageBaseAddress;
//     }
//
//     let ldr = peb.Ldr;
//     let module_list = &ldr.InMemoryOrderModuleList;
//
//     let mut list_entry = module_list.Flink;
//     while addr_of!(*list_entry) as usize != addr_of!(*module_list) as usize {
//         let entry = addr_of!(*list_entry) as *const TRUNC_LDR_DATA_TABLE_ENTRY;
//         let name = std::slice::from_raw_parts(
//             (*entry).BaseDllName.Buffer,
//             (*entry).BaseDllName.Length as usize / 2,
//         );
//
//         if compare_str_and_w_str_bytes(module_name.as_bytes(), name, true) {
//             return (*entry).DllBase;
//         }
//         list_entry = list_entry.Flink;
//     }
//
//     0
// }
// pub enum ExportType<'a> {
//     Name(&'a str),
//     Ordinal(u16),
// }
// pub unsafe fn GetProcAddressInternal(base_address: usize, export: ExportType) -> usize {
//     let dos_header = base_address as *const IMAGE_DOS_HEADER;
//     let nt_headers =
//         (base_address + (*dos_header).e_lfanew as usize) as *const IMAGE_NT_HEADERS;
//     let optional_header = &(*nt_headers).OptionalHeader;
//     let export_data_directory =
//         &optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT as usize];
//     let export_directory =
//         (base_address + export_data_directory.VirtualAddress as usize) as *const IMAGE_EXPORT_DIRECTORY;
//
//     let export_address_table_rva = base_address + (*export_directory).AddressOfFunctions as usize;
//     let export_address_table_array = std::slice::from_raw_parts(
//         export_address_table_rva as *const u32,
//         (*export_directory).NumberOfFunctions as usize,
//     );
//
//     let mut proc_address = 0;
//     match export {
//         ExportType::Ordinal(ordinal) => {
//             let ordinal = ordinal as u32;
//             let base = (*export_directory).Base;
//
//             if (ordinal < base) || (ordinal >= base + (*export_directory).NumberOfFunctions) {
//                 return 0;
//             }
//
//             proc_address =
//                 base_address + export_address_table_array[(ordinal - base) as usize] as usize;
//         }
//         ExportType::Name(proc_name) => {
//             let name_table_address = base_address + (*export_directory).AddressOfNames as usize;
//             let name_table = std::slice::from_raw_parts(
//                 name_table_address as *const u32,
//                 (*export_directory).NumberOfNames as usize,
//             );
//
//             for i in 0..(*export_directory).NumberOfNames as usize {
//                 let string_address = base_address + name_table[i] as usize;
//                 let name = std::slice::from_raw_parts(
//                     string_address as *const u8,
//                     strlen(string_address as *const u8),
//                 );
//
//                 if proc_name.as_bytes() == name {
//                     let hints_table_address =
//                         base_address + (*export_directory).AddressOfNameOrdinals as usize;
//                     let hints_table = std::slice::from_raw_parts(
//                         hints_table_address as *const u16,
//                         (*export_directory).NumberOfNames as usize,
//                     );
//
//                     proc_address =
//                         base_address + export_address_table_array[hints_table[i] as usize] as usize;
//                 }
//             }
//         }
//     }
//
//     if proc_address >= export_directory as usize
//         && proc_address < export_directory as usize + export_data_directory.Size as usize
//     {
//         proc_address = get_fwd_addr(proc_address as *const u8);
//     }
//
//     proc_address
// }
//
// unsafe fn get_fwd_addr(proc_address: *const u8) -> usize {
//     let len = strlen(proc_address);
//
//     #[cfg(not(feature = "no_std"))]
//     let mut forward_dll = String::from_utf8(std::slice::from_raw_parts(proc_address, len).to_vec()).expect("Could not read forward string.");
//
//     let split_pos = match forward_dll.find('.') {
//         None => {
//             return 0;
//         }
//         Some(sz) => sz,
//     };
//     forward_dll.as_bytes_mut()[split_pos] = 0;
//
//     let forward_handle = LoadLibraryA(&forward_dll);
//     if forward_handle == 0 {
//         return 0;
//     }
//
//     GetProcAddressInternal(forward_handle, ExportType::Name(&forward_dll[split_pos + 1..len]))
// }
//
// pub unsafe fn strlen(s: *const u8) -> usize {
//     let mut len = 0;
//     while *s.add(len) != 0 && len <= MAX_PATH {
//         len += 1;
//     }
//
//     len
// }
//
// extern "C" {
//     pub fn get_peb() -> &'static PEB;
// }
// #[cfg(all(windows, target_arch = "x86_64"))]
// global_asm!(
//     r"
// .global get_peb
// get_peb:
//     mov rax, gs:0x60
//     ret",
// );
// #[cfg(all(windows, target_arch = "x86"))]
// global_asm!(
//     r"
// .global _get_peb
// _get_peb:
//     mov eax, fs:0x30
//     ret",
// );
// const CASE_BIT: u8 = 0x20;
//
// fn compare_str_and_w_str_bytes(
//     string_bytes: &[u8],
//     w_string_bytes: &[u16],
//     case_insensitive: bool,
// ) -> bool {
//     if string_bytes.len() != w_string_bytes.len() {
//         return false;
//     }
//
//     for i in 0..string_bytes.len() {
//         let mut val = string_bytes[i] as u16;
//         let mut val2 = w_string_bytes[i];
//
//         if case_insensitive {
//             if val >= 0x41 && val <= 0x5A {
//                 val ^= CASE_BIT as u16
//             }
//             if val2 >= 0x41 && val2 <= 0x5A {
//                 val2 ^= CASE_BIT as u16
//             }
//         }
//
//         if val != val2 {
//             return false;
//         }
//     }
//
//     true
// }
