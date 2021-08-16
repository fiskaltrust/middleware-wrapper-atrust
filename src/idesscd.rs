use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "mocks")]
use fake::{Dummy, Fake};
#[cfg(feature = "mocks")]
use mockall::{predicate::*, *};

use crate::helpers::Result;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartTransactionRequest {
    pub client_id: String,
    pub process_type: String,
    pub process_data_base64: String,
    pub queue_item_id: Uuid,
    pub is_retry: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TseSignatureData {
    pub signature_counter: u64,
    pub signature_algorithm: String,
    pub signature_base64: String,
    pub public_key_base64: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartTransactionResponse {
    pub transaction_number: u64,
    pub time_stamp: DateTime<Utc>,
    pub tse_serial_number_octet: String,
    pub client_id: String,
    pub signature_data: TseSignatureData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateTransactionRequest {
    pub client_id: String,
    pub transaction_number: u64,
    pub process_type: String,
    pub process_data_base64: String,
    pub queue_item_id: Uuid,
    pub is_retry: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateTransactionResponse {
    pub transaction_number: u64,
    pub time_stamp: DateTime<Utc>,
    pub tse_serial_number_octet: String,
    pub client_id: String,
    pub process_type: String,
    pub process_data_base64: String,
    pub signature_data: TseSignatureData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FinishTransactionRequest {
    pub client_id: String,
    pub transaction_number: u64,
    pub process_type: String,
    pub process_data_base64: String,
    pub queue_item_id: Uuid,
    pub is_retry: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FinishTransactionResponse {
    pub transaction_number: u64,
    pub start_transaction_time_stamp: DateTime<Utc>,
    pub time_stamp: DateTime<Utc>,
    pub tse_time_stamp_format: String,
    pub tse_serial_number_octet: String,
    pub client_id: String,
    pub process_type: String,
    pub process_data_base64: String,
    pub signature_data: TseSignatureData,
}

#[cfg(feature = "mocks")]
#[derive(Serialize, Deserialize)]
pub struct DummyInfo(HashMap<String, serde_json::Value>);

#[cfg(feature = "mocks")]
impl fake::Dummy<HashMap<String, serde_json::Value>> for DummyInfo {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &HashMap<String, serde_json::Value>, _: &mut R) -> Self {
        DummyInfo(HashMap::new())
    }

    fn dummy(_: &HashMap<String, serde_json::Value>) -> Self {
        DummyInfo(HashMap::new())
    }
}

#[derive(Serialize, Deserialize)]
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
    pub public_key_base64: String,
    pub certificates_base64: Vec<String>,
    #[cfg(feature = "mocks")]
    #[dummy(faker = "HashMap::new()")]
    pub info: DummyInfo,
    #[cfg(not(feature = "mocks"))]
    pub info: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
pub enum TseStates {
    Uninitialized = 0,
    Initialized = 1,
    Terminated = 2,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TseState {
    pub current_state: TseStates,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegisterClientIdRequest {
    pub client_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegisterClientIdResponse {
    pub client_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UnregisterClientIdRequest {
    pub client_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UnregisterClientIdResponse {
    pub client_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartExportSessionRequest {
    pub client_id: String,
    pub erase: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartExportSessionResponse {
    pub token_id: String,
    pub tse_serial_number_octet: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartExportSessionByTimeStampRequest {
    pub client_id: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartExportSessionByTransactionRequest {
    pub client_id: String,
    pub from: u64,
    pub to: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExportDataRequest {
    pub token_id: String,
    pub max_chunk_size: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExportDataResponse {
    pub token_id: String,
    pub tar_file_byte_chunk_base64: String,
    pub tar_file_end_of_file: bool,
    pub total_tar_file_size_available: bool,
    pub total_tar_file_size: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndExportSessionRequest {
    pub token_id: String,
    pub sha256_checksum_base64: String,
    pub erase: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndExportSessionResponse {
    pub token_id: String,
    pub is_valid: bool,
    pub is_erased: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScuDeEchoRequest {
    pub message: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScuDeEchoResponse {
    pub message: String,
}

#[cfg_attr(feature = "mocks", automock)]
pub trait IDeSscd {
    fn start_transaction(&self, request: StartTransactionRequest) -> Result<StartTransactionResponse>;
    fn update_transaction(&self, request: UpdateTransactionRequest) -> Result<UpdateTransactionResponse>;
    fn finish_transaction(&self, request: FinishTransactionRequest) -> Result<FinishTransactionResponse>;
    fn get_tse_info(&self) -> Result<TseInfo>;
    fn set_tse_state(&self, state: TseState) -> Result<TseState>;
    fn register_client_id(&self, request: RegisterClientIdRequest) -> Result<RegisterClientIdResponse>;
    fn unregister_client_id(&self, request: UnregisterClientIdRequest) -> Result<UnregisterClientIdResponse>;
    fn execute_set_tse_time(&self) -> Result<()>;
    fn execute_self_test(&self) -> Result<()>;
    fn start_export_session(&self, request: StartExportSessionRequest) -> Result<StartExportSessionResponse>;
    fn start_export_session_by_time_stamp(&self, request: StartExportSessionByTimeStampRequest) -> Result<StartExportSessionResponse>;
    fn start_export_session_by_transaction(&self, request: StartExportSessionByTransactionRequest) -> Result<StartExportSessionResponse>;
    fn export_data(&self, request: ExportDataRequest) -> Result<ExportDataResponse>;
    fn end_export_session(&self, request: EndExportSessionRequest) -> Result<EndExportSessionResponse>;
    fn echo(&self, request: ScuDeEchoRequest) -> Result<ScuDeEchoResponse>;
}
