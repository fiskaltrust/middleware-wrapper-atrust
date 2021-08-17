pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[allow(clippy::missing_safety_doc)]
pub mod ffi;

#[cfg(feature = "mocks")]
pub mod fakers;

macro_rules! ok_or_return {
    ($wrapped:expr, $return:expr) => {
        match $wrapped {
            Ok(ok) => ok,
            Err(err) => return $return(err),
        }
    };
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
    ($wrapped:expr, $return:expr) => {
        match $wrapped() {
            Ok(ok) => ok,
            Err(err) => return $return(err),
        }
    };
}

#[macro_use]
pub mod rest {
    macro_rules! call {
        ($response:ident) => {{
            if !$response.status().is_success() {
                return Err(Error::Unsuccessful($response.status()).into());
            }

            Ok($response.json()?)
        }};
    }

    macro_rules! post {
        ($url:expr, $body:expr) => {{
            let response = CLIENT.post($url).json($body).send().map_err(|source| Error::RequestFailed { source })?;

            call!(response)
        }};
    }

    macro_rules! get {
        ($url:expr) => {{
            let response = CLIENT.get($url).send().map_err(|source| Error::RequestFailed { source })?;

            call!(response)
        }};
    }
}
