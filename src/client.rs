use log::error;
use restcrab::{crabs::reqwest, Restcrab};
use thiserror::Error;

use crate::{atrustapi::return_codes::ReturnCode, config, idesscd::*};

const URL_VERSION: &str = "v1";

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    RequestFailed {
        #[from]
        source: reqwest::Error,
    },

    #[error(transparent)]
    CouldNotConstructUri {
        #[from]
        source: restcrab::http::uri::InvalidUri,
    },

    #[error("scu_url not found for tse: {0}")]
    NoScuUrl(String),
}

impl Error {
    pub fn reqwest_eror(&self) -> Option<&reqwest::Error> {
        if let Error::RequestFailed { source } = self {
            Some(source)
        } else {
            None
        }
    }
}

impl From<Error> for ReturnCode {
    fn from(err: Error) -> Self {
        match err {
            Error::RequestFailed { source: _ } => ReturnCode::SeCommunicationFailed,
            _ => ReturnCode::InvalidConfig,
        }
    }
}

pub struct Client();

impl Client {
    pub fn get(tse_id: String) -> Result<IDeSscdClient, Error> {
        Ok(IDeSscdClient::from_options(reqwest::Options {
            base_url: format!("{}/{}", config::get_tss(&tse_id).ok_or(Error::NoScuUrl(tse_id))?.scu_url, URL_VERSION).parse()?,
        })?)
    }
}

// impl IDeSscd for Client {
//     type Error = Error;

//     fn start_transaction(&self, request: &StartTransactionRequest) -> Result<StartTransactionResponse, Self::Error> {
//         process_response!(post!(url_version!(self.base_url, "starttransaction"), &request))
//     }

//     fn update_transaction(&self, request: &UpdateTransactionRequest) -> Result<UpdateTransactionResponse, Error> {
//         process_response!(post!(url_version!(self.base_url, "updatetransaction"), &request))
//     }

//     fn finish_transaction(&self, request: &FinishTransactionRequest) -> Result<FinishTransactionResponse, Error> {
//         process_response!(post!(url_version!(self.base_url, "finishtransaction"), &request))
//     }

//     fn get_tse_info(&self) -> Result<TseInfo, Error> {
//         process_response!(get!(url_version!(self.base_url, "tseinfo")))
//     }

//     fn set_tse_state(&self, state: &TseState) -> Result<TseState, Error> {
//         process_response!(post!(url_version!(self.base_url, "tsestate"), &state))
//     }

//     fn register_client_id(&self, request: &RegisterClientIdRequest) -> Result<RegisterClientIdResponse, Error> {
//         process_response!(post!(url_version!(self.base_url, "registerclientid"), &request))
//     }

//     fn unregister_client_id(&self, request: &UnregisterClientIdRequest) -> Result<UnregisterClientIdResponse, Error> {
//         process_response!(post!(url_version!(self.base_url, "unregisterclientid"), &request))
//     }

//     fn execute_set_tse_time(&self) -> Result<(), Error> {
//         process_empty_response!(post!(url_version!(self.base_url, "executesettsetime")))
//     }

//     fn execute_self_test(&self) -> Result<(), Error> {
//         process_empty_response!(post!(url_version!(self.base_url, "executeselftest")))
//     }

//     fn start_export_session(&self, request: &StartExportSessionRequest) -> Result<StartExportSessionResponse, Error> {
//         process_response!(post!(url_version!(self.base_url, "startexportsession"), &request))
//     }

//     fn start_export_session_by_time_stamp(&self, request: &StartExportSessionByTimeStampRequest) -> Result<StartExportSessionResponse, Error> {
//         process_response!(post!(url_version!(self.base_url, "startexportsessionbytimestamp"), &request))
//     }

//     fn start_export_session_by_transaction(&self, request: &StartExportSessionByTransactionRequest) -> Result<StartExportSessionResponse, Error> {
//         process_response!(post!(url_version!(self.base_url, "startexportsessionbytransaction"), &request))
//     }

//     fn export_data(&self, request: &ExportDataRequest) -> Result<ExportDataResponse, Error> {
//         process_response!(post!(url_version!(self.base_url, "exportdata"), &request))
//     }

//     fn end_export_session(&self, request: &EndExportSessionRequest) -> Result<EndExportSessionResponse, Error> {
//         process_response!(post!(url_version!(self.base_url, "endexportsession"), &request))
//     }

//     fn echo(&self, request: &ScuDeEchoRequest) -> Result<ScuDeEchoResponse, Error> {
//         process_response!(post!(url_version!(self.base_url, "echo"), &request))
//     }
// }
