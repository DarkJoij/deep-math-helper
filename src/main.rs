mod gui;
mod instruments;
mod macros;

use crate::gui::app::DeepMathHelper;

use iced::{Application, Error, Settings as IcedSettings};
use iced::window::Settings as WindowSettings;

fn main() -> Result<(), Error> {
    DeepMathHelper::run(IcedSettings {
        window: WindowSettings {
            size: (500, 600),
            ..WindowSettings::default()
        },
        ..IcedSettings::default()
    })
}
