use super::{Container, DisplayableResult};

impl Container {
    pub fn found_results(&self) -> DisplayableResult {
        let coefficients = match self.parse_in_vec::<f32>(self.cells()) {
            Ok(vector) => vector,
            Err(message) => return DisplayableResult::Text(message)
        };

        let a = coefficients[0];
        let b = coefficients[1];
        let c = coefficients[2];

        match Discriminant::from(b.powf(2.) - 4. * a * c) {
            Discriminant::Positive(number) => {
                let x1 = (-b + number.sqrt()) / (2. * a);
                let x2 = (-b - number.sqrt()) / (2. * a);

                DisplayableResult::Double(x1.to_string(), x2.to_string())
            },
            Discriminant::Zero => {
                let x = -b / (2. * a);
                DisplayableResult::Single(x.to_string())
            },
            Discriminant::Negative => {
                DisplayableResult::Text("Данное уравнение не имеет рациональных решений.".to_owned())
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
