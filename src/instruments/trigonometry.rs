use crate::instruments::{DisplayableResult, Part, Unit};

use crate::{displayable_err, displayable_ok, res_err};
use super::{Container, Res};

use std::f64::consts::PI; // Must be replaced with [`FloatConst::PI()`].
use std::fmt::{Display, Formatter, Result};

type TFloatNumber = f64;

impl Container {
    pub fn evaluate(&self) -> Vec<DisplayableResult> {
        let cells = self.cells();
        let mut results = Vec::new();
        let funcs = Function::get_all(self.part);

        for index in 0..4 {
            let cell = cells[index];

            if !cell.is_empty() {
                let value = match TrigonometryValue::from(cell, self.unit) {
                    Res::Ok(value) => value,
                    Res::Err(message) => {
                        results.push(displayable_err!(message));
                        break;
                    }
                };

                let func = &funcs[index];
                let result = self.exec_func(&value, func);

                results.push(displayable_ok!("{func}({value}) = {}", TryRound(result)));
            }
        }

        results
    }

    fn exec_func(&self, value: &TrigonometryValue, function: &Function) -> TFloatNumber {
        let number = match value {
            TrigonometryValue::Radian(radian) => *radian,
            TrigonometryValue::Degree(degree) => *degree * PI / 180f64
        };

        match function {
            Function::Sin => number.sin(),
            Function::Cos => number.cos(),
            Function::Tan => number.tan(),
            Function::Cot => number.cos() / number.sin(),
            Function::ASin => number.asin(),
            Function::ACos => number.acos(),
            Function::ATan => number.atan(),
            Function::ACot => PI / 2f64 - number.atan(),
        }
    }
}

enum TrigonometryValue {
    Radian(TFloatNumber),
    Degree(TFloatNumber)
}

impl TrigonometryValue {
    pub fn from(literal: &str, unit: Unit) -> Res<Self> {
        let number = match literal.parse::<TFloatNumber>() {
            Ok(number) => number,
            _ => return res_err!("Введено некорректное число: '{}'.", &literal)
        };

        Res::Ok(match unit {
            Unit::Radians => Self::Radian(number),
            Unit::Degrees => Self::Degree(number)
        })
    }
}

impl Display for TrigonometryValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use TrigonometryValue::*;

        write!(f, "{}", match self {
            Radian(number) => number.to_string(),
            Degree(number) => format!("{}\u{00B0}", number)
        })
    }
}

/// A-functions maybe will be added only for Ultimate-version.
enum Function {
    Sin,
    Cos,
    Tan,
    Cot,
    ASin,
    ACos,
    ATan,
    ACot
}

impl Function {
    pub fn get_all(part: Part) -> [Function; 4] {
        use Function::*;

        match part {
            Part::Fundamental => [Sin, Cos, Tan, Cot],
            Part::ArcFunctions => [ASin, ACos, ATan, ACot]
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Function::*;

        write!(f, "{}", match self {
            Sin => "sin",
            Cos => "cos",
            Tan => "tan",
            Cot => "cot",
            ASin => "asin",
            ACos => "acos",
            ATan => "atan",
            ACot => "acot"
        })
    }
}

struct TryRound(TFloatNumber);

impl Display for TryRound {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let rounded_string = format!("{:.6}", self.0);
        let rounded = match rounded_string.parse::<TFloatNumber>() {
            Ok(number) => number,
            _ => return write!(f, "{}", self.0)
        };

        write!(f, "{}", if (self.0 - rounded).abs() < 0.000001 { rounded } else { self.0 })
    }
}
