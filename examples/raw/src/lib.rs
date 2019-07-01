extern crate fluentbit;
use fluentbit::*;

#[derive(Default)]
struct MyPluginDemo{
    num_flushes: u32,
}

impl FLBPluginMethods for MyPluginDemo{
    
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
        println!("data: {:?}", data);
        self.num_flushes += 1;
        println!("FLUSH NUMBER {}", self.num_flushes);
        Ok(())
    }

    fn plugin_exit(&mut self) -> FLBResult{
        println!("exiting");
        Ok(())
    }
    
}

create_boilerplate!(MyPluginDemo::default());