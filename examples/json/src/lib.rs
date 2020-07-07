use fluentbit::*;

use rmpv;
use serde_json;

#[derive(Default)]
struct JsonExample {}

impl FLBPluginMethods for JsonExample {
    fn plugin_register(&mut self, info: &mut PluginInfo) -> FLBResult {
        info.name = "rustout".into();
        info.description = "This is a default description".into();
        Ok(())
    }

    fn plugin_init(&mut self, plugin: &FLBPlugin) -> FLBResult {
        println!("default init");
        let param = plugin
            .config_param("params")
            .map_err(|_| FLBError::FLB_ERROR)?;
        if let Some(p) = param {
            println!("parameter {}", p);
        } else {
            println!("no params");
        }
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
