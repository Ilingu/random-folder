use std::env;

fn main() {
    env::set_var("OUT_DIR", "./");
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "resources/bin/compiled.gresource",
    );
}
