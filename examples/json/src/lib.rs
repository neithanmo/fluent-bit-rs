extern crate fluentbit;
use fluentbit::*;

extern crate rmpv;
extern crate serde;
extern crate serde_json;

#[derive(Default)]
struct JsonExample {}

impl FLBPluginMethods for JsonExample {
    fn plugin_register(&mut self, info: &mut PluginInfo) -> FLBResult {
        info.name = "rustout".into();
        info.description = "This is a default description".into();
        Ok(())
    }

    fn plugin_init(&mut self) -> FLBResult {
        println!("default init");
        Ok(())
    }

    fn plugin_flush(&mut self, data: &[u8], tag: &str) -> FLBResult {
        let mut value = data.clone();
        let value: rmpv::Value = rmpv::decode::value::read_value(&mut value).unwrap();
        let json = serde_json::to_string_pretty(&value).unwrap();

        println!("tag: {} - data: {} \n", tag, json);

        Ok(())
    }

    fn plugin_exit(&mut self) -> FLBResult {
        println!("exiting");
        Ok(())
    }
}

create_boilerplate!(JsonExample::default());
