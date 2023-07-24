mod gui;
mod instruments;

use crate::gui::app::DeepMathHelper;

use iced::{Application, Error, Settings};

fn main() -> Result<(), Error> {
    DeepMathHelper::run(Settings::default())
}
