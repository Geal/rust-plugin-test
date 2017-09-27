use std::env;
use std::path::Path;

fn main() {
  let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not defined");
  let dest_path = Path::new(&out_dir).join("english.rs");
}
