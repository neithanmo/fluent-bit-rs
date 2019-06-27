extern crate fluent_bit_rs;
use fluent_bit_rs::*;

#[derive(Default)]
struct MyPluginDemo{
    num_flushes: u32,
}

impl PluginFunctions for MyPluginDemo{
    
    fn plugin_register(&mut self, info: &mut PluginInfo){
        info.name = "rustout".into();
        info.description = "This is a default description".into();
    }

    fn plugin_init(&mut self){
        println!("default init");
    }

    fn plugin_flush(&mut self, data: &[u8]){
        println!("data: {:?}", data);
        self.num_flushes += 1;
        println!("FLUSH NUMBER {}", self.num_flushes);
    }

    fn plugin_exit(&mut self){
        println!("exiting");
    }
    
}

create_boilerplate!(MyPluginDemo::default());