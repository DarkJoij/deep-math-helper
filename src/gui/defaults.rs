use crate::gui::tools::Message;

use iced::{Alignment, Length, Padding};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Row, Text, TextInput, Column, Button};

pub fn get_default_row<'a>() -> Row<'a, Message> {
    Row::new()
        .align_items(Alignment::Center)
}

pub fn get_default_column<'a>() -> Column<'a, Message> {
    Column::new()
        .align_items(Alignment::Center)
        .padding(Padding::new(35.))
}

pub fn get_default_button<'a>(content: &'a str, message: Message) -> Button<'a, Message> {
    Button::new(content)
        .on_press(message)
        .width(Length::Fill)
}

pub fn get_default_text_input<'a, M>(placeholder: &str, value: &str, message: M) -> TextInput<'a, Message> 
where
    M: 'a + Fn(String) -> Message
{
    TextInput::new(placeholder, value)
        .padding(5)
        .on_input(message)
        .width(Length::Fill)        
}

pub fn get_default_text<'a>(content: String) -> Text<'a> {
    Text::new(content)
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
        .width(Length::Fill)
}
