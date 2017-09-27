# how to make a plugin system in Rust

## First idea: in tree (or build folder)

- pass an env var for the folder containing plugins
- read a metadata file
- generate the mod.rs file from this metadata
  - the mod file will declare a module for every plugin
  - the mod file will declare a function that returns all the plugins and their info (what they do, etc)

problem: what if a plugin needs to depend on a crate?
- it has to be declared in the top level lib.rs or main.rs
- it must also be declared in the project's Cargo.toml

declaring in the top level file could be done with a procedural macro
that can go read the metadata.
-> but it's in nightly only

declaring in Cargo.toml is an issue, since we must modify that file

we might want to have a stable version with a pre-declared set of plugins
we might want to publish the software on crates.io, so every dependency must
be a published crate

## Second idea

Use a dependency to a crate from a published version or a subfolder
cf lapin-futures, Cargo.toml:
lapin-async = {version = "^0.10", path = "../async"}

We would thus define:
myproject-plugins = {version = "^0.1", path = "./plugins" }

The plugins folder would not appear in the project by default
We can ship a version that builds on stable and can be downloaded from crates.io
(the plugin published would have a limited set of default plugins)

if someone wants a custom build with the plugins they need, they can:
- download the source
- run a script to create that plugins folder (or just create the plugins folder manually?)
- add the plugins they want to the folder (plugins/src/plugin1, plugins/src/plugin2, each with their own mod.rs and a file describing their dependencies)
- run a script to read the metadata of each plugin and generate a Cargo.toml file
  - maybe the build.rs file could detect the plugins folder and run the metadata generation automatically?
    - possibly build.rs runs after parsing Cargo.toml
    - so read the metadata, generate the plugin's Cargo.toml, then stop compilation with a message? (can we show a message to the user?)
- the script wouls also generate the src/lib.rs file

problem: the plugin might depend on structs and other stuff from the main project
so the main project should expose a crate with the things a plugin should use
it might be less flexible than a plugin that can directly reference the source
BUT: plugins are versioned!!!

the metadata tool can check than the plugin's set of dependencies is sane (multiple plugins depending
on a same crate with different versions or options?)
all the plugins should depend on the same exposed plugin API
We might even be able to publish a plugin on crates.io and just declare
them as dependency instead of copying their code

CALL THE TOOL CARGO-PLUGIN
