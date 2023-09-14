use crate::{displayable_err, displayable_ok, if_ultimate_version};
use super::{Container, DisplayableResult, Res};

use num_base::Based;

impl Container {
    pub fn convert(&self) -> DisplayableResult {
        let bases = match self.parse_in_vec_sliced::<usize>(1, 3) {
            Res::Ok(vector) => vector,
            Res::Err(message) => return displayable_err!(message)
        };
        if let Res::Err(message) = check(&bases) {
            return displayable_err!(message);
        }

        let number = Based::new(&self.cell_1, bases[0]);

        match number.to(bases[1]) {
            Ok(based) => displayable_ok!("Ответ: {}.", to_uppercase(based.val)),
            Err(_) => {
                if_ultimate_version! {
                    eprintln!("Failed to evaluate `number.to(bases[1])`.");
                }
                
                displayable_err!("Произошла неизвестная ошибка с кодом UE1, пожалуйста, обратитесь к разработчику.")
            }
        }
    }
}

fn check(bases: &[usize]) -> Res<()> {
    let biggest_base = if_ultimate_version! {{ 36 } else { 16 }};

    for &base in bases {
        let greater = base > biggest_base;

        if base < 2 || greater {
            let mut message = format!("Некорректное основание системы счисления: '{base}'.");

            if !cfg!(feature = "ultimate") && greater {
                message.push_str("\nПерейдите на Ultimate-версию программы с расширенными возможностями.");
            }

            return Res::Err(message);
        }
    }

    Res::Ok(())
}

fn to_uppercase(string: String) -> String {
    let mut buffer = String::new();

    for mut letter in string.chars() {
        if !letter.is_uppercase() {
            letter = letter.to_ascii_uppercase();
        }

        buffer.push(letter);
    }

    buffer
}
