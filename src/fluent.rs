#![allow(non_camel_case_types)]

use libc::{c_char, c_int, c_void};

pub const FLB_ERROR: u32 = 0;
pub const FLB_OK: u32 = 1;
pub const FLB_RETRY: u32 = 2;
pub const FLB_PROXY_OUTPUT_PLUGIN: u32 = 2;
pub const FLB_PROXY_GOLANG: u32 = 11;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flb_plugin_proxy_def {
    pub type_: c_int,
    pub proxy: c_int,
    pub flags: c_int,
    pub name: *mut c_char,
    pub description: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flb_api {
    pub output_get_property: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut c_char,
            arg2: *mut c_void,
        ) -> *mut c_char,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flb_plugin_proxy_context {
    pub remote_context: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flbgo_output_plugin {
    pub __: *mut c_void,
    pub api: *mut flb_api,
    pub o_ins: *mut flb_output_instance,
    pub context: *mut flb_plugin_proxy_context,
}

extern "C" {
    pub fn output_get_property(
        key: *mut c_char,
        plugin: *mut c_void,
    ) -> *mut c_char;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flb_output_instance {
    pub _address: u8,
}

//************************ RUST TYPES********************** */

pub type FLBPluginProxyDef = flb_plugin_proxy_def;
pub type FLBOutPlugin = flbgo_output_plugin;

#[derive(Default)]
pub struct PluginInfo {
    pub name: String,
    pub description: String,
}

/// Trait which defines the basic function that shloud be implemented by any rust plugin
pub trait PluginFunctions {

    /// A plugin register method
    /// 
    /// When the FLBPluginInit is triggered by Fluent Bit, a plugin context
    /// is passed and the next step is to invoke this FLBPluginRegister() function
    /// to fill the required information: name and
    /// description.
    fn plugin_register(&mut self, info: &mut PluginInfo);

    /// Before the engine starts, it initialize all plugins that were requested to start
    /// Useful for initializing any data structure that would be used for this plugin during the processing
    fn plugin_init(&mut self);

    /// Upon flush time, when Fluent Bit want's to flush it buffers, the runtime flush callback will be triggered.
    /// The callback will receive a raw buffer of msgpack data.
    /// The passed data is a bytes buffer which could be processed in anyway. A there are differents crates that would be helpful 
    /// in the ecosystem, such as 
    /// [`rmp`](https://crates.io/crates/rmp), [`msgpack`](https://crates.io/crates/msgpack) 
    /// or you can implement a custom parser by using one of most parser libraries in the rust ecosystem.
    fn plugin_flush(&mut self, data: &[u8]);

    /// When Fluent Bit will stop using the instance of the plugin, it will trigger the exit callback.
    fn plugin_exit(&mut self);
}

#[macro_export]
macro_rules! create_boilerplate{
    ($e:expr) => {
        
        use libc::{c_char, c_int, c_void};
        use std::ffi::{CStr, CString, NulError};
        use std::slice;
        use std::sync::{Arc, Mutex};
        use std::cell::RefCell;
        #[macro_use]
        extern crate lazy_static;
        
       lazy_static! {
            static ref handler: Arc<Mutex<Box<PluginFunctions + Send>>> = Arc::new(Mutex::new(Box::new($e))); // si
       }

        #[no_mangle]
        pub extern fn FLBPluginRegister( ptr: *mut c_void) -> c_int {
            unsafe{
                let p = &mut *(ptr as *mut FLBPluginProxyDef);
                p.type_ = FLB_PROXY_OUTPUT_PLUGIN as c_int;
                p.proxy = FLB_PROXY_GOLANG as c_int;
                p.flags = 0;
                let mut plugin_info = PluginInfo::default();
                handler.lock().unwrap().plugin_register(&mut plugin_info);
                match CString::new(plugin_info.name.as_bytes()) {
                    Ok(cname) => {
                        p.name = cname.into_raw() as *mut _
                    },
                    _   => return -1,
                }

                match CString::new(plugin_info.description.as_bytes()) {
                    Ok(d) => {
                        p.description = d.into_raw() as *mut _;
                        println!("description");

                    },
                    _ => return -1,
                }
                FLB_OK as c_int
            }
        }

        #[no_mangle]
        pub extern fn FLBPluginInit(ptr: *mut c_void) -> c_int {
            handler.lock().unwrap().plugin_exit();
	        FLB_OK as c_int
        }

        #[no_mangle]
        pub extern fn FLBPluginFlush(data: *mut c_void, length: c_int, tag: *const c_char) -> c_int {
            unsafe{
                let bytes = slice::from_raw_parts(data as *const _, length as usize);
                handler.lock().unwrap().plugin_flush(bytes);
            }
            FLB_OK as c_int
        }

        #[no_mangle]
        pub extern fn FLBPluginExit() -> c_int {
            handler.lock().unwrap().plugin_exit();
            FLB_OK as c_int
        }
    }
}
