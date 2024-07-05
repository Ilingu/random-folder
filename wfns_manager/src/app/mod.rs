mod backend;
pub mod preferences;

use std::time::Duration;

use adw::prelude::*;
use backend::{AppFolderManager, SubFolder};
use preferences::AppPreferences;
use relm4::{
    abstractions::Toaster,
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    adw,
    factory::FactoryVecDeque,
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    RelmWidgetExt, SimpleComponent,
};

use crate::components::{
    about::{AboutInput, AboutPageModel},
    fav_folder::{FavFolderModel, FavFolderOutput},
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

// actions
relm4::new_action_group!(ShortcutsActionGroup, "app_shortcuts");
relm4::new_stateless_action!(NextSFAction, ShortcutsActionGroup, "next");
relm4::new_stateless_action!(PrevSFAction, ShortcutsActionGroup, "prev");
relm4::new_stateless_action!(OpenDir, ShortcutsActionGroup, "open_dir");

// Model

#[derive(Debug)]
pub enum AppPages {
    ChooseFolder,
    ViewFolder,
}

pub struct AppModel {
    prefs: AppPreferences,
    current_page: AppPages,

    curr_folder: Option<AppFolderManager>,
    curr_sf: Option<SubFolder>,

    // factories
    favs_folders: FactoryVecDeque<FavFolderModel>,

    // components
    header: Controller<HeaderModel>,
    about_page: Controller<AboutPageModel>,
    toaster: Toaster,
    shortcuts_window: gtk::ShortcutsWindow,
}

#[derive(Debug)]
pub enum AppInput {
    OpenAbout,
    OpenShortcuts,
    ChooseFolder,
    AddFolder(String),
    NextSF,
    PrevSF,
    PushToast((String, Duration)),
    SwitchPage(AppPages),
    SetBookmarked(bool),
}

// component
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
                        set_hexpand: true,
                        set_vexpand: true,

                        // Here lie the app UI code
                        gtk::Stack {
                            set_transition_type: gtk::StackTransitionType::SlideLeftRight,
                            set_transition_duration: 500,
                            set_hexpand: true,
                            set_vexpand: true,

                            #[watch]
                            set_visible_child_name: &format!("{:?}", model.current_page).to_lowercase(),

                            add_named[Some("choosefolder")] = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_valign: gtk::Align::Center,

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
                                        set_hexpand: true,
                                    },
                                }
                            },

                            add_named[Some("viewfolder")] = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_halign: gtk::Align::Center,
                                set_valign: gtk::Align::Start,
                                // layout with a header card with thumbnail and in the side action button and bellow the images
                                gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_css_classes: &["view", "card"],
                                    set_size_request: (1000, 500),
                                    set_spacing: 10,

                                    gtk::Image {
                                        set_valign:gtk::Align::Center,
                                        #[watch]
                                        set_from_file: match &model.curr_sf {
                                            Some(f) if f.thumbnail.is_some() =>
                                                Some(format!("{}/{}", f.get_path(), f.thumbnail.as_ref().unwrap())),
                                            _ => None::<String>,
                                        },
                                        set_size_request: (100, 300),
                                        inline_css: "border: 1px solid ",
                                    }
                                }
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
                HeaderOutput::Shortcuts => AppInput::OpenShortcuts,
                HeaderOutput::NewDir => AppInput::ChooseFolder,
                HeaderOutput::SetBookmarked(b) => AppInput::SetBookmarked(b),
            });
        let about_page = AboutPageModel::builder()
            .transient_for(&root)
            .launch(true)
            .detach();
        let shortcuts_window = {
            let shortcuts_window = gtk::ShortcutsWindow::builder()
                .modal(true)
                .transient_for(&root)
                .width_request(800)
                .height_request(500)
                .build();
            // Add sections, groups, and shortcuts
            let section = gtk::ShortcutsSection::builder()
                .title("General")
                .valign(gtk::Align::Start)
                .build();
            let group = gtk::ShortcutsGroup::builder().title("Application").build();

            let open = gtk::ShortcutsShortcut::builder()
                .accelerator("<ctrl>o")
                .title("Open new folder")
                .build();
            let next = gtk::ShortcutsShortcut::builder()
                .accelerator("<ctrl>n")
                .title("Pick next subfolder")
                .build();
            let prev = gtk::ShortcutsShortcut::builder()
                .accelerator("<ctrl>p")
                .title("Rollback to last subfolder")
                .build();
            group.append(&open);
            group.append(&next);
            group.append(&prev);

            section.append(&group);
            shortcuts_window.set_child(Some(&section));
            shortcuts_window.set_hide_on_close(true);
            shortcuts_window
        };

        // factories
        let mut favs_folders = FactoryVecDeque::builder()
            .launch(adw::ExpanderRow::default())
            .forward(sender.input_sender(), |msg| match msg {
                FavFolderOutput::ChoseFavFolder(path) => AppInput::AddFolder(path),
            });
        for fpath in &prefs.favs_folders {
            favs_folders.guard().push_back(fpath.to_owned()); // set init value
        }

        // define default model
        let model = AppModel {
            prefs,
            current_page: AppPages::ChooseFolder,

            curr_folder: None,
            curr_sf: None,

            header,
            about_page,
            toaster: Toaster::default(),
            favs_folders,
            shortcuts_window,
        };

        // inject to view!
        let toast_overlay = model.toaster.overlay_widget();
        let favs_folders_factory = model.favs_folders.widget();

        let widgets = view_output!();
        // todo: https://docs.gtk.org/gtk4/class.ShortcutsWindow.html
        // actions
        {
            let app = relm4::main_application();
            app.set_accelerators_for_action::<NextSFAction>(&["<ctrl>n"]);
            app.set_accelerators_for_action::<PrevSFAction>(&["<ctrl>p"]);
            app.set_accelerators_for_action::<OpenDir>(&["<ctrl>o"]);

            let next_sender = sender.clone();
            let action_next: RelmAction<NextSFAction> =
                RelmAction::new_stateless(move |_| next_sender.input(AppInput::NextSF));

            let prev_sender = sender.clone();
            let action_prev: RelmAction<PrevSFAction> =
                RelmAction::new_stateless(move |_| prev_sender.input(AppInput::PrevSF));

            let open_sender = sender.clone();
            let action_open: RelmAction<OpenDir> =
                RelmAction::new_stateless(move |_| open_sender.input(AppInput::ChooseFolder));

            let mut alone_group = RelmActionGroup::<ShortcutsActionGroup>::new();
            alone_group.add_action(action_next);
            alone_group.add_action(action_prev);
            alone_group.add_action(action_open);
            alone_group.register_for_widget(&widgets.main_window);
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppInput::OpenAbout => {
                if self.about_page.sender().send(AboutInput::Show).is_err() {
                    push_toast!("Failed to open about page", 2, sender);
                }
            }
            AppInput::OpenShortcuts => {
                self.shortcuts_window.present();
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
                println!("{folder:?}");
                let _ = self
                    .header
                    .sender()
                    .send(HeaderInput::ShowBookmarkBtn(true));
                let _ = self.header.sender().send(HeaderInput::SetBookmark(
                    self.prefs
                        .favs_folders
                        .iter()
                        .find(|n| n == &&folder.root_path)
                        .map(|_| true)
                        .unwrap_or(false),
                ));
                self.curr_folder = Some(folder);

                sender.input(AppInput::NextSF); // before UI update to prevent user to see "blank" screen
                sender.input(AppInput::SwitchPage(AppPages::ViewFolder));
            }
            AppInput::NextSF => {
                if let Some(folder) = self.curr_folder.as_mut() {
                    self.curr_sf = Some(folder.choose());
                }
            }
            AppInput::PrevSF => {
                if let Some(folder) = self.curr_folder.as_mut() {
                    self.curr_sf = Some(folder.rollback().unwrap_or_else(|| folder.choose()));
                }
            }
            AppInput::SetBookmarked(bookmarked) => {
                if let Some(folder) = &self.curr_folder {
                    match bookmarked {
                        true => match self.prefs.favs_folders.insert(folder.root_path.to_owned()) {
                            true => push_toast!("Successfully bookmarked", 2, sender),
                            false => {
                                push_toast!("Failed to bookmark folder", 2, sender);
                                let _ = self
                                    .header
                                    .sender()
                                    .send(HeaderInput::ToogleBookmark(false));
                            }
                        },
                        false => match self.prefs.favs_folders.remove(&folder.root_path) {
                            true => push_toast!("Successfully unbookmarked", 2, sender),
                            false => {
                                push_toast!("Failed to unbookmark folder", 2, sender);
                                let _ = self
                                    .header
                                    .sender()
                                    .send(HeaderInput::ToogleBookmark(false));
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
