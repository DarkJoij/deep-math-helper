use crate::{displayable_err, if_ultimate_version};
use crate::gui::scenes::get_scene;
use crate::gui::tools::{Message, Page, ShortElement};
use crate::instruments::{Container, DataStore, DisplayableResult};
use crate::settings::write_file;

use iced::executor::Default;
use iced::{Application, Command, Theme};

pub struct DeepMathHelper {
    data: DataStore
}

impl Application for DeepMathHelper {
    type Executor = Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            DeepMathHelper { data: DataStore::default() },
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
        match message {
            Message::SwitchTheme => {
                self.data.settings.theme.switch();

                if let Err(error) = write_file(&self.data.settings) {
                    let message = "Failed to switch theme.";
                    
                    if_ultimate_version! {
                        eprintln!("{} {error}", &message);
                    }

                    self.data.pending.push(displayable_err!(message));
                }
            },
            Message::SetPage(page) => {
                if let Page::Selection = page {
                    self.data.pending.clear();
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
            Message::Calculate => {
                let result = self.data.container.calculate(&self.data);

                // This is weird mechanism.
                if let DisplayableResult::Success(..) = result {
                    self.data.pending.clear();
                }

                self.data.pending.push(result);
            }
        };

        Command::none()
    }

    fn view(&self) -> ShortElement<'_> {
        get_scene(&self.data)
    }
    
    fn theme(&self) -> Self::Theme {
        self.data.settings.theme.get()  
    }
}
