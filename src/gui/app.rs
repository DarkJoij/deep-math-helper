use crate::gui::scenes::get_scene;
use crate::gui::tools::{Message, Page, ShortElement};
use crate::instruments::{Container, DataStore, DisplayableResult};

use iced::executor::Default as DefaultExecutor;
use iced::{Application, Command, Theme};

pub struct DeepMathHelper {
    data: DataStore
}

impl DeepMathHelper {
    fn set_defaults(&mut self) {
        self.data.pending = DisplayableResult::None;
        self.data.container = Container::default();
    }
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
        "Deep Math Helper".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SetPage(page) => {
                if let Page::Selection = page {
                    self.set_defaults();
                }
                
                self.data.current_page = page;
            },
            Message::UpdateCell1(value) => {
                self.data.container.cell_1 = value;
            },
            Message::UpdateCell2(value) => {
                self.data.container.cell_2 = value;
            },
            Message::UpdateCell3(value) => {
                self.data.container.cell_3 = value;
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
        Theme::Dark
    }
}
