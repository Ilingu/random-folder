use std::{rc::Rc, sync::Mutex};

use adw::{prelude::FileExt, Toast, ToastOverlay};
use gtk::{
    gio::{self, Cancellable},
    glib::{self, object::IsA},
};

#[derive(Debug)]
pub struct AppBackend {
    is_loaded: bool,
    root: Option<String>,
    sub_folders: Option<Vec<String>>,
}

impl AppBackend {
    pub fn new() -> Self {
        Self {
            is_loaded: false,
            root: None,
            sub_folders: None,
        }
    }

    pub fn load<W: IsA<gtk::Window>>(backend: Rc<Mutex<AppBackend>>, window: Rc<W>) {
        // spawn dialog
        folder_chooser(window, move |res| {
            if let Ok(f) = res {
                let path = format!("{:?}", f.path());
                // get subfolders

                let mut backend_state = backend.lock().unwrap();
                backend_state.is_loaded = true;
                backend_state.root = Some(path);
            }
        });
    }
}

fn folder_chooser<W, P>(window: Rc<W>, callback: P)
where
    W: IsA<gtk::Window>,
    P: FnOnce(Result<gio::File, glib::Error>) + 'static,
{
    let dialog = gtk::FileDialog::builder().title("Choose folder").build();
    dialog.select_folder(Some(window.as_ref()), Some(&Cancellable::new()), callback);
    // res.map(|f| ).map_err(|_| ())
}
