use plugin_api::{PluginInformation,SayHello};

pub struct PluginMetadataType;

pub const PLUGIN_METADATA: PluginMetadataType = PluginMetadataType;

impl PluginInformation for PluginMetadataType {
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
