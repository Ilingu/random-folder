mod app;
mod components;
mod config;

use app::{pref_backend::AppPreferences, AppModel};
use config::APP_ID;
use relm4::{
    gtk::{self, gdk, gio, glib},
    RelmApp,
};

fn main() {
    glib::set_application_name("WFNS Manager");

    // create app
    let app = RelmApp::new(APP_ID);

    // init icons
    initialize_custom_icons();
    gtk::Window::set_default_icon_name("logo");

    // launch app
    app.run::<AppModel>(AppPreferences::load());
}

fn initialize_custom_icons() {
    gio::resources_register_include!("../../../../../icons.gresource").unwrap();

    let display = gdk::Display::default().unwrap();
    let theme = gtk::IconTheme::for_display(&display);
    theme.add_resource_path("/com/ilingu/icons/");
}
