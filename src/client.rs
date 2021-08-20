
use std::sync::Arc;

use log::{error, warn};
use once_cell::sync::Lazy;
use thiserror::Error;
use arc_swap::ArcSwap;

use crate::{config, idesscd::*, return_codes::ReturnCode};

fn set_client() -> reqwest::blocking::Client {
    #[derive(Debug, thiserror::Error)]
    enum Error {
        #[error("Unknown proxy scheme configured: {0}")]
        UnknownProxyScheme(String),
    }

    try_or_return! {
        || -> Result<reqwest::blocking::Client, Box<dyn std::error::Error>> {
            let mut builder = reqwest::blocking::Client::builder();

            let general_config = &*config::GENERAL_CONFIG.lock().map_err(Box::new)?;

            if let Some(http_proxy) = &general_config.http_proxy {
                let url = reqwest::Url::parse(http_proxy).map_err(Box::new)?;

                builder = match url.scheme() {
                    "http" => builder.proxy(reqwest::Proxy::http(http_proxy).map_err(Box::new)?),
                    "https" => builder.proxy(reqwest::Proxy::https(http_proxy).map_err(Box::new)?),
                    scheme => { return Err(Box::new(Error::UnknownProxyScheme(scheme.to_string()))) }
                };
            } else {
                return Ok(reqwest::blocking::Client::new());
            }

            Ok(builder.build().map_err(Box::new)?)
        },
        |err| {
            error!("{}", err);
            warn!("No proxy used");
            reqwest::blocking::Client::new()
        }
    }
}

// CLARIFY: using an `ArcSwap` here allows for simultaieous calls to the fiskaltrust.Middleware. If this is not safe we should use a Mutex for synchronization
static CLIENT: Lazy<ArcSwap<reqwest::blocking::Client>> = Lazy::new(|| ArcSwap::new(Arc::new(set_client())));

pub fn reset_client() {
    CLIENT.store(Arc::new(set_client()));
}

const URL_VERSION: &str = "v1";
macro_rules! url_version {
    ($base_url:expr, $path:expr) => {
        format!("{}/{}/{}", $base_url, URL_VERSION, $path)
    };
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Request could not be sent: {source}")]
    RequestFailed {
        #[from]
        source: reqwest::Error,
    },

    #[error("Response statuscode did not indicate success: {0}")]
    Unsuccessful(reqwest::StatusCode),

    #[error("scu_url not found for tse: {0}")]
    NoScuUrl(String),
}

impl From<Error> for ReturnCode {
    fn from(err: Error) -> Self {
        match err {
            Error::RequestFailed { source: _ } => ReturnCode::SeCommunicationFailed,
            Error::Unsuccessful(_) => ReturnCode::Unknown,
            Error::NoScuUrl(_) => ReturnCode::InvalidConfig,
        }
    }
}

macro_rules! process_response {
    ($response:ident) => {{
        if !$response.status().is_success() {
            return Err(Error::Unsuccessful($response.status()).into());
        }

        Ok($response.json()?)
    }};
}

macro_rules! post {
    ($url:expr, $body:expr) => {{
        let response = CLIENT.load().post($url).json($body).send().map_err(|source| Error::RequestFailed { source })?;

        process_response!(response)
    }};
}

macro_rules! get {
    ($url:expr) => {{
        let response = CLIENT.load().get($url).send().map_err(|source| Error::RequestFailed { source })?;

        process_response!(response)
    }};
}

pub struct Client {
    base_url: String,
}

impl Client {
    pub fn get(tse_id: String) -> Result<Client, Error> {
        Ok(Client {
            base_url: config::get_tss(&tse_id).ok_or(Error::NoScuUrl(tse_id))?.scu_url,
        })
    }
}

impl IDeSscd for Client {
    type Error = Error;

    fn start_transaction(&self, request: &StartTransactionRequest) -> Result<StartTransactionResponse, Self::Error> {
        post!(url_version!(self.base_url, "starttransaction"), &request)
    }

    fn update_transaction(&self, request: &UpdateTransactionRequest) -> Result<UpdateTransactionResponse, Error> {
        post!(url_version!(self.base_url, "updatetransaction"), &request)
    }

    fn finish_transaction(&self, request: &FinishTransactionRequest) -> Result<FinishTransactionResponse, Error> {
        post!(url_version!(self.base_url, "finishtransaction"), &request)
    }

    fn get_tse_info(&self) -> Result<TseInfo, Error> {
        get!(url_version!(self.base_url, "tseinfo"))
    }

    fn set_tse_state(&self, state: &TseState) -> Result<TseState, Error> {
        post!(url_version!(self.base_url, "tsestate"), &state)
    }

    fn register_client_id(&self, request: &RegisterClientIdRequest) -> Result<RegisterClientIdResponse, Error> {
        post!(url_version!(self.base_url, "registerclientid"), &request)
    }

    fn unregister_client_id(&self, request: &UnregisterClientIdRequest) -> Result<UnregisterClientIdResponse, Error> {
        post!(url_version!(self.base_url, "unregisterclientid"), &request)
    }

    fn execute_set_tse_time(&self) -> Result<(), Error> {
        get!(url_version!(self.base_url, "executesettsetime"))
    }

    fn execute_self_test(&self) -> Result<(), Error> {
        get!(url_version!(self.base_url, "executeselftest"))
    }

    fn start_export_session(&self, request: &StartExportSessionRequest) -> Result<StartExportSessionResponse, Error> {
        post!(url_version!(self.base_url, "startexportsession"), &request)
    }

    fn start_export_session_by_time_stamp(&self, request: &StartExportSessionByTimeStampRequest) -> Result<StartExportSessionResponse, Error> {
        post!(url_version!(self.base_url, "startexportsessionbytimestamp"), &request)
    }

    fn start_export_session_by_transaction(&self, request: &StartExportSessionByTransactionRequest) -> Result<StartExportSessionResponse, Error> {
        post!(url_version!(self.base_url, "startexportsessionbytransaction"), &request)
    }

    fn export_data(&self, request: &ExportDataRequest) -> Result<ExportDataResponse, Error> {
        post!(url_version!(self.base_url, "exportdata"), &request)
    }

    fn end_export_session(&self, request: &EndExportSessionRequest) -> Result<EndExportSessionResponse, Error> {
        post!(url_version!(self.base_url, "endexportsession"), &request)
    }

    fn echo(&self, request: &ScuDeEchoRequest) -> Result<ScuDeEchoResponse, Error> {
        post!(url_version!(self.base_url, "echo"), &request)
    }
}
