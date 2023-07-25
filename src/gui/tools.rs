use crate::instruments::DisplayableResult;
use crate::instruments::qe::QuadraticEquationsContainer;

use iced::{Element, Renderer, Theme};

#[derive(Default)]
pub struct DataStore {
    pub query: String,
    pub current_page: Page,
    pub must_be_shown: DisplayableResult,

    pub qe_container: QuadraticEquationsContainer
}

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
