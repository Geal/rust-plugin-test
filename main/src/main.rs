extern crate plugin_test_api as plugin_api;
extern crate plugin_test_plugins as plugins;

use plugin_api::SayHello;

fn main() {
    let all_plugins = plugins::plugins();

    println!("saying hello in:");
    for (ref name, ref plugin) in all_plugins.list.iter() {
        let instance = plugin.get_instance();
        println!("\t{}: \"{}\"", name, (*instance).say_hello());
    }

}
