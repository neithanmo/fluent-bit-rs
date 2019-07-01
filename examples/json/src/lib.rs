extern crate fluentbit;
use fluentbit::*;

extern crate rmpv;
extern crate rmp_serde;
extern crate serde_json;
extern crate serde_transcode;

use serde_json::{Serializer, Deserializer};
use std::io::{Read, Write};

use std::io;

#[derive(Default)]
struct JsonExample{}

impl FLBPluginMethods for JsonExample{
    
    fn plugin_register(&mut self, info: &mut PluginInfo) -> FLBResult{
        info.name = "rustout".into();
        info.description = "This is a default description".into();
        Ok(())
    }

    fn plugin_init(&mut self) -> FLBResult{
        println!("default init");
        Ok(())
    }

    fn plugin_flush(&mut self, data: &[u8]) -> FLBResult{

        //let value: rmpv::Value = rmp_serde::decode::from_slice(&data).unwrap();
        let des = rmp_serde::encoder::Serializer::from_slice(&data);
        println!("{:?}", des.deserialize_byte_buf(std::io));
        let json = serde_json::to_string(&value).unwrap();

        println!("{}", json);
        Ok(())
    }

    fn plugin_exit(&mut self) -> FLBResult{
        println!("exiting");
        Ok(())
    }
    
}

create_boilerplate!(JsonExample::default());