use adw::prelude::*;
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    adw, gtk, ComponentParts, ComponentSender, SimpleComponent,
};

pub struct HeaderModel {
    bookmarked: bool,
    show_bookmark_btn: bool,
}

#[derive(Debug)]
pub enum HeaderInput {
    ShowBookmarkBtn(bool),
    ToogleBookmark(bool),
    SetBookmark(bool),
}

#[derive(Debug)]
pub enum HeaderOutput {
    About,
    Shortcuts,
    NewDir,
    SetBookmarked(bool),
}

relm4::new_action_group!(HeaderMenuActionGroup, "win");
relm4::new_stateless_action!(OpenAbout, HeaderMenuActionGroup, "about");
relm4::new_stateless_action!(OpenShortcuts, HeaderMenuActionGroup, "shortcuts");

#[relm4::component(pub)]
impl SimpleComponent for HeaderModel {
    type Init = (bool, bool);
    type Input = HeaderInput;
    type Output = HeaderOutput;

    view! {
        #[root]
        header = adw::HeaderBar {
            pack_start = &gtk::Button {
                set_margin_start: 5,
                adw::ButtonContent {
                    set_label: "Open",
                    set_icon_name: "folder-open-symbolic",
                },
                connect_clicked[sender] => move |_| { let _ = sender.output(HeaderOutput::NewDir); },
            },
            pack_start = &gtk::Button {
                set_margin_start: 5,
                #[watch]
                set_visible: model.show_bookmark_btn,

                adw::ButtonContent {
                    #[watch]
                    set_icon_name: match model.bookmarked {
                        true => "user-bookmarks-symbolic",
                        false => "bookmark-new-symbolic",
                    },
                },
                connect_clicked => HeaderInput::ToogleBookmark(true),
            },
            pack_end = &gtk::MenuButton {
                set_icon_name: "open-menu-symbolic",
                #[wrap(Some)]
                set_popover = &gtk::PopoverMenu::from_model(Some(&main_menu)) {}
            },
        }
    }

    menu! {
        main_menu: {
            "About WFNS" => OpenAbout,
            "Shortcuts" => OpenShortcuts,
        }
    }

    fn init(
        (bookmarked, show): Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = HeaderModel {
            bookmarked,
            show_bookmark_btn: show,
        };
        let widgets = view_output!();

        // register actions for menu
        {
            let about_sender = sender.clone();
            let action_about: RelmAction<OpenAbout> = RelmAction::new_stateless(move |_| {
                let _ = about_sender.output(HeaderOutput::About);
            });

            let shortcuts_sender = sender.clone();
            let action_shortcuts: RelmAction<OpenShortcuts> =
                RelmAction::new_stateless(move |_| {
                    let _ = shortcuts_sender.output(HeaderOutput::Shortcuts);
                });

            let mut menu_group = RelmActionGroup::<HeaderMenuActionGroup>::new();
            menu_group.add_action(action_about);
            menu_group.add_action(action_shortcuts);
            menu_group.register_for_widget(&widgets.header);
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            HeaderInput::ShowBookmarkBtn(show) => self.show_bookmark_btn = show,
            HeaderInput::ToogleBookmark(back_to_app) => {
                self.bookmarked = !self.bookmarked;
                if back_to_app {
                    let _ = sender.output(HeaderOutput::SetBookmarked(self.bookmarked));
                }
            }
            HeaderInput::SetBookmark(b) => self.bookmarked = b,
        }
    }
}
