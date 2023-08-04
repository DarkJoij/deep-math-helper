use crate::if_ultimate_version;
use crate::gui::scenes::get_scene;
use crate::gui::tools::{Message, Page, ShortElement};
use crate::instruments::{Container, DataStore, DisplayableResult};
use crate::settings::write_file;

use iced::executor::Default as DefaultExecutor;
use iced::{Application, Command, Theme};

pub struct DeepMathHelper {
    data: DataStore
}

impl Application for DeepMathHelper {
    type Executor = DefaultExecutor;
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
        if_ultimate_version! {
            return "Deep Math Helper Ultimate Version".to_owned()
        }

        "Deep Math Helper".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SwitchTheme => {
                self.data.settings.theme = if self.data.settings.theme == Theme::Light { 
                    Theme::Dark 
                } else { 
                    Theme::Light 
                };

                if let Err(_) = write_file(&self.data.settings) {
                    let message = "Failed to swith theme.";
                    
                    if_ultimate_version! {
                        println!("{} {}.", message, "See [`switch_theme()`]");
                    }

                    self.data.pending = DisplayableResult::Error(message.to_owned());
                }
            },
            Message::SetPage(page) => {
                if let Page::Selection = page {
                    self.data.pending = DisplayableResult::None;
                    self.data.container = Container::default();
                }
                if_ultimate_version! { // Something like auto-completion???
                    if let Page::QuadraticEquations = page {
                        self.data.container.cell_3 = "0".to_owned(); 
                    }
                    if let Page::BasesConverter = page {
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
            Message::Calculate => {
                self.data.pending = self.data.container.calculate(&self.data)
            }
        };

        Command::none()
    }

    fn view(&self) -> ShortElement<'_> {
        get_scene(&self.data)
    }
    
    fn theme(&self) -> Self::Theme {
        self.data.settings.theme.clone()
    }
}
