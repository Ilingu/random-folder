use adw::prelude::*;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::config;

pub struct AboutPageModel {
    hidden: bool,
}

#[derive(Debug)]
pub enum AboutInput {
    Show,
    Hide,
}

#[relm4::component(pub)]
impl SimpleComponent for AboutPageModel {
    type Input = AboutInput;
    type Output = ();
    type Init = bool;

    view! {
        #[root]
        adw::AboutWindow {
          set_modal: true,

          set_application_name: "WFNS Manager",
          set_application_icon: "logo",
          set_developer_name: "Ilingu",
          set_issue_url: "https://github.com/Ilingu/random-folder/issues",
          set_version: config::VERSION,
          set_website: "https://github.com/Ilingu/random-folder/tree/main/wfns_manager",
          set_comments: "Encrypts and enjoy your special folders",
          set_license_type: gtk::License::MitX11,
          set_copyright :"© 2024 Ilingu",


          #[watch]
          set_visible: !model.hidden,
          connect_close_request[sender] => move |_| {
                sender.input(AboutInput::Hide);
                gtk::glib::Propagation::Stop
          }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AboutPageModel { hidden: init };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _s: ComponentSender<Self>) {
        match msg {
            AboutInput::Show => self.hidden = false,
            AboutInput::Hide => self.hidden = true,
        }
    }
}
