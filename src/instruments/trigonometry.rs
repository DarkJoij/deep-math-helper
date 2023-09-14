use crate::{displayable_err, displayable_ok, if_ultimate_version};
use super::{Container, DisplayableResult, Res};

impl Container {
    pub fn evaluate(&self) -> DisplayableResult {
        DisplayableResult::Success(self.cell_4.clone())
    }
}
