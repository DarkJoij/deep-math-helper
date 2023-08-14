use crate::{displayable_err, displayable_ok, if_ultimate_version, res_err};
use super::{Container, DisplayableResult, Res};

use num_base::Based;

impl Container {
    pub fn convert(&self) -> DisplayableResult {
        let mut cells = self.cells();
        cells.remove(0);

        let bases = match self.parse_in_vec::<usize>(cells) {
            Res::Ok(vector) => vector,
            Res::Err(message) => return displayable_err!(message)
        };
        if let Res::Err(message) = check(&bases) {
            return displayable_err!(message);
        } 

        let number = Based::new(&self.cell_1, bases[0]);

        match number.to(bases[1]) {
            Ok(based) => displayable_ok!("Ответ: {}.", based.val),
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
    for &base in bases {
        if base < 2 || base > 36 { // Is this normal: [`&1`]?
            return res_err!("Некорректное основание системы счисления: '{}'.", base);
        }
    }

    Res::Ok(())
}
