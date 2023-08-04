use crate::{displayable_err, displayable_ok, if_ultimate_version};
use super::{Container, DisplayableResult};

use num_base::Based;

impl Container {
    pub fn convert(&self) -> DisplayableResult {
        let mut cells = self.cells();
        cells.remove(0);

        let bases = match self.parse_in_vec::<usize>(cells) {
            Ok(vector) => vector,
            Err(message) => return displayable_err!(message)
        };
        if let Err(message) = check(&bases) {
            return displayable_err!(message);
        } 

        let number = Based::new(&self.cell_1, bases[0]);

        match number.to(bases[1]) {
            Ok(based) => displayable_ok!("Ответ: {}.", based.val),
            Err(_) => {
                if_ultimate_version! {
                    eprintln!("Failed to evaluate `number.to(bases[1])`.");
                }

                displayable_err!("Произошла неизвестная ошибка, пожалуйста, обратитесь к разработчику.")
            }
        }
    }
}

fn check(bases: &[usize]) -> Result<(), String> {
    for &base in bases {
        if base < 2 || base > 36 { // Is this normal: [`&1`]?
            return Err(format!("Некорректное основание системы счисления: '{}'.", base));
        }
    }

    Ok(())
}
