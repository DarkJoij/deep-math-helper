use deep_math_helper::settings::*;

use iced::Theme;

#[test]
fn settings_structures_test() {
    let dirty = DirtySettings::default();
    let settings = Settings::from(&dirty);
    
    assert_eq!(dirty.theme, "Light".to_owned());
    assert_eq!(settings.theme, Theme::Light);
}

#[test]
fn io_functions_test() {
    let mut settings = match read_file() {
        Ok(dirty) => dirty,
        _ => panic!("failed to read settings file.")
    };

    settings.theme = Theme::Dark;
    match write_file(&settings) {
        Ok(result) => assert_eq!(result, ()),
        _ => panic!("failed to write into settings file.")
    }
}
