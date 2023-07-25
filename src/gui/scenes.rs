use crate::gui::defaults::*;
use crate::gui::tools::{DataStore, Message, Page, ShortElement};
use crate::instruments::DisplayableResult;

use iced::alignment::{Horizontal, Vertical};
use iced::Length;
use iced::widget::{Column, Text};

pub fn get_scene<'a>(data: &DataStore) -> ShortElement<'a> {
    let back_button = get_default_button("Back", Message::SetPage(Page::Selection));
    let main = get_main_page(data);

    if let Page::Selection = data.current_page {
        return main.into();
    }

    main.push(back_button)
        .into()
}

fn get_main_page<'a>(data: &DataStore) -> Column<'a, Message> {
    let main_page = get_default_column();
    let current_page = match data.current_page {
        Page::Selection => get_selection_page(data),
        Page::QuadraticEquations => get_quadratic_equations_page(data)
    };

    let text_content = match &data.must_be_shown {
        DisplayableResult::None => return main_page.push(current_page),
        DisplayableResult::Text(message) => format!("Error:\n{message}"),
        DisplayableResult::Single(number) => format!("Answer:\nx = {number}."),
        DisplayableResult::Double(one, two) => format!("Answer:\nx1 = {one}, x2 = {two}.")
    };
    let showing_text = Text::new(text_content)
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
        .width(Length::Fill);

    main_page.push(current_page)
        .push(showing_text)
} 

fn get_selection_page<'a>(_data: &DataStore) -> ShortElement<'a> {
    let quadratic_equations_button = get_default_button(
        "Quadratic Equations", 
        Message::SetPage(Page::QuadraticEquations)
    );

    get_default_row()
        .push(quadratic_equations_button)
        .into()
}

fn get_quadratic_equations_page<'a>(data: &DataStore) -> ShortElement<'a> {
    let a = &data.qe_container.a;
    let b = &data.qe_container.b;
    let c = &data.qe_container.c;

    let coefficients = get_default_row()
        .push(get_default_text_input("Type A coefficient: ", a, Message::UpdateA))
        .push(get_default_text_input("Type B coefficient: ", b, Message::UpdateB))
        .push(get_default_text_input("Type C coefficient: ", c, Message::UpdateC));
    let calculate_button = get_default_button("Calculate", Message::Calculate);

    get_default_column()
        .push(coefficients)
        .push(calculate_button)
        .into()
}
