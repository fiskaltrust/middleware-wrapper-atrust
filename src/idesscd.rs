use std::collections::HashMap;

use chrono::{DateTime, Utc};
#[cfg(feature = "mocks")]
use fake::{Dummy, Fake};
#[cfg(feature = "mocks")]
use mockall::{predicate::*, *};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "mocks")]
use crate::helpers::fakers::*;

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct TseSignatureData {
    pub signature_counter: u64,
    pub signature_algorithm: String,
    pub signature_base64: Base64,
    pub public_key_base64: Base64,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct StartTransactionResponse {
    pub transaction_number: u64,
    pub time_stamp: DateTime<Utc>, // CLARIFY: how is the datetime serialized by the fiskaltrust.Middleware. ISO time will work. For net-ticks we will probably have to implement our own serializer and deserializer https://serde.rs/custom-serialization.html.
    pub tse_serial_number_octet: String,
    pub client_id: String,
    pub signature_data: TseSignatureData,
}

#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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
    pub public_key_base64: Base64,
    pub certificates_base64: Vec<Base64>,
    #[cfg_attr(feature = "mocks", dummy(faker = "InfoFaker"))]
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
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct TseState {
    pub current_state: TseStates,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct RegisterClientIdRequest {
    pub client_id: String,
}
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct RegisterClientIdResponse {
    pub client_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct UnregisterClientIdRequest {
    pub client_id: String,
}
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct UnregisterClientIdResponse {
    pub client_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct StartExportSessionRequest {
    pub client_id: String,
    pub erase: bool,
}
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct StartExportSessionResponse {
    pub token_id: String,
    pub tse_serial_number_octet: String,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct StartExportSessionByTimeStampRequest {
    pub client_id: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct StartExportSessionByTransactionRequest {
    pub client_id: String,
    pub from: u64,
    pub to: u64,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct ExportDataRequest {
    pub token_id: String,
    pub max_chunk_size: i32,
}
#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct EndExportSessionRequest {
    pub token_id: String,
    pub sha256_checksum_base64: String,
    pub erase: bool,
}
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct EndExportSessionResponse {
    pub token_id: String,
    #[cfg_attr(feature = "mocks", dummy(faker = "fake::faker::boolean::en::Boolean(100)"))]
    pub is_valid: bool,
    pub is_erased: bool,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct ScuDeEchoRequest {
    pub message: String,
}
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "mocks", derive(Dummy))]
#[serde(rename_all = "PascalCase")]
pub struct ScuDeEchoResponse {
    pub message: String,
}

#[cfg(feature = "mocks")]
#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[cfg_attr(feature = "mocks", automock(type Error = Error;))]
pub trait IDeSscd {
    type Error: std::error::Error;

    fn start_transaction(&self, request: &StartTransactionRequest) -> Result<StartTransactionResponse, Self::Error>;
    fn update_transaction(&self, request: &UpdateTransactionRequest) -> Result<UpdateTransactionResponse, Self::Error>;
    fn finish_transaction(&self, request: &FinishTransactionRequest) -> Result<FinishTransactionResponse, Self::Error>;
    fn get_tse_info(&self) -> Result<TseInfo, Self::Error>;
    fn set_tse_state(&self, state: &TseState) -> Result<TseState, Self::Error>;
    fn register_client_id(&self, request: &RegisterClientIdRequest) -> Result<RegisterClientIdResponse, Self::Error>;
    fn unregister_client_id(&self, request: &UnregisterClientIdRequest) -> Result<UnregisterClientIdResponse, Self::Error>;
    fn execute_set_tse_time(&self) -> Result<(), Self::Error>;
    fn execute_self_test(&self) -> Result<(), Self::Error>;
    fn start_export_session(&self, request: &StartExportSessionRequest) -> Result<StartExportSessionResponse, Self::Error>;
    fn start_export_session_by_time_stamp(&self, request: &StartExportSessionByTimeStampRequest) -> Result<StartExportSessionResponse, Self::Error>;
    fn start_export_session_by_transaction(&self, request: &StartExportSessionByTransactionRequest) -> Result<StartExportSessionResponse, Self::Error>;
    fn export_data(&self, request: &ExportDataRequest) -> Result<ExportDataResponse, Self::Error>;
    fn end_export_session(&self, request: &EndExportSessionRequest) -> Result<EndExportSessionResponse, Self::Error>;
    fn echo(&self, request: &ScuDeEchoRequest) -> Result<ScuDeEchoResponse, Self::Error>;
}
