#[macro_export]
macro_rules! ser_as_bytes {
    ($from:expr) => {
        ser::to_string(&$from.unwrap()).unwrap().as_bytes()
    };
}
