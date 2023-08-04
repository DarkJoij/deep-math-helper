use iced::Theme;
use serde::{Deserialize, Serialize};
use serde_json::{Error as SerdeError, from_str, to_string_pretty};

use std::fs::{File, read_to_string, write};
use std::io::Write;

const SETTINGS_FILE_NAME: &str = "settings.json";

pub struct Settings {
    pub theme: Theme
}

impl From<&DirtySettings> for Settings {
    fn from(value: &DirtySettings) -> Self {
        use Theme::{Dark, Light};

        Settings { 
            theme: if value.theme.as_str() == "Light" { Light } else { Dark } 
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        if let Ok(settings) = read_file() { 
            settings 
        } else {
            Self::from(&DirtySettings::default())
        }  
    }
}

#[derive(Deserialize, Serialize)]
pub struct DirtySettings {
    pub theme: String
}

impl DirtySettings {
    fn get_str_skip_serialization<'a>() -> &'a str {
        "{\n  \"theme\": \"Light\"\n}"
    }
}

impl Default for DirtySettings {
    fn default() -> Self {
        DirtySettings { theme: "Light".to_owned() }
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
        Err(_) => try_create_file() // Might [`if let`] needed?
    };

    let dirty: DirtySettings = from_str(&content)?;
    Ok(Settings::from(&dirty))
}

#[allow(unused_must_use)] // FIXME.
#[warn(unstable_features)]
pub fn write_file(settings: &Settings) -> Result<(), SerdeError> {
    let dirty = DirtySettings { theme: format!("{:?}", settings.theme) };
    let serialized = to_string_pretty(&dirty)?; // Later might be switched to 4 spaces.

    write(SETTINGS_FILE_NAME, serialized); // let _ = ...
    Ok(())
}
