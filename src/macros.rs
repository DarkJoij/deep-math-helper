#[macro_export]
macro_rules! if_debug {
    ($($body:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            $($body)*
        }
    }};
}

#[macro_export]
macro_rules! if_ultimate_version {
    ($($body:tt)*) => {{
        if cfg!(feature = "ultimate") {
            $($body)*
        }
    }};
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
        crate::instruments::DisplayableResult::Error(format_args!($($arg)*))
    };
}
