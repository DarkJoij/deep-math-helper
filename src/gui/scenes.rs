use crate::gui::defaults::*;
use crate::gui::tools::{Message, Page, ShortElement};
use crate::instruments::{DataStore, DisplayableResult};

use iced::alignment::{Horizontal, Vertical};
use iced::{Length, Theme};
use iced::widget::{Button, Text, Column};

pub fn get_scene<'a>(data: &DataStore) -> ShortElement<'a> {
    let mut main = get_default_column();
    let mut current_page = match data.current_page {
        Page::Selection => get_selection_page(),
        Page::QuadraticEquations => get_quadratic_equations_page(data),
        Page::BasesConverter => get_bases_converter_page(data)
    };
    
    // From here...
    if data.current_page != Page::Selection {
        current_page = current_page.push(get_calc_button())
            .push(get_back_button())
    } else {
        main = main.push(get_theme_button(data));
    }
    // ... to here code must be refactored.

    let text_content = match &data.pending {
        DisplayableResult::None => return main.push(current_page)
            .into(),
        DisplayableResult::Error(message) => format!("Ошибка: {message}"),
        DisplayableResult::Success(result) => result.to_owned(),
    };
    let showing_text = Text::new(text_content)
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
        .width(Length::Fill);

    main.push(current_page)
        .push(showing_text)
        .into()
} 

fn get_selection_page<'a>() -> Column<'a, Message> {
    let quadratic_equations_button = get_default_button(
        "Квадратные уравнения", 
        Message::SetPage(Page::QuadraticEquations)
    );
    let bases_converter_button = get_default_button(
        "Конвертер СС", 
        Message::SetPage(Page::BasesConverter)
    );

    get_default_column()
        .push(quadratic_equations_button)
        .push(bases_converter_button)
}

fn get_quadratic_equations_page<'a>(data: &DataStore) -> Column<'a, Message> {
    let a = &data.container.cell_1;
    let b = &data.container.cell_2;
    let c = &data.container.cell_3;

    let coefficients = get_default_row()
        .push(get_default_text_input("Введите коэффициент a:", a, Message::UpdateCell1))
        .push(get_default_text_input("Введите коэффициент b:", b, Message::UpdateCell2))
        .push(get_default_text_input("Введите коэффициент c:", c, Message::UpdateCell3));

    get_default_column()
        .push(coefficients)
}

fn get_bases_converter_page<'a>(data: &DataStore) -> Column<'a, Message> {
    let number_literal = &data.container.cell_1;
    let from_base = &data.container.cell_2;
    let to_base = &data.container.cell_3;

    let number_and_bases = get_default_row()
        .push(get_default_text_input("Конвертировать число:", number_literal, Message::UpdateCell1))
        .push(get_default_text_input("Из СС:", from_base, Message::UpdateCell2))
        .push(get_default_text_input("В СС:", to_base, Message::UpdateCell3));

    get_default_column()
        .push(number_and_bases)
}

fn get_theme_button<'a>(data: &DataStore) -> Button<'a, Message> {
    let text = if data.settings.theme == Theme::Light { 
        "Тема: Светлая" 
    } else { 
        "Тема: Тёмная" 
    };

    Button::new(text)
        .on_press(Message::SwitchTheme)
}

fn get_calc_button<'a>() -> Button<'a, Message> {
    get_default_button("Вычислить", Message::Calculate)
}

fn get_back_button<'a>() -> Button<'a, Message> {
    get_default_button("Назад", Message::SetPage(Page::Selection))
}
