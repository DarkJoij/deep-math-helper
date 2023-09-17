use std::fmt::{Debug, Display, Formatter, Result};

use iced::Theme;

#[derive(Copy, Default, PartialEq)]
pub enum Part {
    #[default]
    Fundamental,
    ArcFunctions
}

#[derive(Copy, Default, PartialEq)]
pub enum Unit {
    #[default]
    Radians,
    Degrees
}

pub trait PseudoIterator: Sized {
    fn all(&self) -> &[Self];
    fn next(&self) -> Self;
}

impl PseudoIterator for Part {
    fn all(&self) -> &[Self] {
        &[Self::Fundamental, Self::ArcFunctions]
    }

    fn next(&self) -> Self {
        let parts = self.all();
        let parts_iter = parts.iter();

        for (index, part) in parts_iter.enumerate() {
            if self == part {
                return match parts.get(index + 1) {
                    None => Self::Fundamental,
                    Some(part) => *part
                };
            }
        }

        Self::Fundamental
    }
}

impl PseudoIterator for Unit {
    fn all(&self) -> &[Self] {
        &[Self::Radians, Self::Degrees]
    }

    fn next(&self) -> Self {
        let parts = self.all();
        let parts_iter = parts.iter();

        for (index, part) in parts_iter.enumerate() {
            if self == part {
                return match parts.get(index + 1) {
                    None => Self::Radians,
                    Some(unit) => *unit
                };
            }
        }

        Self::Radians
    }
}

pub trait AsStr {
    fn as_str(&self) -> &str;
}

impl AsStr for Part {
    fn as_str(&self) -> &str {
        match self {
            Part::Fundamental => "Фундаментальные",
            Part::ArcFunctions => "Обратные функции"
        }
    }
}

impl AsStr for Unit {
    fn as_str(&self) -> &str {
        match self {
            Unit::Radians => "Радианы",
            Unit::Degrees => "Градусы"
        }
    }
}

pub struct Switcher<T: Clone> {
    one: T,
    two: T,
    what: bool
}

impl<T: Clone> Switcher<T> {
    pub fn new(one: T, two: T) -> Self {
        Switcher { one, two, what: true }
    }

    pub fn get(&self) -> T {
        match self.what {
            true => self.one.clone(),
            false => self.two.clone()
        }
    }

    pub fn switch(&mut self) -> T {
        self.what = !self.what;
        self.get()
    }
}

impl<T: Clone + Debug> Debug for Switcher<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let current = self.get();
        current.fmt(f)
    }
}

impl<T: Clone + Display> Display for Switcher<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let current = self.get();
        current.fmt(f)
    }
}

pub trait ThemeStrings {
    fn sys_name(&self) -> &str;
    fn display_name(&self) -> &str;
}

impl ThemeStrings for Switcher<Theme> {
    fn sys_name(&self) -> &str {
        if self.get() == Theme::Light { "Light" } else { "Dark" }
    }

    fn display_name(&self) -> &str {
        if self.get() == Theme::Light { "Тема: Светлая" } else { "Тема: Тёмная" }
    }
}
