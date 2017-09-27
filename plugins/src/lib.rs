extern crate plugin_test_api as plugin_api;
//extern crate nom;
//extern crate french;

mod english;
mod french;

// should we do that?
pub use english::*;
pub use french::*;

use plugin_api::PluginInformation;
use std::collections::hash_map::HashMap;


pub struct Plugins {
  pub list: HashMap<String, Box<PluginInformation> >,
}

///FIXME: should return a static list of plugins
///FIXME: the plugin tool must know about  the traits
///PluginInfo and the PluginMetadata from each plugin
/// do we import them by predefined name or declare them somewhere?
pub fn plugins() -> Plugins {
  let mut h:  HashMap<String, Box<PluginInformation> > = HashMap::new();
  h.insert("english".to_string(), Box::new(english::PLUGIN_METADATA));
  h.insert("french".to_string(), Box::new(french::PLUGIN_METADATA));

  Plugins {
    list: h
  }
}

