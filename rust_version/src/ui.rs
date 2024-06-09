use std::rc::Rc;
use std::sync::Mutex;

use gtk::prelude::*;
use gtk::Application;

use crate::backend::AppBackend;
use crate::window::Window;

pub fn build_ui(app: &Application, backend: Rc<Mutex<AppBackend>>) {
    let window = Window::new(app);
    window.present();
}

/*
fn build_ui_old(app: &Application, backend: Rc<Mutex<AppBackend>>) {
    // Markup
    let choose_folder_btn = ActionRow::builder()
        .activatable(true)
        .title("Choose folder")
        .build();

    // Assemblage
    let list = ListBox::builder()
        .margin_top(32)
        .margin_end(32)
        .margin_bottom(32)
        .margin_start(32)
        .selection_mode(SelectionMode::None)
        // makes the list look nicer
        .css_classes(vec![String::from("boxed-list")])
        .build();
    list.append(&choose_folder_btn);

    // Combine the content in a box
    let content = Box::new(Orientation::Vertical, 0);
    // Adwaitas' ApplicationWindow does not include a HeaderBar
    content.append(&HeaderBar::new());
    content.append(&list);

    // create window
    let window = Rc::new(
        ApplicationWindow::builder()
            .application(app)
            .title("Random Folder 📁")
            .decorated(true)
            .fullscreened(false)
            // add content to window
            .content(&content)
            .visible(true)
            .build(),
    );

    // events
    let bec = Rc::clone(&backend);
    choose_folder_btn.connect_activated(move |_| AppBackend::load(bec.clone(), Rc::clone(&window)));
}
*/
