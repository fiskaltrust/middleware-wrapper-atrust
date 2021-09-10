#![doc(hidden)]

use chrono::{DateTime, Utc};
#[cfg(feature = "mocks")]
use fake::{Dummy, Fake};
#[cfg(feature = "mocks")]
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;
use restcrab::{crabs::reqwest::*, restcrab};

#[cfg(feature = "mocks")]
use crate::helpers::fakers::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Base64(String);

impl Base64 {
    pub fn from<T>(from: T) -> Self
    where
        T: AsRef<[u8]>,
    {
        Base64(base64::encode(from))
    }

    pub fn decode(&self) -> Result<Vec<u8>, base64::DecodeError> {
        base64::decode(&self.0)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct StartTransactionRequest {
    pub client_id: String,
    pub process_type: String,
    pub process_data_base64: Base64,
    #[cfg_attr(feature = "mocks", dummy(faker = "UuidFaker"))]
    pub queue_item_id: Uuid,
    pub is_retry: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct TseSignatureData {
    pub signature_counter: u64,
    pub signature_algorithm: String,
    pub signature_base64: Base64,
    pub public_key_base64: Base64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct StartTransactionResponse {
    pub transaction_number: u64,
    pub time_stamp: DateTime<Utc>,
    pub tse_serial_number_octet: String,
    pub client_id: String,
    pub signature_data: TseSignatureData,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct UpdateTransactionRequest {
    pub client_id: String,
    pub transaction_number: u64,
    pub process_type: String,
    pub process_data_base64: Base64,
    #[cfg_attr(feature = "mocks", dummy(faker = "UuidFaker"))]
    pub queue_item_id: Uuid,
    pub is_retry: bool,
}
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct UpdateTransactionResponse {
    pub transaction_number: u64,
    pub time_stamp: DateTime<Utc>,
    pub tse_serial_number_octet: String,
    pub client_id: String,
    pub process_type: String,
    pub process_data_base64: Base64,
    pub signature_data: TseSignatureData,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct FinishTransactionRequest {
    pub client_id: String,
    pub transaction_number: u64,
    pub process_type: String,
    pub process_data_base64: Base64,
    #[cfg_attr(feature = "mocks", dummy(faker = "UuidFaker"))]
    pub queue_item_id: Uuid,
    pub is_retry: bool,
}
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct FinishTransactionResponse {
    pub transaction_number: u64,
    pub start_transaction_time_stamp: DateTime<Utc>,
    pub time_stamp: DateTime<Utc>,
    pub tse_time_stamp_format: String,
    pub tse_serial_number_octet: String,
    pub client_id: String,
    pub process_type: String,
    pub process_data_base64: Base64,
    pub signature_data: TseSignatureData,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct TseInfo {
    pub max_number_of_clients: i64,
    pub current_number_of_clients: i64,
    pub current_client_ids: Vec<String>,
    pub max_number_of_started_transactions: i64,
    pub current_number_of_started_transactions: i64,
    pub current_started_transaction_numbers: Vec<u64>,
    pub max_number_of_signatures: i64,
    pub current_number_of_signatures: i64,
    pub max_log_memory_size: i64,
    pub current_log_memory_size: i64,
    pub current_state: TseStates,
    pub firmware_identification: String,
    pub certification_identification: String,
    pub signature_algorithm: String,
    pub log_time_format: String,
    pub serial_number_octet: String,
    pub public_key_base64: Base64,
    pub certificates_base64: Vec<Base64>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
pub enum TseStates {
    Uninitialized = 0,
    Initialized = 1,
    Terminated = 2,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct TseState {
    pub current_state: TseStates,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct RegisterClientIdRequest {
    pub client_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct RegisterClientIdResponse {
    pub client_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct UnregisterClientIdRequest {
    pub client_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct UnregisterClientIdResponse {
    pub client_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct StartExportSessionRequest {
    pub client_id: String,
    pub erase: bool,
}
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct StartExportSessionResponse {
    pub token_id: String,
    pub tse_serial_number_octet: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct StartExportSessionByTimeStampRequest {
    pub client_id: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct StartExportSessionByTransactionRequest {
    pub client_id: String,
    pub from: u64,
    pub to: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct ExportDataRequest {
    pub token_id: String,
    pub max_chunk_size: i32,
}
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct ExportDataResponse {
    pub token_id: String,
    pub tar_file_byte_chunk_base64: String,
    #[cfg_attr(feature = "mocks", dummy(faker = "fake::faker::boolean::en::Boolean(5)"))]
    pub tar_file_end_of_file: bool,
    pub total_tar_file_size_available: bool,
    pub total_tar_file_size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct EndExportSessionRequest {
    pub token_id: String,
    pub sha256_checksum_base64: String,
    pub erase: bool,
}
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct EndExportSessionResponse {
    pub token_id: String,
    #[cfg_attr(feature = "mocks", dummy(faker = "fake::faker::boolean::en::Boolean(100)"))]
    pub is_valid: bool,
    pub is_erased: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct ScuDeEchoRequest {
    pub message: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct ScuDeEchoResponse {
    pub message: String,
}

#[restcrab(crab = "Reqwest")]
pub trait IDeSscd {
    #[restcrab(method = "POST", uri = "/v1/starttransaction")]
    fn start_transaction(#[body] request: &StartTransactionRequest) -> StartTransactionResponse;
    
    #[restcrab(method = "POST", uri = "/v1/updatetransaction")]
    fn update_transaction(#[body] request: &UpdateTransactionRequest) -> UpdateTransactionResponse;
    
    #[restcrab(method = "POST", uri = "/v1/finishtransaction")]
    fn finish_transaction(#[body] request: &FinishTransactionRequest) -> FinishTransactionResponse;
    
    #[restcrab(method = "GET", uri = "/v1/tseinfo")]
    fn get_tse_info() -> TseInfo;
    
    #[restcrab(method = "POST", uri = "/v1/tsestate")]
    fn set_tse_state(#[body] state: &TseState) -> TseState;
    
    #[restcrab(method = "POST", uri = "/v1/registerclientid")]
    fn register_client_id(#[body] request: &RegisterClientIdRequest) -> RegisterClientIdResponse;
    
    #[restcrab(method = "POST", uri = "/v1/unregisterclientid")]
    fn unregister_client_id(#[body] request: &UnregisterClientIdRequest) -> UnregisterClientIdResponse;
    
    #[restcrab(method = "POST", uri = "/v1/executesettsetime")]
    fn execute_set_tse_time();
    
    #[restcrab(method = "POST", uri = "/v1/executeselftest")]
    fn execute_self_test();
    
    #[restcrab(method = "POST", uri = "/v1/startexportsession")]
    fn start_export_session(#[body] request: &StartExportSessionRequest) -> StartExportSessionResponse;
    
    #[restcrab(method = "POST", uri = "/v1/startexportsessionbytimestamp")]
    fn start_export_session_by_time_stamp(#[body] request: &StartExportSessionByTimeStampRequest) -> StartExportSessionResponse;
    
    #[restcrab(method = "POST", uri = "/v1/startexportsessionbytransaction")]
    fn start_export_session_by_transaction(#[body] request: &StartExportSessionByTransactionRequest) -> StartExportSessionResponse;

    #[restcrab(method = "POST", uri = "/v1/exportdata")]
    fn export_data(#[body] request: &ExportDataRequest) -> ExportDataResponse;
    
    #[restcrab(method = "POST", uri = "/v1/endexportsession")]
    fn end_export_session(#[body] request: &EndExportSessionRequest) -> EndExportSessionResponse;
    
    #[restcrab(method = "POST", uri = "/v1/echo")]
    fn echo(#[body] request: &ScuDeEchoRequest) -> ScuDeEchoResponse;
}
