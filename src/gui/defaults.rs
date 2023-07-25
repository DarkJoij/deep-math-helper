use crate::gui::tools::Message;

#[allow(unused_imports)]
use iced::{Alignment, Length, Padding };
use iced::widget::{Row, TextInput, Column, Button};

pub fn get_default_row<'a>() -> Row<'a, Message> {
    Row::new()
        .align_items(Alignment::Center)
}

pub fn get_default_column<'a>() -> Column<'a, Message> {
    Column::new()
        .align_items(Alignment::Center)
        .padding(Padding::new(35.)) // FIXME!..
}

pub fn get_default_text_input<'a, M>(placeholder: &str, value: &str, message: M) -> TextInput<'a, Message> 
where
    M: 'a + Fn(String) -> Message
{
    TextInput::new(placeholder, value)
        .padding(5)
        .on_input(message)
        .width(Length::Fill)
        
        // .size(20) // FIXME!
}

pub fn get_default_button<'a>(content: &'a str, message: Message) -> Button<'a, Message> {
    Button::new(content)
        // .padding(5) // FIXME!
        .on_press(message)
        .width(Length::Fill)
}
