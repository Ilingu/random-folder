mod backend;
mod ui;

use std::rc::Rc;
use std::sync::Mutex;

use adw::prelude::*;

use adw::Application;
use backend::AppBackend;
use ui::build_ui;

fn main() {
    let application = Application::builder()
        .application_id("com.ilingu.randomfolder")
        .build();

    application.connect_activate(|app| {
        let backend = AppBackend::new();
        build_ui(app, Rc::new(Mutex::new(backend)))
    });
    application.run();
}
