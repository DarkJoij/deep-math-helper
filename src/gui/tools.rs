use iced::{Element, Renderer, Theme};

pub type ShortElement<'a> = Element<'a, Message, Renderer<Theme>>;

#[derive(Clone, Debug)]
pub enum Message {
    SetPage(Page),
    UpdateA(String),
    UpdateB(String),
    UpdateC(String),
    Calculate
}

#[derive(Clone, Debug, Default)]
pub enum Page {
    #[default]
    Selection,
    QuadraticEquations 
}
