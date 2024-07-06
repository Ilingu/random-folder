mod actions;
mod backend;
pub mod preferences;

use std::time::Duration;

use adw::prelude::*;
use backend::AppFolderManager;
use preferences::AppPreferences;
use relm4::{
    abstractions::Toaster,
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    adw,
    factory::FactoryVecDeque,
    gtk::{self, EventControllerMotion},
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt,
    SimpleComponent,
};

use crate::{
    components::{
        about::{AboutInput, AboutPageModel},
        fav_folder::{FavFolderModel, FavFolderOutput},
        header::{HeaderInput, HeaderModel, HeaderOutput},
        shortcuts::build_shortcuts_window,
    },
    init_app_actions,
};

// macro

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
relm4::new_stateless_action!(OpenNewDir, ShortcutsActionGroup, "open_new_dir");
relm4::new_stateless_action!(OpenSFImg, ShortcutsActionGroup, "open_sf_img");
relm4::new_stateless_action!(OpenSF, ShortcutsActionGroup, "open_sf");

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

    // factories
    favs_folders: FactoryVecDeque<FavFolderModel>,

    // components
    title_popover: gtk::Popover,
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
    OpenSFImg,
    OpenSF,
    PushToast((String, Duration)),
    SwitchPage(AppPages),
    SetBookmarked(bool),
    TitlePopup(bool),
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
                                set_valign: gtk::Align::Center,
                                // layout with a header card with thumbnail and in the side action button and bellow the images
                                gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_css_classes: &["view", "card"],
                                    set_spacing: 10,
                                    set_margin_top: 20,

                                    gtk::Image {
                                        set_pixel_size: 600,
                                        set_size_request: (400, 600),
                                        set_margin_horizontal: 10,
                                        set_margin_top: 10,

                                        #[watch]
                                        set_from_file: match &model.curr_folder {
                                            Some(f) if f.curr_sf.thumbnail.is_some() =>
                                                Some(format!("{}/{}", f.curr_sf.get_path(), f.curr_sf.thumbnail.as_ref().unwrap())),
                                            _ => None::<String>,
                                        },
                                    },

                                    gtk::Label {
                                        #[watch]
                                        set_label: model.curr_folder.as_ref().map(|f| &f.curr_sf.name).unwrap_or(&String::new()),
                                        add_css_class: "title-2",
                                        set_margin_horizontal: 10,
                                        set_ellipsize: gtk::pango::EllipsizeMode::End,
                                        set_max_width_chars: 50,

                                        add_controller: {
                                            let motion_controller = EventControllerMotion::new();

                                            let sender_enter = sender.clone();
                                            motion_controller.connect_enter(move |_,_,_| sender_enter.input(AppInput::TitlePopup(true)));

                                            let sender_leave = sender.clone();
                                            motion_controller.connect_leave(move |_| sender_leave.input(AppInput::TitlePopup(false)));

                                            motion_controller
                                        },
                                    },

                                    #[name = "popover"]
                                    gtk::Popover {
                                        set_position: gtk::PositionType::Bottom,
                                        set_autohide: false,

                                        gtk::Label {
                                            #[watch]
                                            set_label: model.curr_folder.as_ref().map(|f| &f.curr_sf.name).unwrap_or(&String::new()),
                                            set_margin_all: 12,
                                        }
                                    },

                                    gtk::Box {
                                        set_orientation: gtk::Orientation::Horizontal,
                                        set_halign: gtk::Align::Center,
                                        set_spacing: 10,

                                        gtk::Button {
                                            set_css_classes: &["pill", "suggested-action"],
                                            set_icon_name: "media-playlist-shuffle-symbolic",
                                            connect_clicked => AppInput::NextSF,
                                        },
                                        gtk::Button {
                                            set_css_classes: &["pill", "suggested-action"],
                                            set_icon_name: "media-seek-backward-symbolic",
                                            connect_clicked => AppInput::PrevSF,
                                        },
                                        gtk::Button {
                                            set_css_classes: &["pill", "suggested-action"],
                                            set_icon_name: "eye",
                                            connect_clicked => AppInput::OpenSFImg,
                                        },
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
        let shortcuts_window = build_shortcuts_window(&root);

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
        let mut model = AppModel {
            prefs,
            current_page: AppPages::ChooseFolder,
            curr_folder: None,

            // components
            title_popover: gtk::Popover::default(),
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
        model.title_popover = widgets.popover.clone();
        // actions
        init_app_actions!(sender, widgets);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppInput::OpenAbout => {
                if self.about_page.sender().send(AboutInput::Show).is_err() {
                    push_toast!("Failed to open about page", 2, sender);
                }
            }
            AppInput::OpenShortcuts => self.shortcuts_window.present(),
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
                    match folder.choose(true) {
                        Ok(sf) => folder.curr_sf = sf,
                        Err(_) => {
                            sender.input(AppInput::SwitchPage(AppPages::ChooseFolder));
                            self.curr_folder = None;
                        }
                    }
                }
            }
            AppInput::PrevSF => {
                if let Some(folder) = self.curr_folder.as_mut() {
                    match folder.rollback() {
                        Some(sfr) => folder.curr_sf = sfr,
                        None => {
                            sender.input(AppInput::SwitchPage(AppPages::ChooseFolder));
                            self.curr_folder = None;
                        }
                    }
                }
            }
            AppInput::OpenSFImg => {
                if let Some(folder) = self.curr_folder.as_ref() {
                    folder.curr_sf.open_image();
                }
            }
            AppInput::OpenSF => {
                if let Some(folder) = self.curr_folder.as_ref() {
                    folder.curr_sf.open_dir();
                }
            }
            AppInput::TitlePopup(show) => match show {
                true => self.title_popover.popup(),
                false => self.title_popover.popdown(),
            },
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
