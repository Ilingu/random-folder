use gtk::prelude::*;
use relm4::gtk;

pub fn build_shortcuts_window(root: &gtk::Window) -> gtk::ShortcutsWindow {
    let shortcuts_window = gtk::ShortcutsWindow::builder()
        .modal(true)
        .transient_for(root)
        .width_request(800)
        .height_request(500)
        .build();
    // Add sections, groups, and shortcuts
    let section = gtk::ShortcutsSection::builder()
        .title("General")
        .valign(gtk::Align::Start)
        .build();
    let group = gtk::ShortcutsGroup::builder().title("Application").build();

    let open_new = gtk::ShortcutsShortcut::builder()
        .accelerator("<ctrl><shift>o")
        .title("Open new folder")
        .build();
    let open_sf_img = gtk::ShortcutsShortcut::builder()
        .accelerator("Up")
        .title("Open images in default OS image viewer")
        .build();
    let open_sf = gtk::ShortcutsShortcut::builder()
        .accelerator("<ctrl>e")
        .title("Open subfolder in default OS file explorer")
        .build();
    let next = gtk::ShortcutsShortcut::builder()
        .accelerator("Right")
        .title("Pick next subfolder")
        .build();
    let prev = gtk::ShortcutsShortcut::builder()
        .accelerator("Left")
        .title("Rollback to last subfolder")
        .build();
    group.append(&open_new);
    group.append(&open_sf_img);
    group.append(&open_sf);
    group.append(&next);
    group.append(&prev);

    section.append(&group);
    shortcuts_window.set_child(Some(&section));
    shortcuts_window.set_hide_on_close(true);
    shortcuts_window
}
