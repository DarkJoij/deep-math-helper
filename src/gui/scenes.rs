use crate::gui::defaults::*;
use crate::gui::tools::{DataStore, Message, Page, ShortElement};

use iced::Length;
use iced::widget::Button;

pub fn get_scene<'a>(data: &DataStore) -> ShortElement<'a> {
    match data.current_page {
        Page::Selection => get_selection_page(data),
        Page::QuadraticEquations => get_quadratic_equations_page(data)
    }
}

fn get_selection_page<'a>(_data: &DataStore) -> ShortElement<'a> {
    let quadratic_equations_button = Button::new("Квадратные уравнения")
        .on_press(Message::SetPage(Page::QuadraticEquations))
        .width(Length::Fill);

    get_default_row()
        .push(quadratic_equations_button)
        .into()
}

fn get_quadratic_equations_page<'a>(data: &DataStore) -> ShortElement<'a> {
    let a = &data.qe_container.a;
    let b = &data.qe_container.b;
    let c = &data.qe_container.c;

    let coefficients = get_default_row()
        .push(get_default_text_input("Type A coefficient: ", a, Message::UpdateQEA))
        .push(get_default_text_input("Type B coefficient: ", b, Message::UpdateQEB))
        .push(get_default_text_input("Type C coefficient: ", c, Message::UpdateQEC));
    let calculate_button = Button::new("Вычислить")
        .on_press(Message::Calculate)
        .width(Length::Fill);

    get_default_column()
        .push(coefficients)
        .push(calculate_button)
        .into()
}
