mod backend;
pub mod pref_backend;

use std::time::Duration;

use adw::prelude::*;
use backend::AppFolderManager;
use pref_backend::AppPreferences;
use relm4::{
    abstractions::Toaster, adw, gtk, Component, ComponentController, ComponentParts,
    ComponentSender, Controller, RelmWidgetExt, SimpleComponent,
};

use crate::components::{
    about::{AboutInput, AboutPageModel},
    header::{HeaderModel, HeaderOutput},
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
    folder: AppFolderManager,
    current_page: AppPages,
    favs_folders_expended: bool,

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

                                        adw::ExpanderRow {
                                            set_title: "Favorites folders",
                                            set_subtitle: "Click to expend",
                                            set_enable_expansion: true,
                                            add_row = &adw::ActionRow {
                                                set_title: "Expanded content",
                                            },
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
        let header: Controller<HeaderModel> =
            HeaderModel::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    HeaderOutput::About => AppInput::OpenAbout,
                    HeaderOutput::NewDir => AppInput::ChooseFolder,
                });
        let about_page = AboutPageModel::builder()
            .transient_for(&root)
            .launch(true)
            .detach();

        // define default model
        let model = AppModel {
            prefs,
            folder: AppFolderManager::default(),
            current_page: AppPages::ChooseFolder,
            favs_folders_expended: false,

            header,
            about_page,
            toaster: Toaster::default(),
        };

        // inject to view!
        let toast_overlay = model.toaster.overlay_widget();

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
                            }
                        }
                        _ => push_toast!("Failed to choose folder", 2, sender),
                    },
                )
            }
            AppInput::AddFolder(path) => {
                println!("New Folder: {path}")
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
