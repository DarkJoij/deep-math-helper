pub mod bases;
pub mod qe;
pub mod trigonometry;

use crate::res_err;
use crate::gui::tools::Page;
use crate::settings::Settings;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

pub enum Res<T> {
    Ok(T),
    Err(String)
}

#[derive(Default)]
pub enum DisplayableResult {
    #[default]
    None,
    Error(String),
    Success(String),
}

impl Display for DisplayableResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", match self {
            DisplayableResult::None => "None",
            DisplayableResult::Error(message) => message,
            DisplayableResult::Success(result) => result,
        })
    }
}

#[derive(Default)]
pub struct Container {
    pub cell_1: String,
    pub cell_2: String,
    pub cell_3: String,
    pub cell_4: String
}

impl Container {
    fn cells(&self) -> Vec<&str> {
        vec![&self.cell_1, &self.cell_2, &self.cell_3, &self.cell_4]
    }

    fn parse_in_vec_to<T: FromStr>(&self, to: usize) -> Res<Vec<T>> {
        self.parse_in_vec_sliced(0, to)
    }

    fn parse_in_vec_sliced<T: FromStr>(&self, from: usize, to: usize) -> Res<Vec<T>> {
        let cells = self.cells();
        let mut vector = Vec::with_capacity(to);

        for index in from..to {
            let literal = cells[index];

            if let Ok(number) = literal.parse::<T>() {
                vector.push(number);
                continue;
            }

            return res_err!("Введено некорректное число: '{}'.", literal);
        }

        Res::Ok(vector)
    }

    pub fn calculate(&self, data: &DataStore) -> DisplayableResult {
        match data.current_page {
            Page::Selection => DisplayableResult::None,
            Page::QuadraticEquations => self.found_results(),
            Page::BasesConverter => self.convert(),
            Page::Trigonometry => self.evaluate()
        }
    }
}

#[derive(Default)]
pub struct DataStore {
    pub query: String,
    pub current_page: Page,
    pub container: Container,
    pub pending: Vec<DisplayableResult>,
    pub settings: Settings
}
