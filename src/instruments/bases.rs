use super::{Container, DisplayableResult};

use num_base::Based;

impl Container {
    pub fn convert(&self) -> DisplayableResult {
        let mut cells = self.cells();
        cells.remove(0);

        let bases = match self.parse_in_vec::<usize>(cells) {
            Ok(vector) => vector,
            Err(message) => return DisplayableResult::Text(message)
        };

        let converted = Based::new(&self.cell_1, bases[0])
            .to(bases[1]);

        match converted {
            Ok(based) => DisplayableResult::Single(based.val),
            Err(_) => DisplayableResult::Text(format!("Не удалось обработать число: '{}'.", self.cell_1))
        }
    }
}
