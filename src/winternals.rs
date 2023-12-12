// #[repr(C)]
// pub struct PEB {
//     pub InheritedAddressSpace: u8,
//     pub ReadImageFileExecOptions: u8,
//     pub BeingDebugged: u8,
//     pub BitField: u8,
//     pub Mutant: usize,
//     pub ImageBaseAddress: usize,
//     pub Ldr: &'static PEB_LDR_DATA,
//     pub ProcessParameters: u32,
//     pub SubSystemData: usize,
//     pub ProcessHeap: usize,
//     pub FastPebLock: usize,
//     pub AtlThunkSListPtr: usize,
//     pub IFEOKey: usize,
//     pub CrossProcessFlags: u32,
//     pub KernelCallbackTable: usize,
//     pub SystemReserved: u32,
//     pub AtlThunkSListPtr32: u32,
//     pub ApiSetMap: *const u32,
// }
// #[repr(C)]
// pub struct PEB_LDR_DATA {
//     pub Length: u32,
//     pub Initialized: u8,
//     pub SsHandle: usize,
//     pub InLoadOrderModuleList: LIST_ENTRY,
//     pub InMemoryOrderModuleList: LIST_ENTRY,
//     pub InInitializationOrderModuleList: LIST_ENTRY,
//     pub EntryInProgress: usize,
//     pub ShutdownInProgress: u32,
//     pub ShutdownThreadId: usize,
// }
//
// #[repr(C)]
// pub struct TRUNC_LDR_DATA_TABLE_ENTRY {
//     //pub InLoadOrderLinks: LIST_ENTRY, // removed to start from InMemoryOrderLinks without recalculating offset.
//     pub InMemoryOrderLinks: LIST_ENTRY,
//     pub InInitializationOrderLinks: LIST_ENTRY,
//     pub DllBase: usize,
//     pub EntryPoint: usize,
//     pub SizeOfImage: usize,
//     pub FullDllName: UNICODE_STRING,
//     pub BaseDllName: UNICODE_STRING,
// }
// #[repr(C)]
// pub struct UNICODE_STRING {
//     pub Length: u16,
//     pub MaximumLength: u16,
//     pub Buffer: *mut u16,
// }
// #[repr(C)]
// pub struct LIST_ENTRY {
//     pub Flink: &'static LIST_ENTRY,
//     pub Blink: &'static LIST_ENTRY,
// }
// #[repr(C, packed(2))]
// pub struct IMAGE_DOS_HEADER {
//     pub e_magic: u16,
//     pub e_cblp: u16,
//     pub e_cp: u16,
//     pub e_crlc: u16,
//     pub e_cparhdr: u16,
//     pub e_minalloc: u16,
//     pub e_maxalloc: u16,
//     pub e_ss: u16,
//     pub e_sp: u16,
//     pub e_csum: u16,
//     pub e_ip: u16,
//     pub e_cs: u16,
//     pub e_lfarlc: u16,
//     pub e_ovno: u16,
//     pub e_res: [u16; 4],
//     pub e_oemid: u16,
//     pub e_oeminfo: u16,
//     pub e_res2: [u16; 10],
//     pub e_lfanew: i32,
// }
//
// #[repr(C)]
// pub struct IMAGE_FILE_HEADER {
//     pub Machine: u16,
//     pub NumberOfSections: u16,
//     pub TimeDateStamp: u32,
//     pub PointerToSymbolTable: u32,
//     pub NumberOfSymbols: u32,
//     pub SizeOfOptionalHeader: u16,
//     pub Characteristics: u16,
// }
//
// #[repr(C)]
// pub struct IMAGE_NT_HEADERS {
//     pub Signature: u32,
//     pub FileHeader: IMAGE_FILE_HEADER,
//     pub OptionalHeader: IMAGE_OPTIONAL_HEADER,
// }
//
// #[repr(C)]
// pub struct IMAGE_OPTIONAL_HEADER {
//     pub Magic: u16,
//     pub MajorLinkerVersion: u8,
//     pub MinorLinkerVersion: u8,
//     pub SizeOfCode: u32,
//     pub SizeOfInitializedData: u32,
//     pub SizeOfUninitializedData: u32,
//     pub AddressOfEntryPoint: u32,
//     pub BaseOfCode: u32,
//     #[cfg(target_arch = "x86")]
//     pub BaseOfData: u32,
//     pub ImageBase: usize,
//     pub SectionAlignment: u32,
//     pub FileAlignment: u32,
//     pub MajorOperatingSystemVersion: u16,
//     pub MinorOperatingSystemVersion: u16,
//     pub MajorImageVersion: u16,
//     pub MinorImageVersion: u16,
//     pub MajorSubsystemVersion: u16,
//     pub MinorSubsystemVersion: u16,
//     pub Win32VersionValue: u32,
//     pub SizeOfImage: u32,
//     pub SizeOfHeaders: u32,
//     pub CheckSum: u32,
//     pub Subsystem: u16,
//     pub DllCharacteristics: u16,
//     pub SizeOfStackReserve: usize,
//     pub SizeOfStackCommit: usize,
//     pub SizeOfHeapReserve: usize,
//     pub SizeOfHeapCommit: usize,
//     pub LoaderFlags: u32,
//     pub NumberOfRvaAndSizes: u32,
//     pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
// }
// #[repr(C)]
// pub struct IMAGE_DATA_DIRECTORY {
//     pub VirtualAddress: u32,
//     pub Size: u32,
// }
// #[repr(C)]
// //[derive(Debug)]
// pub struct IMAGE_SECTION_HEADER {
//     pub Name: [u8; 8],
//     pub Misc: IMAGE_SECTION_HEADER_UNION,
//     pub VirtualAddress: u32,
//     pub SizeOfRawData: u32,
//     pub PointerToRawData: u32,
//     pub PointerToRelocations: u32,
//     pub PointerToLinenumbers: u32,
//     pub NumberOfRelocations: u16,
//     pub NumberOfLinenumbers: u16,
//     pub Characteristics: u32,
// }
//
// #[repr(C)]
// pub union IMAGE_SECTION_HEADER_UNION {
//     pub PhysicalAddress: u32,
//     pub VirtualSize: u32,
// }
//
// #[repr(C)]
// pub struct IMAGE_EXPORT_DIRECTORY {
//     pub Characteristics: u32,
//     pub TimeDateStamp: u32,
//     pub MajorVersion: u16,
//     pub MinorVersion: u16,
//     pub Name: u32,
//     pub Base: u32,
//     pub NumberOfFunctions: u32,
//     pub NumberOfNames: u32,
//     pub AddressOfFunctions: u32,
//     // RVA from base of image
//     pub AddressOfNames: u32,
//     // RVA from base of image
//     pub AddressOfNameOrdinals: u32, // RVA from base of image
// }
//
// pub const IMAGE_DIRECTORY_ENTRY_ARCHITECTURE: u16 = 7;
// pub const IMAGE_DIRECTORY_ENTRY_BASERELOC: u16 = 5;
// pub const IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT: u16 = 11;
// pub const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: u16 = 14;
// pub const IMAGE_DIRECTORY_ENTRY_DEBUG: u16 = 6;
// pub const IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT: u16 = 13;
// pub const IMAGE_DIRECTORY_ENTRY_EXCEPTION: u16 = 3;
// pub const IMAGE_DIRECTORY_ENTRY_EXPORT: u16 = 0;
// pub const IMAGE_DIRECTORY_ENTRY_GLOBALPTR: u16 = 8;
// pub const IMAGE_DIRECTORY_ENTRY_IAT: u16 = 12;
// pub const IMAGE_DIRECTORY_ENTRY_IMPORT: u16 = 1;
// pub const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: u16 = 10;
// pub const IMAGE_DIRECTORY_ENTRY_RESOURCE: u16 = 2;
// pub const IMAGE_DIRECTORY_ENTRY_SECURITY: u16 = 4;
// pub const IMAGE_DIRECTORY_ENTRY_TLS: u16 = 9;
#[link(name = "kernel32", kind = "raw-dylib")]
extern "system" {
    pub fn GetModuleHandleA(module_name: *const u8) -> usize;
    pub fn GetProcAddress(module_handle: usize, proc_name: *const u8) -> usize;
    pub fn LoadLibraryA(lpLibFileName: *const u8) -> usize;
    pub fn GetSystemDirectoryA(lpBuffer: *mut u8, uSize: u32) -> u32;
    pub fn GetWindowsDirectoryA(lpBuffer: *mut u8, uSize: u32) -> u32;
}

