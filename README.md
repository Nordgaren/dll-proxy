# dll-proxy

```rust
proxy_dll!("dinput8.dll")
```

This will generate and re-export for every function that is exported in the dll you passed in. It will also generate a public function you can call (if you need to call the proxies functions), and a private pointer as a usize that you can make public yourself with a get method in the file you run the macro in. Then it writes an init function to find the real dlls, and patches them into the re-exported thunks (which are just functions that just jump out of the thunk, like this: `jmp [pExportFuncAddr]`  

You will need to call the init function, and you will also need to pass in the base address of your dll when it's loaded, to the init function. I suggest a DllMain like this:  
```rust
#[no_mangle]
#[allow(unused)]
pub extern "stdcall" fn DllMain(hinstDLL: usize, dwReason: u32, lpReserved: *mut usize) -> i32 {
    match dwReason {
        DLL_PROCESS_ATTACH => unsafe {
            let path = match init_proxy(hinstDLL) {
                Ok(p) => p,
                Err(e) => panic!("Could not proxy dll: {e}"),
            };
            1
        },
        DLL_PROCESS_DETACH => {
            1
        }
        _ => 0,
    }
}
```
The init_proxy function passes back a Result<String, String>, The resulting string is either the path to your dll on the system, or an error message with more information about what went wrong.  

You can also pass in an absolute or relative path. Relative paths will stay in tact, while absolute paths are expected to be found in the target machines System Folder, Windows Folder or PATH folder.  

Here is an example that I use as a base template. Previously it was using a very rudementary and semi-manual approach.
[Example Project](https://github.com/Nordgaren/dinput8-wrapper/blob/8c42ecb06c3d4fa71c3de4515bcc8bca9ab4eaee/src/lib.rs)

