pub mod bases;
pub mod qe;

use crate::{if_ultimate_version, res_err};
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
            DisplayableResult::None => "None".to_owned(),
            DisplayableResult::Error(message) => message.to_owned(),
            DisplayableResult::Success(result) => result.to_owned(),
        })
    }
}

#[derive(Default)]
pub struct Container {
    pub cell_1: String,
    pub cell_2: String,
    pub cell_3: String
}

impl Container {
    fn cells(&self) -> Vec<&str> {
        vec![&self.cell_1, &self.cell_2, &self.cell_3]
    }

    fn parse_in_vec<T: FromStr>(&self, cells: Vec<&str>) -> Res<Vec<T>> {
        let mut vector = Vec::with_capacity(cells.len());

        for literal in cells {
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
            Page::Selection => {
                if_ultimate_version! {
                    eprintln!("Ignoring: It's start page, cannot return back.");
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
    pub pending: DisplayableResult,
    pub settings: Settings
}
