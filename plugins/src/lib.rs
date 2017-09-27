extern crate plugin_test_api as plugin_api;
extern crate nom;
//extern crate french;

mod english;

// should we do that?
pub use english::*;
//pub use french::*;

use plugin_api::{PluginMetadata,PluginInfo};
use std::collections::hash_map::HashMap;

///FIXME: should return a static list of plugins
///FIXME: the plugin tool must know about  the traits
///PluginInfo and the PluginMetadata from each plugin
/// do we import them by predefined name or declare them somewhere?
pub fn plugins_list() -> HashMap<String,Box<PluginInfo>> {
  let mut h = HashMap::new();
  h.insert("english", Box::new(english::PluginMetadata));
  //h.insert("french", Box::new(french::PluginMetadata));

  h
}
