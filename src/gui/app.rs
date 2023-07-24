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
            Message::UpdateQEA(a) => {
                self.data.qe_container.a = a;
            },
            Message::UpdateQEB(b) => {
                self.data.qe_container.b = b;
            },
            Message::UpdateQEC(c) => {
                self.data.qe_container.c = c;
            },
            Message::Calculate => {
                match self.data.current_page {
                    Page::Selection => {},
                    Page::QuadraticEquations => {
                        
                        println!("Calculating...");
                    }
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
