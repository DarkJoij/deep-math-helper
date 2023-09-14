#[macro_export]
macro_rules! if_ultimate_version {
    ($ultimate_block:block else $else_block:block) => {{
        if cfg!(feature = "ultimate") {
            $ultimate_block
        } else {
            $else_block
        }
    }};
    ($($block:tt)*) => {{
        if cfg!(feature = "ultimate") {
            $($block)*
        }
    }};
}

#[warn(unstable_features)]
#[macro_export]
macro_rules! res_err {
    ($val:expr) => {
        crate::instruments::Res::Err($val.to_owned())
    };
    ($($arg:tt)*) => {
        crate::instruments::Res::Err(format!($($arg)*))
    };
}

#[warn(unstable_features)]
#[macro_export]
macro_rules! displayable_ok {
    ($val:expr) => {
        crate::instruments::DisplayableResult::Success($val.to_owned())
    };
    ($($arg:tt)*) => {
        crate::instruments::DisplayableResult::Success(format!($($arg)*))
    };
}

#[warn(unstable_features)]
#[macro_export]
macro_rules! displayable_err {
    ($val:expr) => {
        crate::instruments::DisplayableResult::Error($val.to_owned())
    };
    ($($arg:tt)*) => {
        crate::instruments::DisplayableResult::Error(format!($($arg)*))
    };
}

/// Untyped macro invoke.
#[warn(unstable_features)]
#[macro_export]
macro_rules! switcher {
    ($val_1:expr, $val_2:expr) => {
        crate::gui::tools::Switcher::new($val_1, $val_2)
    };
}
