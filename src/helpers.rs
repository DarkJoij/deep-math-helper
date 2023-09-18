use std::fmt::{Debug, Display, Formatter, Result};

use iced::Theme;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Part {
    #[default]
    Fundamental,
    ArcFunctions
}

#[derive(Clone, Copy, Default, PartialEq)]
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
                    None => break,
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
                    None => break,
                    Some(unit) => *unit
                };
            }
        }

        Self::Radians
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

pub trait DisplayNames {
    fn sys_name(&self) -> &str {
        unimplemented!()
    }

    fn display_name(&self) -> &str {
        unimplemented!()
    }
}

impl DisplayNames for Part {
    fn display_name(&self) -> &str {
        match self {
            Part::Fundamental => "Фундаментальные",
            Part::ArcFunctions => "Обратные функции"
        }
    }
}

impl DisplayNames for Unit {
    fn display_name(&self) -> &str {
        match self {
            Unit::Radians => "Радианы",
            Unit::Degrees => "Градусы"
        }
    }
}

impl DisplayNames for Switcher<Theme> {
    fn sys_name(&self) -> &str {
        if self.get() == Theme::Light { "Light" } else { "Dark" }
    }

    fn display_name(&self) -> &str {
        if self.get() == Theme::Light { "Тема: Светлая" } else { "Тема: Тёмная" }
    }
}
