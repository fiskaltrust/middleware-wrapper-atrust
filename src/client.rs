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

static CLIENT: Lazy<ArcSwap<reqwest::blocking::Client>> = Lazy::new(|| ArcSwap::new(Arc::new(set_client())));

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

    #[error("Request could not be built: {source}")]
    BuildingRequestFailed {
        #[source]
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
            Error::NoScuUrl(_) => ReturnCode::InvalidConfig,
            _ => ReturnCode::Unknown,
        }
    }
}

macro_rules! process_response {
    ($response:expr) => {{
        if !$response.status().is_success() {
            return Err(Error::Unsuccessful($response.status()).into());
        }

        let response = $response.json()?;

        log::trace!("{:?}", response);

        Ok(response)
    }};
}

macro_rules! process_empty_response {
    ($response:expr) => {{
        if !$response.status().is_success() {
            return Err(Error::Unsuccessful($response.status()).into());
        }

        Ok(())
    }};
}

macro_rules! post {
    ($url:expr, $body:expr) => {{
        let client = CLIENT.load();
        let request = client.post($url).json($body).build().map_err(|source| Error::BuildingRequestFailed { source })?;
        
        log::trace!("{:?}", $body);
        log::trace!("{:?}", request);

        client.execute(request).map_err(|source| Error::RequestFailed { source })?
    }};

    ($url:expr) => {{
        let client = CLIENT.load();
        let request = client.post($url).header(reqwest::header::CONTENT_LENGTH, 0).build().map_err(|source| Error::BuildingRequestFailed { source })?;
        
        log::trace!("{:?}", request);

        client.execute(request).map_err(|source| Error::RequestFailed { source })?
    }};
}

macro_rules! get {
    ($url:expr) => {{
        let client = CLIENT.load();
        let request = client.get($url).build().map_err(|source| Error::BuildingRequestFailed { source })?;
        
        log::trace!("{:?}", request);

        client.execute(request).map_err(|source| Error::RequestFailed { source })?
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
        process_response!(post!(url_version!(self.base_url, "starttransaction"), &request))
    }

    fn update_transaction(&self, request: &UpdateTransactionRequest) -> Result<UpdateTransactionResponse, Error> {
        process_response!(post!(url_version!(self.base_url, "updatetransaction"), &request))
    }

    fn finish_transaction(&self, request: &FinishTransactionRequest) -> Result<FinishTransactionResponse, Error> {
        process_response!(post!(url_version!(self.base_url, "finishtransaction"), &request))
    }

    fn get_tse_info(&self) -> Result<TseInfo, Error> {
        process_response!(get!(url_version!(self.base_url, "tseinfo")))
    }

    fn set_tse_state(&self, state: &TseState) -> Result<TseState, Error> {
        process_response!(post!(url_version!(self.base_url, "tsestate"), &state))
    }

    fn register_client_id(&self, request: &RegisterClientIdRequest) -> Result<RegisterClientIdResponse, Error> {
        process_response!(post!(url_version!(self.base_url, "registerclientid"), &request))
    }

    fn unregister_client_id(&self, request: &UnregisterClientIdRequest) -> Result<UnregisterClientIdResponse, Error> {
        process_response!(post!(url_version!(self.base_url, "unregisterclientid"), &request))
    }

    fn execute_set_tse_time(&self) -> Result<(), Error> {
        process_empty_response!(post!(url_version!(self.base_url, "executesettsetime")))
    }

    fn execute_self_test(&self) -> Result<(), Error> {
        process_empty_response!(post!(url_version!(self.base_url, "executeselftest")))
    }

    fn start_export_session(&self, request: &StartExportSessionRequest) -> Result<StartExportSessionResponse, Error> {
        process_response!(post!(url_version!(self.base_url, "startexportsession"), &request))
    }

    fn start_export_session_by_time_stamp(&self, request: &StartExportSessionByTimeStampRequest) -> Result<StartExportSessionResponse, Error> {
        process_response!(post!(url_version!(self.base_url, "startexportsessionbytimestamp"), &request))
    }

    fn start_export_session_by_transaction(&self, request: &StartExportSessionByTransactionRequest) -> Result<StartExportSessionResponse, Error> {
        process_response!(post!(url_version!(self.base_url, "startexportsessionbytransaction"), &request))
    }

    fn export_data(&self, request: &ExportDataRequest) -> Result<ExportDataResponse, Error> {
        process_response!(post!(url_version!(self.base_url, "exportdata"), &request))
    }

    fn end_export_session(&self, request: &EndExportSessionRequest) -> Result<EndExportSessionResponse, Error> {
        process_response!(post!(url_version!(self.base_url, "endexportsession"), &request))
    }

    fn echo(&self, request: &ScuDeEchoRequest) -> Result<ScuDeEchoResponse, Error> {
        process_response!(post!(url_version!(self.base_url, "echo"), &request))
    }
}
