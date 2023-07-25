use crate::instruments::DisplayableResult;

#[derive(Default)]
pub struct QuadraticEquationsContainer {
    pub a: String,
    pub b: String,
    pub c: String
}

impl QuadraticEquationsContainer {
    pub fn calculate(&self) -> DisplayableResult {
        let mut coefficients: Vec<f32> = Vec::with_capacity(3);

        for string in [&self.a, &self.b, &self.c] {
            if let Ok(number) = string.parse::<f32>() {
                coefficients.push(number);
                continue;
            }

            return DisplayableResult::Text(format!("Invalid number got: '{}'.", string));
        }

        let a = coefficients[0];
        let b = coefficients[1];
        let c = coefficients[2];

        match Discriminant::from(b.powf(2.) - 4. * a * c) {
            Discriminant::Positive(number) => {
                let x1 = (-b + number.sqrt()) / (2. * a);
                let x2 = (-b - number.sqrt()) / (2. * a);

                DisplayableResult::Double(x1, x2)
            },
            Discriminant::Zero => {
                let x = -b / (2. * a);
                DisplayableResult::Single(x)
            },
            Discriminant::Negative => {
                DisplayableResult::Text("This equation has no rational solutions.".to_owned())
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
