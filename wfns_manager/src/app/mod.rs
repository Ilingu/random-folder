mod backend;
pub mod preferences;

use std::time::Duration;

use adw::prelude::*;
use backend::AppFolderManager;
use preferences::AppPreferences;
use relm4::{
    abstractions::Toaster, adw, factory::FactoryVecDeque, gtk, Component, ComponentController,
    ComponentParts, ComponentSender, Controller, RelmWidgetExt, SimpleComponent,
};

use crate::components::{
    about::{AboutInput, AboutPageModel},
    fav_folder::FavFolderModel,
    header::{HeaderInput, HeaderModel, HeaderOutput},
};

macro_rules! push_toast {
    ($e:expr, $f:expr, $sender:expr) => {
        $sender.input(AppInput::PushToast((
            $e.to_string(),
            Duration::from_secs($f),
        )))
    };
}

#[derive(Debug)]
enum AppPages {
    ChooseFolder,
    ViewFolder,
}

pub struct AppModel {
    prefs: AppPreferences,
    curr_folder: Option<AppFolderManager>,
    current_page: AppPages,

    // factories
    favs_folders: FactoryVecDeque<FavFolderModel>,

    // components
    header: Controller<HeaderModel>,
    about_page: Controller<AboutPageModel>,
    toaster: Toaster,
}

#[derive(Debug)]
pub enum AppInput {
    OpenAbout,
    ChooseFolder,
    AddFolder(String),
    PushToast((String, Duration)),
    SwitchPage(AppPages),
    SetBookmarked(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Input = AppInput;
    type Output = ();
    type Init = AppPreferences;

    view! {
        main_window = gtk::Window {
            set_title: Some("WFNS Manager"),
            set_maximized: true,
            set_titlebar: Some(model.header.widget()),
            set_icon_name: Some("logo"),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                #[local_ref]
                toast_overlay -> adw::ToastOverlay {
                    set_vexpand: true,
                    set_hexpand: true,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_margin_all: 5,
                        set_valign: gtk::Align::Center,

                        // Here lie the app UI code
                        gtk::Stack {
                            set_transition_type: gtk::StackTransitionType::SlideLeftRight,
                            set_transition_duration: 500,

                            #[watch]
                            set_visible_child_name: &format!("{:?}", model.current_page).to_lowercase(),

                            add_named[Some("choosefolder")] = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,

                                adw::StatusPage {
                                    set_icon_name: Some("logo"),
                                    set_title: "Welcome to WFNS!",
                                    set_description: Some("Enjoy your special folders"),

                                    gtk::Button {
                                            set_css_classes: &["suggested-action", "pill"],
                                            set_label: "Choose Folder!",
                                            set_use_underline: true,
                                            set_halign: gtk::Align::Center,
                                            connect_clicked => AppInput::ChooseFolder
                                    },
                                },

                                gtk::ListBox {
                                    set_selection_mode: gtk::SelectionMode::None,
                                    set_halign: gtk::Align::Center,
                                    set_css_classes: &["boxed-list"],
                                    set_width_request: 275,
                                    #[watch]
                                    set_visible: !model.favs_folders.is_empty(),

                                    #[local_ref]
                                    favs_folders_factory -> adw::ExpanderRow {
                                        set_title: "Favorites folders",
                                        set_subtitle: "Click to expend",
                                        set_enable_expansion: true,
                                    },
                                }

                            },

                            add_named[Some("viewfolder")]  = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 5,
                            }
                        }
                    }
                }
            }
        }
    }

    fn init(
        prefs: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // components init
        let header: Controller<HeaderModel> = HeaderModel::builder()
            .launch((false, false))
            .forward(sender.input_sender(), |msg| match msg {
                HeaderOutput::About => AppInput::OpenAbout,
                HeaderOutput::NewDir => AppInput::ChooseFolder,
                HeaderOutput::SetBookmarked(b) => AppInput::SetBookmarked(b),
            });
        let about_page = AboutPageModel::builder()
            .transient_for(&root)
            .launch(true)
            .detach();

        // factories
        let mut favs_folders = FactoryVecDeque::builder()
            .launch(adw::ExpanderRow::default())
            .detach();
        for fpath in &prefs.favs_folders {
            favs_folders.guard().push_back(fpath.to_owned()); // set init value
        }

        // define default model
        let model = AppModel {
            prefs,
            curr_folder: None,
            current_page: AppPages::ChooseFolder,
            favs_folders,

            header,
            about_page,
            toaster: Toaster::default(),
        };

        // inject to view!
        let toast_overlay = model.toaster.overlay_widget();
        let favs_folders_factory = model.favs_folders.widget();

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppInput::OpenAbout => {
                if self.about_page.sender().send(AboutInput::Show).is_err() {
                    push_toast!("Failed to open about page", 2, sender);
                }
            }
            AppInput::ChooseFolder => {
                let dialog = gtk::FileDialog::builder()
                    .title("Choose folder")
                    .initial_folder({
                        let pics_raw = dirs::picture_dir().unwrap();
                        let pics = pics_raw.to_str().unwrap();
                        &gtk::gio::File::for_path(pics)
                    })
                    .build();
                dialog.select_folder(
                    None::<&gtk::Window>,
                    None::<&gtk::gio::Cancellable>,
                    move |result| match result {
                        Ok(f) => {
                            if let Some(path) =
                                f.path().and_then(|p| p.to_str().map(|s| s.to_string()))
                            {
                                sender.input(AppInput::AddFolder(path))
                            } else {
                                push_toast!("Failed to choose folder", 2, sender)
                            }
                        }
                        _ => push_toast!("Failed to choose folder", 2, sender),
                    },
                )
            }
            AppInput::AddFolder(path) => {
                let folder = match AppFolderManager::set_folder(path) {
                    Ok(f) => f,
                    Err(_) => {
                        return push_toast!(
                            "Failed to gather information about this folder",
                            4,
                            sender
                        )
                    }
                };
                self.curr_folder = Some(folder);
                println!("{:?}", self.curr_folder);
                let _ = self
                    .header
                    .sender()
                    .send(HeaderInput::ShowBookmarkBtn(true));
                sender.input(AppInput::SwitchPage(AppPages::ViewFolder));
            }
            AppInput::SetBookmarked(bookmarked) => {
                if let Some(folder) = &self.curr_folder {
                    match bookmarked {
                        true => match self.prefs.favs_folders.insert(folder.root_path.to_owned()) {
                            true => push_toast!("Successfully bookmarked", 2, sender),
                            false => {
                                push_toast!("Failed to bookmark folder", 2, sender);
                                let _ = self.header.sender().send(HeaderInput::RollbackBookmark);
                            }
                        },
                        false => match self.prefs.favs_folders.remove(&folder.root_path) {
                            true => push_toast!("Successfully unbookmarked", 2, sender),
                            false => {
                                push_toast!("Failed to unbookmark folder", 2, sender);
                                let _ = self.header.sender().send(HeaderInput::RollbackBookmark);
                            }
                        },
                    }
                    let _ = self.prefs.save();
                }
            }
            AppInput::PushToast((text, timeout)) => {
                let toast = adw::Toast::builder()
                    .title(text)
                    .button_label("Cancel")
                    .timeout(timeout.as_secs() as u32)
                    .build();
                toast.connect_button_clicked(move |this| this.dismiss());
                self.toaster.add_toast(toast);
            }
            AppInput::SwitchPage(page) => self.current_page = page,
        };
    }
}
