use crate::gui::scenes::get_scene;
use crate::gui::tools::{DataStore, Message, Page, ShortElement};

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
        String::from("Deep Math Helper")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SetPage(page) => { 
                self.data.current_page = page 
            },
            Message::UpdateA(a) => {
                self.data.qe_container.a = a;
            },
            Message::UpdateB(b) => {
                self.data.qe_container.b = b;
            },
            Message::UpdateC(c) => {
                self.data.qe_container.c = c;
            },
            Message::Calculate => {
                if let Page::QuadraticEquations = self.data.current_page {
                    let result = self.data.qe_container.calculate();
                    println!("{}", &result);
                    self.data.must_be_shown = result;
                }
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
