use iced::Theme;
use serde::{Deserialize, Serialize};
use serde_json::{Error as SerdeError, from_str, to_string_pretty};

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::fs::{File, read_to_string, write};
use std::io::Write;

const SETTINGS_FILE_NAME: &str = "settings.json";

pub struct Switcher<T: Clone> {
    one: T,
    two: T,
    what: bool
}

impl<T: Clone> Switcher<T> {
    pub fn new(one: T, two: T) -> Self {
        Switcher { one, two, what: true }
    }

    pub fn get(&self) -> T {
        match self.what {
            true => self.one.clone(),
            false => self.two.clone()
        }
    }

    pub fn switch(&mut self) -> T {
        self.what = !self.what;
        self.get()
    }
}

impl<T: Clone + Debug> Debug for Switcher<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let current = self.get();
        current.fmt(f)
    }
}

impl<T: Clone + Display> Display for Switcher<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let current = self.get();
        current.fmt(f)
    }
}

pub trait ThemeStrings {
    fn sys_name(&self) -> &str;
    fn display_name(&self) -> &str;
}

impl ThemeStrings for Switcher<Theme> {
    fn sys_name(&self) -> &str {
        if self.get() == Theme::Light { "Light" } else { "Dark" }
    }

    fn display_name(&self) -> &str {
        if self.get() == Theme::Light { "Тема: Светлая" } else { "Тема: Тёмная" }
    }
}

pub struct Settings {
    pub theme: Switcher<Theme>
}

impl Settings {
    pub fn to_dirty(&self) -> DirtySettings {
        let sys_theme_name = self.theme.sys_name();

        DirtySettings { 
            theme: sys_theme_name.to_owned()
        }
    }
}

impl From<&DirtySettings> for Settings {
    fn from(value: &DirtySettings) -> Self {
        let mut theme = Switcher::new(Theme::Light, Theme::Dark);

        if value.theme.as_str() != "Light" {
            theme.switch();
        }

        Settings { theme }
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
    #[warn(unstable_features)]
    fn get_str_skip_serialization() -> &'static str {
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
        Err(_) => try_create_file()
    };

    let dirty: DirtySettings = from_str(&content)?;
    Ok(Settings::from(&dirty))
}

#[allow(unused_must_use)] // FIXME.
#[warn(unstable_features)]
pub fn write_file(settings: &Settings) -> Result<(), SerdeError> {
    let serialized = to_string_pretty(&settings.to_dirty())?; // Later might be switched to 4 spaces.
    write(SETTINGS_FILE_NAME, serialized); // let _ = ...
    Ok(())
}
