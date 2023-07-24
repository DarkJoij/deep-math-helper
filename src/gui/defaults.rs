use crate::gui::tools::Message;

use iced::{Alignment, Length, Padding};
use iced::widget::{Row, TextInput, Column};

pub fn get_default_row<'a>() -> Row<'a, Message> {
    Row::new()
        .align_items(Alignment::Center)
        .padding(Padding::new(50.))
}

pub fn get_default_column<'a>() -> Column<'a, Message> {
    Column::new()
        .align_items(Alignment::Center)
        .padding(Padding::new(50.))
}

pub fn get_default_text_input<'a, F>(placeholder: &str, value: &str, message: F) -> TextInput<'a, Message>
where 
    F: 'a + Fn(String) -> Message
{
    TextInput::new(placeholder, value)
        .on_input(message)
        .width(Length::Fill)
}
