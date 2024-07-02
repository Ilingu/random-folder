use adw::prelude::*;
use relm4::{
    adw, gtk,
    prelude::{DynamicIndex, FactoryComponent},
    FactorySender,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PiDigitState {
    Right,
    Wrong,
    Placeholder,
}

#[derive(Clone)]
pub struct FavFolderModel {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum PiDigitInput {
    UpdateDigitState((u8, PiDigitState)),
}

#[relm4::factory(pub)]
impl FactoryComponent for FavFolderModel {
    type ParentWidget = gtk::Grid;
    type Input = PiDigitInput;
    type Output = ();
    type Init = (u8, PiDigitState, u8);
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Button {
            #[watch]
            set_css_classes: &["pill", "title-3",
                match self.state {
                    PiDigitState::Right => "suggested-action",
                    PiDigitState::Wrong => "destructive-action",
                    PiDigitState::Placeholder => "raised",
                }
            ],
            #[watch]
            set_label: &self.digit.to_string(),
        }
    }

    fn init_model(
        (digit, state, dpr): Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            digit,
            state,
            digits_per_row: dpr,
        }
    }

    fn update(&mut self, message: Self::Input, _s: FactorySender<Self>) {
        match message {
            PiDigitInput::UpdateDigitState((digit, state)) => {
                self.digit = digit;
                self.state = state;
            }
        };
    }
}
