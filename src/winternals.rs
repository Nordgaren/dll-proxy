#![allow(non_snake_case)]

#[link(name = "kernel32", kind = "raw-dylib")]
#[allow(unused)]
extern "system" {
    pub fn GetCurrentDirectoryA( uSize: u32, lpBuffer: *mut u8) -> u32;
    pub fn GetLastError() -> u32;
    pub fn GetModuleFileNameA(hModule: usize, lpBuffer: *mut u8, uSize: u32) -> u32;
    pub fn GetModuleHandleA(module_name: *const u8) -> usize;
    pub fn GetProcAddress(module_handle: usize, proc_name: *const u8) -> usize;
    pub fn GetSystemDirectoryA(lpBuffer: *mut u8, uSize: u32) -> u32;
    pub fn GetWindowsDirectoryA(lpBuffer: *mut u8, uSize: u32) -> u32;
    pub fn LoadLibraryA(lpLibFileName: *const u8) -> usize;
}

