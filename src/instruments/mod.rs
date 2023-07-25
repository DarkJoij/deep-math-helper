pub mod qe;

use std::fmt::{Display, Formatter, Result};

#[allow(dead_code)]
pub enum IntermediateValue {
    Erroneous,
    Numeric(f32)
}

impl Display for IntermediateValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            IntermediateValue::Erroneous => "NaN".to_owned(),
            IntermediateValue::Numeric(number) => number.to_string()
        })
    }
}

#[derive(Default)]
pub enum DisplayableResult {
    #[default]
    None,
    Text(String),
    Single(f32),
    Double(f32, f32)
}

impl Display for DisplayableResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            DisplayableResult::None => "None".to_owned(),
            DisplayableResult::Text(string) => string.to_owned(),
            DisplayableResult::Single(number) => number.to_string(),
            DisplayableResult::Double(one, two) => format!("{}, {}", one, two)
        })
    }
}
