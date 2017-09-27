use plugin_api::{PluginInformation,SayHello};

pub struct PluginMetadata;

impl PluginInformation for PluginMetadata {
  fn name(&self) -> String {
    "english".to_string()
  }
}

pub struct English;

impl SayHello for English {
  fn say_hello() -> String {
    "hello, world".to_string()
  }
}
