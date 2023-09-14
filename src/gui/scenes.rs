use crate::gui::defaults::*;
use crate::gui::tools::{Message, Page, ShortElement};
use crate::instruments::{DataStore, DisplayableResult};
use crate::settings::ThemeStrings;

use iced::Alignment;
use iced::widget::{Button, Column};

pub fn get_scene<'a>(data: &'a DataStore) -> ShortElement<'a> {
    let mut main = get_default_column();
    let mut current_page = match data.current_page {
        Page::Selection => get_selection_page(),
        Page::QuadraticEquations => get_quadratic_equations_page(data),
        Page::BasesConverter => get_bases_converter_page(data),
        Page::Trigonometry => get_trigonometry_pages(data)
    };
    
    // From here...
    if data.current_page != Page::Selection {
        current_page = current_page.push(get_calc_button())
            .push(get_back_button());
    } else {
        main = main.push(get_theme_button(data));
    }
    // ... to here code must be refactored.
    main = main.push(current_page);

    if data.pending.len() == 0 {
        return main.into()
    }

    for part in &data.pending {
        let content = match part {
            DisplayableResult::None => continue,
            DisplayableResult::Error(message) => format!("Ошибка: {message}"),
            DisplayableResult::Success(result) => result.to_owned(),
        };

        main = main.push(get_default_text(content)); // FIXME: Must be dropped?!?
    }

    main.into()
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
    let trigonometry_button = get_default_button(
        "Тригонометрия", 
        Message::SetPage(Page::Trigonometry)
    );

    get_default_column()
        .push(quadratic_equations_button)
        .push(bases_converter_button)
        .push(trigonometry_button)
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

fn get_trigonometry_pages<'a>(data: &DataStore) -> Column<'a, Message> {
    let field_1 = &data.container.cell_1;
    let field_2 = &data.container.cell_2;
    let field_3 = &data.container.cell_3;
    let field_4 = &data.container.cell_4;

    let text = get_default_text("Тригонометрические функции:".to_owned());

    let functions = get_default_column()
        /*Column::new() // Must be without padding.
        .align_items(Alignment::Center) */
        .push(get_default_text_input("sin", field_1, Message::UpdateCell1))
        .push(get_default_text_input("cos", field_2, Message::UpdateCell3))
        .push(get_default_text_input("tan", field_3, Message::UpdateCell3))
        .push(get_default_text_input("cot", field_4, Message::UpdateCell4));

    get_default_column()
        .push(text)
        .push(functions)
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

fn get_theme_button<'a>(data: &'a DataStore) -> Button<'a, Message> {
    Button::new(data.settings.theme.display_name())
        .on_press(Message::SwitchTheme)
}

fn get_calc_button<'a>() -> Button<'a, Message> {
    get_default_button("Вычислить", Message::Calculate)
}

fn get_back_button<'a>() -> Button<'a, Message> {
    get_default_button("Назад", Message::SetPage(Page::Selection))
}
