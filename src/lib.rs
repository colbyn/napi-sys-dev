#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_export]
macro_rules! register_module {
    ($module_name:ident, $init:ident) => {
        #[no_mangle]
        #[cfg_attr(target_os = "linux", link_section = ".ctors")]
        #[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
        #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
        pub static __REGISTER_MODULE: extern "C" fn() = {
            use std::io::Write;
            use std::os::raw::c_char;
            use std::ptr;
            use napi_sys_dev::*;
            // use $crate::*;

            extern "C" fn register_module() {
                static mut MODULE_DESCRIPTOR: Option<napi_module> = None;
                unsafe {
                    MODULE_DESCRIPTOR = Some(napi_module {
                        nm_version: 1,
                        nm_flags: 0,
                        nm_filename: concat!(file!(), "\0").as_ptr() as *const c_char,
                        nm_register_func: Some($init),
                        nm_modname: concat!(stringify!($module_name), "\0").as_ptr() as *const c_char,
                        nm_priv: 0 as *mut _,
                        reserved: [0 as *mut _; 4],
                    });
                    napi_module_register(MODULE_DESCRIPTOR.as_mut().unwrap() as *mut napi_module);
                }
            }

            register_module
        };
    };
}