use std::rc::Rc;
use std::sync::Mutex;

use adw::prelude::*;

use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};
use gtk::{Box, ListBox, Orientation, SelectionMode};

use crate::backend::AppBackend;

pub fn build_ui(app: &Application, backend: Rc<Mutex<AppBackend>>) {
    // Markup
    let choose_folder_btn = ActionRow::builder()
        .activatable(true)
        .title("Choose folder")
        .build();
    let print_btn = ActionRow::builder()
        .activatable(true)
        .title("Print backend")
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
    list.append(&print_btn);

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
    print_btn.connect_activated(move |_| println!("{backend:?}"));
}
