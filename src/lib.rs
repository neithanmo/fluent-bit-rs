//! # fluentbit    
//! This crate aims to build output plugins for [Fluent-bit](https://fluentbit.io/).
//! It is based on  the Go interface for writting output plugins.
//!
//! This crate is still in heavy development. At this point, Multiple-instance plugin is not supported
//! but it would be added very soon.
//! # Hello World
//! A simple Hello world example which prints data to stdout, it involves following three steps:
//! - Create a struct/enum with the data you might use for processing the incoming buffer from Fluent-bit
//! - Implement the trait *FLBPluginMethods* for that struct/enum.
//! - After steps 1 and 2, call the macro create_boilerplate!() which will generate the boilerplate code that every plugin should have, 
//!   taking care of the unsafe safe code and abstracting it into a safe Rust style. 
//!   This macro will accept as an argument any type which implements the FLBPluginMethods trait
//!
//! And that's all. Compile your plugin as a dynamic library by adding this line to your Cargo.toml
//!
//! ```
//! [lib]
//! crate-type=["cdylib"]
//! ```
//!
//! Another thing that is worth to mention is that Fluent-bit should be able to load Go plugins even though
//! your plugin was written in Rust. To enable external plugins'  support  you have to compile Fluent-bit with Goland support, e.g:
//!
//! ```
//! $ cd build/
//! $ cmake -DFLB_DEBUG=On -DFLB_PROXY_GO=On ../
//! $ make && make install
//! ```
//! Once compiled, you can see a new option in the binary -e which stands for external plugin, e.g:
//! ```
//! $ bin/fluent-bit -h
//! Usage: fluent-bit [OPTION]

//! Available Options
//!  -c  --config=FILE	specify an optional configuration file
//!  -d, --daemon		run Fluent Bit in background mode
//!  -f, --flush=SECONDS	flush timeout in seconds (default: 5)
//!  -i, --input=INPUT	set an input
//!  -m, --match=MATCH	set plugin match, same as '-p match=abc'
//!  -o, --output=OUTPUT	set an output
//!  -p, --prop="A=B"	set plugin configuration property
//!  -e, --plugin=FILE	load an external plugin (shared lib)
//!  ...
//! ```
//!
//! Now here is a simple output plugin
//!
//! ```
//! extern crate fluentbit;
//! use fluentbit::*;
//! 
//! extern crate rmpv;
//! extern crate serde_json;
//! 
//! extern crate serde;
//! 
//! 
//! #[derive(Default)]
//! struct JsonExample{}
//! 
//! impl FLBPluginMethods for JsonExample{
//!     
//!     fn plugin_register(&mut self, info: &mut PluginInfo) -> FLBResult{
//!         info.name = "rustout".into();
//!         info.description = "This is a default description".into();
//!         Ok(())
//!     }
//! 
//!     fn plugin_init(&mut self) -> FLBResult{
//!         println!("default init");
//!         Ok(())
//!     }
//! 
//!     fn plugin_flush(&mut self, data: &[u8]) -> FLBResult{
//! 
//!         let mut value = data.clone();
//!         let value: rmpv::Value = rmpv::decode::value::read_value(&mut value).unwrap();
//!         let json = serde_json::to_string_pretty(&value).unwrap();
//!         
//!         println!("{}", json);
//!         Ok(())
//!     }
//! 
//!     fn plugin_exit(&mut self) -> FLBResult{
//!         println!("exiting");
//!         Ok(())
//!     }
//!     
//! }
//! 
//! create_boilerplate!(JsonExample::default());
//! ```
//!
//! Test your plugin:
//! ```
//! cargo build --release
//! fluent-bit -e target/release/libexample.so -i cpu -o "rustout"
//! ```

mod fluent;
pub use fluent::*;
