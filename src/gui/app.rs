use crate::{displayable_err, if_ultimate_version};
use crate::instruments::{Container, DataStore, DisplayableResult};
use crate::helpers::PseudoIterator;
use crate::settings::write_file;
use super::auth::Authorization;
use super::scenes::get_scene;
use super::tools::{Message, Page, ShortElement};

use iced::executor::Default;
use iced::{Application, Command, Theme};

pub struct DeepMathHelper {
    data: DataStore,
    auth: Authorization
}

impl DeepMathHelper {
    fn write_settings(&mut self) {
        if let Err(error) = write_file(&self.data.settings) {
            let message = "Failed to switch theme.";
            if_ultimate_version!(eprintln!("{} {error}", &message));
            self.data.container.pending.push(displayable_err!(message));
        }
    }
}

impl Application for DeepMathHelper {
    type Executor = Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let data = DataStore::default();
        let master_key = data.settings.master_key.clone();

        (
            DeepMathHelper { data, auth: Authorization::from(master_key) },
            Command::none()
        )
    }

    fn title(&self) -> String {
        if_ultimate_version! {{ 
            "Deep Math Helper Ultimate Version".to_owned()
        } else { 
            "Deep Math Helper".to_owned()
        }}
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let none = Command::none();

        match message {
            Message::SwitchTheme => {
                self.data.settings.theme.switch();


            },
            Message::SetPage(page) => {
                if let Page::Selection = page {
                    self.data.container.pending.clear();
                    self.data.container = Container::default();
                }
                
                if_ultimate_version! { // Something like auto-completion???
                    if let Page::QuadraticEquations = page {
                        self.data.container.cell_3 = "0".to_owned(); 
                    } 
                    else if let Page::BasesConverter = page {
                        self.data.container.cell_2 = "10".to_owned(); 
                    }
                }
                
                self.data.current_page = page;
            },
            Message::UpdateCell1(cell_1) => {
                self.data.container.cell_1 = cell_1;
            },
            Message::UpdateCell2(cell_2) => {
                self.data.container.cell_2 = cell_2;
            },
            Message::UpdateCell3(cell_3) => {
                self.data.container.cell_3 = cell_3;
            },
            Message::UpdateCell4(cell_4) => {
                self.data.container.cell_4 = cell_4;
            },
            Message::SwitchTrigonometricPart => {
                self.data.container.trig_values.part = self.data.container.trig_values.part.next();
            },
            Message::SwitchTrigonometricUnit => {
                self.data.container.trig_values.unit = self.data.container.trig_values.unit.next();
            },
            Message::Calculate => {
                let result = self.data.container.calculate(&self.data.current_page);

                if result.is_success() || self.data.container.pending.len() >= 10 {
                    self.data.container.pending.clear();
                }
                else if let DisplayableResult::None = result {
                    return none;
                }

                self.data.container.pending.push(result);
            },
            Message::CheckAuth => {
                let entered_master_key = self.data.container.cell_1.clone();

                // TODO: Release client api.

                self.data.settings.master_key = entered_master_key; // Moved here!
                self.write_settings();
            }
        };

        none
    }

    fn view(&self) -> ShortElement<'_> {
        get_scene(&self.data, &self.auth)
    }
    
    fn theme(&self) -> Self::Theme {
        self.data.settings.theme.get()  
    }
}
