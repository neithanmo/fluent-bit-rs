#![allow(non_camel_case_types)]
use libc::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};

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
struct flb_plugin_proxy_context {
    remote_context: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flb_output_instance {
    _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flb_api {
    pub output_get_property: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut c_char, arg2: *mut c_void) -> *mut c_char,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flbgo_output_plugin {
    __: *mut c_void,
    pub api: *mut flb_api,
    pub o_ins: *mut flb_output_instance,
    context: *mut flb_plugin_proxy_context,
}

pub type FLBPluginProxyDef = flb_plugin_proxy_def;
pub type FLBOutPlugin = flbgo_output_plugin;

/// Basic plugin information
#[derive(Default)]
pub struct PluginInfo {
    /// Plugin's name
    pub name: String,
    /// Plugin's description
    pub description: String,
}

/// Fluent-bit error definitions
pub enum FLBError {
    /// The data have been processed normally.
    FLB_ERROR,
    /// A recoverable error have ocurred, the engine can try to flush the records/data later.
    FLB_RETRY,
}

/// Custom result for any plugin's operation
pub type FLBResult = Result<(), FLBError>;

impl From<FLBError> for i32 {
    fn from(error: FLBError) -> Self {
        match error {
            FLBError::FLB_ERROR => 0,
            FLBError::FLB_RETRY => 2,
        }
    }
}

/// Trait which defines the functions that should be implemented by any rust plugin
pub trait FLBPluginMethods {
    /// A plugin register method
    ///
    /// When the FLBPluginInit is triggered by Fluent Bit, a plugin context
    /// is passed and the next step is to invoke this FLBPluginRegister() function
    /// to fill the required information: name and
    /// description.
    /// # Arguments
    /// * `info` A mutable reference to a PluginInfo struct where the plugin's name and description will be filled
    /// # Returns
    /// If the operation was successful an Ok(()) is returned otherwise FLBError
    fn plugin_register(&mut self, info: &mut PluginInfo) -> FLBResult;

    /// Before the engine starts, it initialize all plugins that were requested to start
    /// Useful for initializing any data structure that would be used for this plugin during the processing
    /// # Arguments
    /// * `plugin` A reference to the inner plugin representation which could be used for getting
    /// any parameter passed in to the plugin
    /// # Returns
    /// If the operation was successful an Ok(()) is returned otherwise FLBError
    fn plugin_init(&mut self, plugin: &FLBPlugin) -> FLBResult;

    /// Upon flush time, when Fluent Bit want's to flush it buffers, the runtime flush callback will be triggered.
    /// The callback will receive a raw buffer of msgpack data.
    /// The passed data is a bytes buffer which could be processed in anyway. A there are differents crates that would be helpful
    /// in the ecosystem, such as
    /// [`rmp`](https://crates.io/crates/rmp), [`msgpack`](https://crates.io/crates/msgpack)
    /// or you can implement a custom parser by using one of most parser libraries in the rust ecosystem.
    /// # Arguments
    /// * `data` A byte buffer with the message in a MsgPack format
    /// * `tag` A str containing the tag from fluent-bit
    /// # Returns
    /// If the operation was successful an Ok(()) is returned otherwise FLBError
    fn plugin_flush(&mut self, data: &[u8], tag: &str) -> FLBResult;

    /// When Fluent Bit will stop using the instance of the plugin, it will trigger the exit callback.
    /// # Returns
    /// If the operation was successful an Ok(()) is returned otherwise FLBError
    fn plugin_exit(&mut self) -> FLBResult;
}

/// Internal plugin pointer
///
/// This is passed in during the init function to
/// get any parameter that is required during the plugin's
/// initialization
pub struct FLBPlugin {
    pub(crate) plugin_ptr: *mut libc::c_void,
}

impl FLBPlugin {
    // Creates a new plugin ptr instance
    //
    // This method should not be called
    // by the user.
    pub fn new(ptr: *mut c_void) -> Self {
        Self { plugin_ptr: ptr }
    }

    /// Request the value of a param named *key*
    /// # Returns
    /// Returns and option if succeeded
    /// or and error if the key is not valid.
    pub fn config_param<K: AsRef<str>>(&self, key: K) -> Result<Option<String>, ()> {
        unsafe {
            let p = self.plugin_ptr as *mut flbgo_output_plugin;
            let key = CString::new(key.as_ref()).map_err(|_| ())?;
            let param = if let Some(f) = (*(*p).api).output_get_property {
                f(key.as_ptr() as _, (*p).o_ins as _)
            } else {
                return Ok(None);
            };

            if !param.is_null() {
                let result = CStr::from_ptr(param).to_str().map_err(|_| ())?;
                return Ok(Some(result.to_owned()));
            }
            Ok(None)
        }
    }
}

/// This macro will generate the needed boilerplate for output plugins
///
/// Only one plugin instance is supported, later multi-instance plugins support would be added
#[macro_export]
macro_rules! create_boilerplate{
    ($e:expr) => {

        use std::panic::{self, AssertUnwindSafe};

        use libc::{c_char, c_int, c_void};
        use std::ffi::{CStr, CString, NulError};
        use std::slice;
        use std::sync::{Mutex};
        use std::cell::RefCell;
        #[macro_use]
        extern crate lazy_static;

       lazy_static! {
            static ref handler: Mutex<Box<dyn FLBPluginMethods + Send>> = Mutex::new(Box::new($e));
       }


       // Catch panics
       fn catch<F: FnOnce() -> c_int>(f: F) -> Option<c_int> {
            match panic::catch_unwind(AssertUnwindSafe(f)) {
                Ok(ret) => Some(ret),
                Err(_) => {
                    exit(std::line!(), std::file!());
                    None
                }
            }
        }

        fn exit(line: u32, file: &str) {
            eprintln!("catching panic generated at: {} in file: {}", line, file);
            eprintln!("exiting process!");
            std::process::exit(-1);
        }

        #[no_mangle]
        pub extern fn FLBPluginRegister( ptr: *mut c_void) -> c_int {
            catch(|| unsafe {

                let p = &mut *(ptr as *mut FLBPluginProxyDef);
                p.type_ = FLB_PROXY_OUTPUT_PLUGIN as c_int;
                p.proxy = FLB_PROXY_GOLANG as c_int;
                p.flags = 0;

                let mut plugin_info = PluginInfo::default();

                match handler.lock().unwrap().plugin_register(&mut plugin_info){
                    Ok(()) => {
                        if let Ok(cname) = CString::new(plugin_info.name.as_bytes()) {
                            p.name = cname.into_raw() as *mut _;
                        } else {
                            return FLB_ERROR as c_int;
                        }

                        if let Ok(d) = CString::new(plugin_info.description.as_bytes()) {
                            p.description = d.into_raw() as *mut _;
                        } else {
                            return FLB_ERROR as c_int;
                        }
                    },
                    Err(e) => return i32::from(e)  as c_int,
                }
                FLB_OK as c_int
            }).unwrap()
        }


        #[no_mangle]
        pub extern fn FLBPluginInit(ptr: *mut c_void) -> c_int {
            let plugin = FLBPlugin::new(ptr);
            catch(|| {
                if let Err(e) = handler.lock().unwrap().plugin_init(&plugin){
                    return i32::from(e) as c_int;
                }
                FLB_OK as c_int
            }).unwrap()
        }

        #[no_mangle]
        pub extern fn FLBPluginFlush(data: *mut c_void, length: c_int, tag: *const c_char) -> c_int {
            catch( || unsafe {
                let bytes = slice::from_raw_parts(data as *const _, length as usize);
                let tag = match CStr::from_ptr(tag).to_str() {
                    Ok(str) => str,
                    _ => return FLB_ERROR as c_int,
                };
                if let Err(e) = handler.lock().unwrap().plugin_flush(bytes, tag){
                    return i32::from(e)  as c_int;
                }
                FLB_OK as c_int
            }).unwrap()
        }

        #[no_mangle]
        pub extern fn FLBPluginExit() -> c_int {
            catch( || {
                if let Err(e) = handler.lock().unwrap().plugin_exit(){
                    return i32::from(e) as c_int;
                }
                FLB_OK as c_int
            }).unwrap()
        }

        #[no_mangle]
        pub extern fn FLBPluginUnregister( ptr: *mut c_void) -> c_int {
            catch(|| unsafe {

                let p = &mut *(ptr as *mut FLBPluginProxyDef);
                p.type_ = FLB_PROXY_OUTPUT_PLUGIN as c_int;
                p.proxy = FLB_PROXY_GOLANG as c_int;
                p.flags = 0;

                let _ = CString::from_raw(p.name);
                let _ = CString::from_raw(p.description);

                FLB_OK as c_int
            }).unwrap()
        }
    }
}
