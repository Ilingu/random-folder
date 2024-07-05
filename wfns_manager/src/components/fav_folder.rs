use adw::prelude::*;
use relm4::{
    adw, gtk,
    prelude::{DynamicIndex, FactoryComponent},
    FactorySender,
};

#[derive(Clone)]
pub struct FavFolderModel {
    pub path: String,
}

#[derive(Debug)]
pub enum FavFolderOutput {
    ChoseFavFolder(String),
}

#[relm4::factory(pub)]
impl FactoryComponent for FavFolderModel {
    type ParentWidget = adw::ExpanderRow;
    type Input = ();
    type Output = FavFolderOutput;
    type Init = String;
    type CommandOutput = ();

    view! {
        #[root]
        adw::ActionRow {
            set_title: &self.path,
            set_hexpand: true,
            add_suffix = &gtk::Button {
                set_margin_start: 5,
                adw::ButtonContent {
                    set_icon_name: "check-mini",
                },
                set_widget_name: &self.path, // just for the path to be easily collected bellow
                connect_clicked[sender] => move |b| {
                    let _ = sender.output(FavFolderOutput::ChoseFavFolder(b.widget_name().to_string())); // can't move self..
                },
            },
        }
    }

    fn init_model(path: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { path }
    }
}
