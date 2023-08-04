use deep_math_helper::settings::*;

use iced::Theme;
use serde_json::Error;

#[test]
fn settings_structures_test() -> Result<(), Error> {
    let dirty = DirtySettings::default();
    let settings = Settings::from(&dirty);
    
    assert_eq!(dirty.theme, "Light".to_owned());
    assert_eq!(settings.theme, Theme::Light);
    
    Ok(())
}

#[test]
fn io_functions_test() -> Result<(), Error> {
    let mut settings = match read_file() {
        Ok(dirty) => dirty,
        Err(_) => panic!("failed to read settings file.")
    };

    settings.theme = Theme::Dark;
    match write_file(&settings) {
        Ok(result) => assert_eq!(result, ()),
        Err(_) => panic!("failed to write into settings file.")
    }

    Ok(())
}
