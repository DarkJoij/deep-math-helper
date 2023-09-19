use crate::gui::auth::MasterKey;
use crate::helpers::{Switcher, DisplayNames};

use iced::Theme;
use serde::{Deserialize, Serialize};
use serde_json::{Error as SerdeError, from_str, to_string_pretty};

use std::fs::{File, read_to_string, write};
use std::io::Write;

const SETTINGS_FILE_NAME: &str = "settings.json";

pub struct Settings {
    pub theme: Switcher<Theme>,
    pub master_key: MasterKey
}

impl Settings {
    pub fn to_dirty(&self) -> DirtySettings {
        let sys_theme_name = self.theme.sys_name();

        DirtySettings { 
            theme: sys_theme_name.to_owned(),
            master_key: self.master_key.clone()
        }
    }
}

impl From<&DirtySettings> for Settings {
    fn from(value: &DirtySettings) -> Self {
        let mut theme = Switcher::new(Theme::Light, Theme::Dark);

        if value.theme.as_str() != "Light" {
            theme.switch();
        }

        Settings {
            theme,
            master_key: value.master_key.clone()
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        match read_file() {
            Ok(settings) => settings,
            _ => Self::from(&DirtySettings::default())
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct DirtySettings {
    pub theme: String,
    pub master_key: String
}

impl DirtySettings {
    #[warn(unstable_features)]
    fn get_str_skip_serialization() -> &'static str {
        "{\n  \"theme\": \"Light\",\n  \"master_key\": \"unregistered\"\n}"
    }
}

impl Default for DirtySettings {
    fn default() -> Self {
        DirtySettings {
            theme: "Light".to_owned(),
            master_key: "unregistered".to_owned()
        }
    }
}

#[allow(unused_must_use)] // FIXME.
fn try_create_file() -> String {
    let mut file = File::create(SETTINGS_FILE_NAME)
        .expect("failed to create new file."); // This somehow must be covered.

    let content = DirtySettings::get_str_skip_serialization();
    file.write_all(content.as_bytes()); // let _ = ...

    content.to_owned()
}

#[warn(unstable_features)]
pub fn read_file() -> Result<Settings, SerdeError> {
    let content = match read_to_string(SETTINGS_FILE_NAME) {
        Ok(content) => content,
        _ => try_create_file()
    };

    let dirty: DirtySettings = from_str(&content)?;
    let settings = Settings::from(&dirty);

    drop(dirty); // TODO: Must be checked!
    Ok(settings)
}

#[allow(unused_must_use)] // FIXME.
#[warn(unstable_features)]
pub fn write_file(settings: &Settings) -> Result<(), SerdeError> {
    let serialized = to_string_pretty(&settings.to_dirty())?; // Later might be switched to 4 spaces.
    write(SETTINGS_FILE_NAME, serialized); // let _ = ...
    Ok(())
}
