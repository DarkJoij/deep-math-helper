use iced::{Element, Renderer, Theme};

#[derive(Default)]
pub struct QuadraticEquationsContainer {
    pub a: String,
    pub b: String,
    pub c: String
}

#[derive(Default)]
pub struct DataStore {
    pub query: String,
    pub current_page: Page,

    pub qe_container: QuadraticEquationsContainer
}

pub type ShortElement<'a> = Element<'a, Message, Renderer<Theme>>;

#[derive(Clone, Debug)]
pub enum Message {
    SetPage(Page),

    UpdateQEA(String),
    UpdateQEB(String),
    UpdateQEC(String),

    Calculate
}

#[derive(Clone, Debug, Default)]
pub enum Page {
    #[default]
    Selection,
    QuadraticEquations 
}
