mod backend;
mod ui;
mod window;

use std::rc::Rc;
use std::sync::Mutex;

use gtk::prelude::*;

use backend::AppBackend;
use gtk::{gio, Application};
use ui::build_ui;

fn main() {
    // Register and include resources
    gio::resources_register_include!("../../../../../resources/bin/compiled.gresource")
        .expect("Failed to register resources.");

    let application = Application::builder()
        .application_id("com.ilingu.randomfolder")
        .build();

    application.connect_activate(|app| {
        let backend = AppBackend::new();
        build_ui(app, Rc::new(Mutex::new(backend)))
    });
    application.run();
}
