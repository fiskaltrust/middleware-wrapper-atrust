use once_cell::sync::Lazy;
use thiserror::Error;

use crate::{config, helpers::Result, idesscd::*};

static CLIENT: Lazy<reqwest::blocking::Client> = Lazy::new(reqwest::blocking::Client::new);

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

pub struct Client {
    base_url: String,
}

impl Client {
    pub fn get(tse_id: String) -> Result<Client> {
        Ok(Client {
            base_url: config::get_tss(&tse_id).ok_or(Error::NoScuUrl(tse_id))?.scu_url,
        })
    }
}

impl IDeSscd for Client {
    fn start_transaction(&self, request: StartTransactionRequest) -> Result<StartTransactionResponse> {
        post!(url_version!(self.base_url, "starttransaction"), &request)
    }

    fn update_transaction(&self, request: UpdateTransactionRequest) -> Result<UpdateTransactionResponse> {
        post!(url_version!(self.base_url, "updatetransaction"), &request)
    }

    fn finish_transaction(&self, request: FinishTransactionRequest) -> Result<FinishTransactionResponse> {
        post!(url_version!(self.base_url, "finishtransaction"), &request)
    }

    fn get_tse_info(&self) -> Result<TseInfo> {
        get!(url_version!(self.base_url, "tseinfo"))
    }

    fn set_tse_state(&self, state: TseState) -> Result<TseState> {
        post!(url_version!(self.base_url, "tsestate"), &state)
    }

    fn register_client_id(&self, request: RegisterClientIdRequest) -> Result<RegisterClientIdResponse> {
        post!(url_version!(self.base_url, "registerclientid"), &request)
    }

    fn unregister_client_id(&self, request: UnregisterClientIdRequest) -> Result<UnregisterClientIdResponse> {
        post!(url_version!(self.base_url, "unregisterclientid"), &request)
    }

    fn execute_set_tse_time(&self) -> Result<()> {
        get!(url_version!(self.base_url, "executesettsetime"))
    }

    fn execute_self_test(&self) -> Result<()> {
        get!(url_version!(self.base_url, "executeselftest"))
    }

    fn start_export_session(&self, request: StartExportSessionRequest) -> Result<StartExportSessionResponse> {
        post!(url_version!(self.base_url, "startexportsession"), &request)
    }

    fn start_export_session_by_time_stamp(&self, request: StartExportSessionByTimeStampRequest) -> Result<StartExportSessionResponse> {
        post!(url_version!(self.base_url, "startexportsessionbytimestamp"), &request)
    }

    fn start_export_session_by_transaction(&self, request: StartExportSessionByTransactionRequest) -> Result<StartExportSessionResponse> {
        post!(url_version!(self.base_url, "startexportsessionbytransaction"), &request)
    }

    fn export_data(&self, request: ExportDataRequest) -> Result<ExportDataResponse> {
        post!(url_version!(self.base_url, "exportdata"), &request)
    }

    fn end_export_session(&self, request: EndExportSessionRequest) -> Result<EndExportSessionResponse> {
        post!(url_version!(self.base_url, "endexportsession"), &request)
    }

    fn echo(&self, request: ScuDeEchoRequest) -> Result<ScuDeEchoResponse> {
        post!(url_version!(self.base_url, "echo"), &request)
    }
}
