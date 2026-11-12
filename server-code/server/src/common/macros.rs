#[macro_export]
macro_rules! format_signature_data {
    ($arg_0:expr, $($arg:expr),*) => {
        $( write!($arg_0, "{:?}", $arg); )*
        $arg_0
    };
}
