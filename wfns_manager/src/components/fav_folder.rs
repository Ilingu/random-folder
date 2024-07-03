use adw::prelude::*;
use relm4::{
    adw,
    prelude::{DynamicIndex, FactoryComponent},
    FactorySender,
};

#[derive(Clone)]
pub struct FavFolderModel {
    pub name: String,
}

#[relm4::factory(pub)]
impl FactoryComponent for FavFolderModel {
    type ParentWidget = adw::ExpanderRow;
    type Input = ();
    type Output = ();
    type Init = String;
    type CommandOutput = ();

    view! {
        #[root]
        adw::ActionRow {
            set_title: &self.name,
        }
    }

    fn init_model(name: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { name }
    }

    // fn update(&mut self, message: Self::Input, _s: FactorySender<Self>) {}
}
