pub mod bases;
pub mod qe;

use crate::if_debug;
use crate::gui::tools::Page;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Default)]
pub enum DisplayableResult {
    #[default]
    None,
    Text(String),
    Single(String),
    Double(String, String)
}

impl Display for DisplayableResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", match self {
            DisplayableResult::None => "None".to_owned(),
            DisplayableResult::Text(string) => string.to_owned(),
            DisplayableResult::Single(string) => string.to_string(),
            DisplayableResult::Double(one, two) => format!("{}, {}", one, two)
        })
    }
}

#[derive(Debug, Default)]
pub struct Container {
    pub cell_1: String,
    pub cell_2: String,
    pub cell_3: String
}

impl Container {
    pub fn cells(&self) -> Vec<&str> {
        vec![&self.cell_1, &self.cell_2, &self.cell_3]
    }

    pub fn parse_in_vec<T: FromStr>(&self, cells: Vec<&str>) -> Result<Vec<T>, String> {
        let mut vector = Vec::with_capacity(cells.len());

        for literal in cells {
            if let Ok(number) = literal.parse::<T>() {
                vector.push(number);
                continue;
            }

            return Err(format!("Введено некорректное число: '{}'.", literal));
        }

        Ok(vector)
    }

    pub fn calculate(&self, data: &DataStore) -> DisplayableResult {
        match data.current_page {
            Page::Selection => {
                if_debug! {
                    println!("Ignoring: It's start page, cannot return back.");
                }

                DisplayableResult::None
            },
            Page::QuadraticEquations => self.found_results(),
            Page::BasesConverter => self.convert()
        }
    }
}

#[derive(Default)]
pub struct DataStore {
    pub query: String,
    pub current_page: Page,
    pub container: Container,
    pub pending: DisplayableResult
}
