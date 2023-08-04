//! 32-bit floating-point numbers ([`f32`]), which are used for 
//! all calculations in that module, can later be replaced with 
//! 64-bit numbers ([`f64`]) to increase the accuracy of calculations.

use crate::{displayable_err, displayable_ok};
use super::{Container, DisplayableResult};

impl Container {
    pub fn found_results(&self) -> DisplayableResult {
        let coefficients = match self.parse_in_vec::<f32>(self.cells()) {
            Ok(vector) => vector,
            Err(message) => return displayable_err!(message)
        };

        let a = coefficients[0];
        let b = coefficients[1];
        let c = coefficients[2];

        match Discriminant::from(b.powf(2.) - 4. * a * c) {
            Discriminant::Positive(number) => {
                let x1 = (-b + number.sqrt()) / (2. * a);
                let x2 = (-b - number.sqrt()) / (2. * a);

                displayable_ok!("Ответ: x\u{2081} = {}; x\u{2082} = {}.", x1, x2)
            },
            Discriminant::Zero => {
                let x = -b / (2. * a);
                displayable_ok!("Ответ: x = {}.", x)
            },
            Discriminant::Negative => {
                displayable_err!("Данное уравнение не имеет рациональных решений.")
            }
        }
    }
}

enum Discriminant {
    Positive(f32),
    Zero,
    Negative
}

impl From<f32> for Discriminant {
    fn from(value: f32) -> Self {
        if value.is_sign_positive() { Self::Positive(value) }
        else if value == 0. { Self::Zero } 
        else { Self::Negative }
    } 
}
