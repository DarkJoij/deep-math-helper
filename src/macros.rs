#[macro_export]
macro_rules! if_debug {
    ($($body:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            $($body)*
        }
    }};
}
