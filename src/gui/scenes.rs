use crate::gui::defaults::*;
use crate::gui::tools::{Message, Page, ShortElement};
use crate::instruments::{DataStore, DisplayableResult};
use crate::helpers::{AsStr, Part, ThemeStrings};

use iced::Alignment;
use iced::widget::{Button, Column};

pub fn get_scene(data: &DataStore) -> ShortElement {
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

    if data.container.pending.is_empty() {
        return main.into()
    }

    for part in &data.container.pending {
        let content = match part {
            DisplayableResult::None => continue,
            DisplayableResult::Error(message) => format!("Ошибка: {message}"),
            DisplayableResult::Success(result) => result.to_owned(),
        };

        main = main.push(get_default_text(content));
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

fn get_trigonometry_pages(data: &DataStore) -> Column<Message> {
    let part = data.container.part.as_str();
    let unit = data.container.unit.as_str();
    let literals = [
        "sin", "cos", "tan", "cot"
    ];
    let fields = [
        &data.container.cell_1, &data.container.cell_2,
        &data.container.cell_3, &data.container.cell_4
    ];
    let messages = [
        Message::UpdateCell1, Message::UpdateCell2,
        Message::UpdateCell3, Message::UpdateCell4
    ];

    let title = get_default_text("Тригонометрические функции:".to_owned());

    let switchers = get_default_row()
        .push(Button::new(part)
            .on_press(Message::SwitchTrigonometricPart))
        .push(Button::new(unit)
            .on_press(Message::SwitchTrigonometricUnit));

    let mut functions = Column::new()
        .align_items(Alignment::Center);

    for index in 0..4 {
        let mut literal = String::new();
        let field = fields[index];
        let message = messages[index];

        if let Part::ArcFunctions = data.container.part {
            literal.push('a');
        }

        literal.push_str(literals[index]);
        functions = functions.push(get_default_text_input(&literal, field, message));
    }

    get_default_column()
        .push(switchers)
        .push(title)
        .push(functions)
}

fn get_theme_button(data: &DataStore) -> Button<Message> {
    Button::new(data.settings.theme.display_name())
        .on_press(Message::SwitchTheme)
}

fn get_calc_button<'a>() -> Button<'a, Message> {
    get_default_button("Вычислить", Message::Calculate)
}

fn get_back_button<'a>() -> Button<'a, Message> {
    get_default_button("Назад", Message::SetPage(Page::Selection))
}
