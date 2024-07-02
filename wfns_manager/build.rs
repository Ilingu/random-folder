use glib_build_tools::compile_resources;
use std::env;

fn main() {
    env::set_var("OUT_DIR", "./");
    compile_resources(&["data"], "data/icons.gresource.xml", "icons.gresource");
}
