
pub trait PluginInformation {
    fn name(&self) -> String;
    fn get_instance(&self) -> Box<SayHello>;
}

pub trait SayHello {
    fn say_hello(&self) -> String;
}
