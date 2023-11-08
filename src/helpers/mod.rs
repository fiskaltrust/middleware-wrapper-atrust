#![doc(hidden)]
#[allow(clippy::missing_safety_doc)]
pub mod ffi;

#[cfg(feature = "mocks")]
pub mod fakers;

macro_rules! ok_or_return {
    ($wrapped:expr, $return:expr) => {{
        #[allow(clippy::redundant_closure_call)]
        match $wrapped {
            Ok(ok) => ok,
            Err(err) => return $return(err),
        }
    }};
}

macro_rules! some_or_return {
    ($wrapped:expr, $return:expr) => {
        match $wrapped {
            Some(some) => some,
            None => return $return,
        }
    };
}

macro_rules! try_or_return {
    ($wrapped:expr, $return:expr) => {{
        #[allow(clippy::redundant_closure_call)]
        match $wrapped() {
            Ok(ok) => ok,
            Err(err) => return $return(err),
        }
    }};
}
