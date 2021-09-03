#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use log::error;
use num_enum::IntoPrimitive;

use crate::{
    atrustapi::return_codes::ReturnCode,
    client::{self, Client},
    helpers::ffi,
    idesscd::*,
};

const MAX_CHUNK_SIZE: i32 = 1000;

#[repr(u32)]
#[allow(dead_code)]
pub enum TssType {
    AsignTseOnline = 0,
    CryptoVision = 1,
}

#[derive(IntoPrimitive)]
#[repr(u32)]
pub enum UpdateVariants {
    Signed = 0,
    Unsigned = 1,
    SignedAndUnsigned = 2,
}

#[repr(i32)]
#[derive(IntoPrimitive)]
pub enum AuthenticationResult {
    Ok = 0,
    Failed = 1,
    PinIsBlocked = 2,
    UnknownUserId = 3,
}

#[repr(u32)]
#[derive(IntoPrimitive)]
pub enum UnblockResult {
    Ok = 0,
    Failed = 1,
    UnknownUserId = 2,
    Error = 3,
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn initializeDescriptionNotSet(description: *const i8, description_length: u32) -> i32 {
    log::info!("{}", "initializeDescriptionNotSet");

    initializeDescriptionNotSetWithTse(description, description_length, b"default".as_ptr() as *const i8, "default".len() as u32)
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn initializeDescriptionNotSetWithTse(description: *const i8, description_length: u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented.into()
}

#[no_mangle]
pub extern "C" fn initializeDescriptionSet() -> i32 {
    log::info!("{}", "initializeDescriptionSet");

    initializeDescriptionSetWithTse(b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub extern "C" fn initializeDescriptionSetWithTse(configEntry: *const i8, configEntryLength: u32) -> i32 {
    let tse_state = TseState { current_state: TseStates::Initialized };

    try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.set_tse_state(&tse_state), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
pub extern "C" fn updateTime(newDateTime: i64) -> i32 {
    log::info!("{}", "updateTime");

    updateTimeWithTse(newDateTime, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub extern "C" fn updateTimeWithTse(newDateTime: i64, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
pub extern "C" fn updateTimeWithTimeSync() -> i32 {
    log::info!("{}", "updateTimeWithTimeSync");

    updateTimeWithTimeSyncWithTse(b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub extern "C" fn updateTimeWithTimeSyncWithTse(configEntry: *const i8, configEntryLength: u32) -> i32 {
    try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.execute_set_tse_time(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
pub extern "C" fn disableSecureElement() -> i32 {
    log::info!("{}", "disableSecureElement");

    disableSecureElementWithTse(b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub extern "C" fn disableSecureElementWithTse(configEntry: *const i8, configEntryLength: u32) -> i32 {
    let tse_state = TseState { current_state: TseStates::Terminated };

    try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.set_tse_state(&tse_state), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
pub unsafe extern "C" fn startTransaction(
    clientId: *const i8,
    clientIdLength: u32,
    processData: *const u8,
    processDataLength: u32,
    processType: *const i8,
    processTypeLength: u32,
    additionalData: *const u8,
    additionalDataLength: u32,
    transactionNumber: *mut u32,
    logTime: *mut i64,
    serialNumber: *mut *mut u8,
    serialNumberLength: *mut u32,
    signatureCounter: *mut u32,
    signatureValue: *mut *mut u8,
    signatureValueLength: *mut u32,
) -> i32 {
    log::info!("{}", "startTransaction");

    startTransactionWithTse(
        clientId,
        clientIdLength,
        processData,
        processDataLength,
        processType,
        processTypeLength,
        additionalData,
        additionalDataLength,
        transactionNumber,
        logTime,
        serialNumber,
        serialNumberLength,
        signatureCounter,
        signatureValue,
        signatureValueLength,
        b"default".as_ptr() as *const i8,
        "default".len() as u32,
    )
}

#[no_mangle]
pub unsafe extern "C" fn startTransactionWithTse(
    clientId: *const i8,
    clientIdLength: u32,
    processData: *const u8,
    processDataLength: u32,
    processType: *const i8,
    processTypeLength: u32,
    additionalData: *const u8,
    additionalDataLength: u32,
    transactionNumber: *mut u32,
    logTime: *mut i64,
    serialNumber: *mut *mut u8,
    serialNumberLength: *mut u32,
    signatureCounter: *mut u32,
    signatureValue: *mut *mut u8,
    signatureValueLength: *mut u32,
    configEntry: *const i8,
    configEntryLength: u32,
) -> i32 {
    let start_transaction_request = StartTransactionRequest {
        client_id: ffi::from_cstr(clientId, clientIdLength),
        process_type: ffi::from_cstr(processType, processTypeLength),
        process_data_base64: Base64::from(ffi::from_cba(processData, processDataLength)),
        queue_item_id: uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, "fiskaltrust.eu".as_bytes()),
        is_retry: false,
    };

    let StartTransactionResponse {
        transaction_number,
        time_stamp,
        tse_serial_number_octet,
        client_id,
        signature_data,
    } = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.start_transaction(&start_transaction_request), |err: client::Error| {
        error!("{}", err);
        match err {
            client::Error::Unsuccessful(_) => ReturnCode::StartTransactionFailed,
            err => Into::<ReturnCode>::into(err),
        }
        .into()
    });

    ffi::set_u32_ptr(transactionNumber, transaction_number as u32);
    ffi::set_i64_ptr(logTime, time_stamp.timestamp());
    ffi::set_u32_ptr(signatureCounter, signature_data.signature_counter as u32);
    ffi::set_cstr(serialNumber, serialNumberLength, tse_serial_number_octet);
    let signature_value = try_or_return!(|| signature_data.signature_base64.decode(), |err| {
        error!("{}", err);
        ReturnCode::RetrieveLogMessageFailed.into()
    });
    ffi::set_byte_buf(signatureValue, signature_value.as_slice());
    ffi::set_u32_ptr(signatureValueLength, signature_value.len() as u32);

    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
pub unsafe extern "C" fn updateTransaction(
    clientId: *const i8,
    clientIdLength: u32,
    transactionNumber: u32,
    processData: *const u8,
    processDataLength: u32,
    processType: *const i8,
    processTypeLength: u32,
    logTime: *mut i64,
    signatureValue: *mut *mut u8,
    signatureValueLength: *mut u32,
    signatureCounter: *mut u32,
) -> i32 {
    log::info!("{}", "updateTransaction");

    updateTransactionWithTse(
        clientId,
        clientIdLength,
        transactionNumber,
        processData,
        processDataLength,
        processType,
        processTypeLength,
        logTime,
        signatureValue,
        signatureValueLength,
        signatureCounter,
        b"default".as_ptr() as *const i8,
        "default".len() as u32,
    )
}

#[no_mangle]
pub unsafe extern "C" fn updateTransactionWithTse(
    clientId: *const i8,
    clientIdLength: u32,
    transactionNumber: u32,
    processData: *const u8,
    processDataLength: u32,
    processType: *const i8,
    processTypeLength: u32,
    logTime: *mut i64,
    signatureValue: *mut *mut u8,
    signatureValueLength: *mut u32,
    signatureCounter: *mut u32,
    configEntry: *const i8,
    configEntryLength: u32,
) -> i32 {
    let update_transaction_request = UpdateTransactionRequest {
        client_id: ffi::from_cstr(clientId, clientIdLength),
        process_type: ffi::from_cstr(processType, processTypeLength),
        process_data_base64: Base64::from(ffi::from_cba(processData, processDataLength)),
        queue_item_id: uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, "fiskaltrust.eu".as_bytes()),
        is_retry: false,
        transaction_number: transactionNumber as u64,
    };

    let update_transaction_response = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.update_transaction(&update_transaction_request), |err: client::Error| {
        error!("{}", err);
        match err {
            client::Error::Unsuccessful(_) => ReturnCode::UpdateTransactionFailed,
            err => Into::<ReturnCode>::into(err),
        }
        .into()
    });

    ffi::set_i64_ptr(logTime, update_transaction_response.time_stamp.timestamp());
    ffi::set_u32_ptr(signatureCounter, update_transaction_response.signature_data.signature_counter as u32);
    let signature_value = try_or_return!(|| update_transaction_response.signature_data.signature_base64.decode(), |err| {
        error!("{}", err);
        ReturnCode::RetrieveLogMessageFailed.into()
    });
    ffi::set_byte_buf(signatureValue, signature_value.as_slice());
    ffi::set_u32_ptr(signatureValueLength, signature_value.len() as u32);

    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
pub unsafe extern "C" fn finishTransaction(
    clientId: *const i8,
    clientIdLength: u32,
    transactionNumber: u32,
    processData: *const u8,
    processDataLength: u32,
    processType: *const i8,
    processTypeLength: u32,
    additionalData: *const u8,
    additionalDataLength: u32,
    logTime: *mut i64,
    signatureValue: *mut *mut u8,
    signatureValueLength: *mut u32,
    signatureCounter: *mut u32,
) -> i32 {
    log::info!("{}", "finishTransaction");

    finishTransactionWithTse(
        clientId,
        clientIdLength,
        transactionNumber,
        processData,
        processDataLength,
        processType,
        processTypeLength,
        additionalData,
        additionalDataLength,
        logTime,
        signatureValue,
        signatureValueLength,
        signatureCounter,
        b"default".as_ptr() as *const i8,
        "default".len() as u32,
    )
}

#[no_mangle]
pub unsafe extern "C" fn finishTransactionWithTse(
    clientId: *const i8,
    clientIdLength: u32,
    transactionNumber: u32,
    processData: *const u8,
    processDataLength: u32,
    processType: *const i8,
    processTypeLength: u32,
    additionalData: *const u8,
    additionalDataLength: u32,
    logTime: *mut i64,
    signatureValue: *mut *mut u8,
    signatureValueLength: *mut u32,
    signatureCounter: *mut u32,
    configEntry: *const i8,
    configEntryLength: u32,
) -> i32 {
    let finish_transaction_request = FinishTransactionRequest {
        client_id: ffi::from_cstr(clientId, clientIdLength),
        process_type: ffi::from_cstr(processType, processTypeLength),
        process_data_base64: Base64::from(ffi::from_cba(processData, processDataLength)),
        queue_item_id: uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, "fiskaltrust.eu".as_bytes()),
        is_retry: false,
        transaction_number: transactionNumber as u64,
    };

    let finish_transaction_response = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.finish_transaction(&finish_transaction_request), |err: client::Error| {
        error!("{}", err);
        match err {
            client::Error::Unsuccessful(_) => ReturnCode::FinishTransactionFailed,
            err => Into::<ReturnCode>::into(err),
        }
        .into()
    });

    ffi::set_i64_ptr(logTime, finish_transaction_response.time_stamp.timestamp());
    ffi::set_u32_ptr(signatureCounter, finish_transaction_response.signature_data.signature_counter as u32);
    let signature_value = try_or_return!(|| finish_transaction_response.signature_data.signature_base64.decode(), |err| {
        error!("{}", err);
        ReturnCode::RetrieveLogMessageFailed.into()
    });
    ffi::set_byte_buf(signatureValue, signature_value.as_slice());
    ffi::set_u32_ptr(signatureValueLength, signature_value.len() as u32);

    ReturnCode::ExecutionOk.into()
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn exportDataFilteredByTransactionNumberAndClientId(transactionNumber: u32, clientId: *const i8, clientIdLength: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
    log::info!("{}", "exportDataFilteredByTransactionNumberAndClientId");

    exportDataFilteredByTransactionNumberAndClientIdWithTse(transactionNumber, clientId, clientIdLength, exportedData, exportedDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn exportDataFilteredByTransactionNumberAndClientIdWithTse(transactionNumber: u32, clientId: *const i8, clientIdLength: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented.into()
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn exportDataFilteredByTransactionNumber(transactionNumber: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
    log::info!("{}", "exportDataFilteredByTransactionNumber");

    exportDataFilteredByTransactionNumberWithTse(transactionNumber, exportedData, exportedDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn exportDataFilteredByTransactionNumberWithTse(transactionNumber: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented.into()
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn exportDataFilteredByTransactionNumberInterval(startTransactionNumber: u32, endTransactionNumber: u32, maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
    log::info!("{}", "exportDataFilteredByTransactionNumberInterval");

    exportDataFilteredByTransactionNumberIntervalWithTse(startTransactionNumber, endTransactionNumber, maximumNumberRecords, exportedData, exportedDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn exportDataFilteredByTransactionNumberIntervalWithTse(startTransactionNumber: u32, endTransactionNumber: u32, maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    exportDataFilteredByTransactionNumberIntervalAndClientIdWithTse(startTransactionNumber, endTransactionNumber, "".as_ptr() as *const i8, 0, maximumNumberRecords, exportedData, exportedDataLength, configEntry, configEntryLength)
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn exportDataFilteredByTransactionNumberIntervalAndClientId(startTransactionNumber: u32, endTransactionNumber: u32, clientId: *const i8, clientIdLength: u32, maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
    log::info!("{}", "exportDataFilteredByTransactionNumberIntervalAndClientId");

    exportDataFilteredByTransactionNumberIntervalAndClientIdWithTse(
        startTransactionNumber,
        endTransactionNumber,
        clientId,
        clientIdLength,
        maximumNumberRecords,
        exportedData,
        exportedDataLength,
        b"default".as_ptr() as *const i8,
        "default".len() as u32,
    )
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn exportDataFilteredByTransactionNumberIntervalAndClientIdWithTse(
    startTransactionNumber: u32,
    endTransactionNumber: u32,
    clientId: *const i8,
    clientIdLength: u32,
    maximumNumberRecords: u32,
    exportedData: *mut *mut u8,
    exportedDataLength: *mut u32,
    configEntry: *const i8,
    configEntryLength: u32,
) -> i32 {
    ReturnCode::NotImplemented.into()
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn exportDataFilteredByPeriodOfTime(startDate: i64, endDate: i64, maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
    log::info!("{}", "exportDataFilteredByPeriodOfTime");

    exportDataFilteredByPeriodOfTimeWithTse(startDate, endDate, maximumNumberRecords, exportedData, exportedDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn exportDataFilteredByPeriodOfTimeWithTse(startDate: i64, endDate: i64, maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    exportDataFilteredByPeriodOfTimeAndClientIdWithTse(startDate, endDate, "".as_ptr() as *const i8, 0, maximumNumberRecords, exportedData, exportedDataLength, configEntry, configEntryLength)
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn exportDataFilteredByPeriodOfTimeAndClientId(startDate: i64, endDate: i64, clientId: *const i8, clientIdLength: u32, maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
    log::info!("{}", "exportDataFilteredByPeriodOfTimeAndClientId");

    exportDataFilteredByPeriodOfTimeAndClientIdWithTse(startDate, endDate, clientId, clientIdLength, maximumNumberRecords, exportedData, exportedDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn exportDataFilteredByPeriodOfTimeAndClientIdWithTse(startDate: i64, endDate: i64, clientId: *const i8, clientIdLength: u32, maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented.into()
}

#[no_mangle]
pub unsafe extern "C" fn exportData(maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
    log::info!("{}", "exportData");

    exportDataWithTse(maximumNumberRecords, exportedData, exportedDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub unsafe extern "C" fn exportDataWithTse(maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    exportDataWithClientIdWithTse("".as_ptr() as *const i8, 0, exportedData, exportedDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub unsafe extern "C" fn exportDataWithClientId(clientId: *const i8, clientIdLength: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
    log::info!("{}", "exportData");

    exportDataWithClientIdWithTse(clientId, clientIdLength, exportedData, exportedDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

pub unsafe extern "C" fn exportDataWithClientIdWithTse(clientId: *const i8, clientIdLength: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    let client = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength)), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    let start_export_session_request = StartExportSessionRequest {
        client_id: ffi::from_cstr(clientId, clientIdLength),
        erase: false,
    };

    let start_export_session_response = try_or_return!(|| client.start_export_session(&start_export_session_request), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    let export_data_request = ExportDataRequest {
        token_id: start_export_session_response.token_id.clone(),
        max_chunk_size: MAX_CHUNK_SIZE,
    };

    let mut export_data: Vec<u8> = vec![];

    loop {
        let export_data_response = try_or_return!(|| client.export_data(&export_data_request), |err: client::Error| {
            error!("{}", err);
            Into::<ReturnCode>::into(err).into()
        });

        export_data.extend_from_slice(export_data_response.tar_file_byte_chunk_base64.as_bytes());

        if export_data_response.tar_file_end_of_file {
            break;
        }
    }

    let end_export_session_request = EndExportSessionRequest {
        token_id: start_export_session_response.token_id,
        sha256_checksum_base64: sha256::digest_bytes(export_data.as_slice()),
        erase: false,
    };

    let end_export_session_response = try_or_return!(|| client.end_export_session(&end_export_session_request), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    if !end_export_session_response.is_valid {
        return ReturnCode::Unknown.into();
    }

    ffi::set_byte_buf(exportedData, export_data.as_slice());
    ffi::set_u32_ptr(exportedDataLength, export_data.len() as u32);

    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
pub unsafe extern "C" fn exportCertificates(certificates: *mut *mut u8, certificatesLength: *mut u32) -> i32 {
    log::info!("{}", "exportCertificates");

    exportCertificatesWithTse(certificates, certificatesLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub unsafe extern "C" fn exportCertificatesWithTse(certificates: *mut *mut u8, certificatesLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    super::asigntse::at_getCertificateWithTse(certificates, certificatesLength, configEntry, configEntryLength)
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn restoreFromBackup(restoreData: *mut u8, restoreDataLength: u32) -> i32 {
    log::info!("{}", "restoreFromBackup");

    restoreFromBackupWithTse(restoreData, restoreDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn restoreFromBackupWithTse(restoreData: *mut u8, restoreDataLength: u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented.into()
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn readLogMessage(logMessage: *mut *mut u8, logMessageLength: *mut u32) -> i32 {
    log::info!("{}", "readLogMessage");

    readLogMessageWithTse(logMessage, logMessageLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn readLogMessageWithTse(logMessage: *mut *mut u8, logMessageLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented.into()
}

#[no_mangle]
pub unsafe extern "C" fn exportSerialNumbers(serialNumbers: *mut *mut u8, serialNumbersLength: *mut u32) -> i32 {
    log::info!("{}", "exportSerialNumbers");

    exportSerialNumbersWithTse(serialNumbers, serialNumbersLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub unsafe extern "C" fn exportSerialNumbersWithTse(serialNumbers: *mut *mut u8, serialNumbersLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    super::asigntse::at_getSerialNumberWithTse(serialNumbers, serialNumbersLength, configEntry, configEntryLength)
}

#[no_mangle]
pub unsafe extern "C" fn getMaxNumberOfClients(maxNumberClients: *mut u32) -> i32 {
    log::info!("{}", "getMaxNumberOfClients");

    getMaxNumberOfClientsWithTse(maxNumberClients, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub unsafe extern "C" fn getMaxNumberOfClientsWithTse(maxNumberClients: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.get_tse_info(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    ffi::set_u32_ptr(maxNumberClients, tse_info.max_number_of_clients as u32);
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
pub unsafe extern "C" fn getCurrentNumberOfClients(currentNumberClients: *mut u32) -> i32 {
    log::info!("{}", "getCurrentNumberOfClients");

    getCurrentNumberOfClientsWithTse(currentNumberClients, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub unsafe extern "C" fn getCurrentNumberOfClientsWithTse(currentNumberClients: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.get_tse_info(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    ffi::set_u32_ptr(currentNumberClients, tse_info.current_number_of_clients as u32);
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
pub unsafe extern "C" fn getMaxNumberOfTransactions(maxNumberTransactions: *mut u32) -> i32 {
    log::info!("{}", "getMaxNumberOfTransactions");

    getMaxNumberOfTransactionsWithTse(maxNumberTransactions, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub unsafe extern "C" fn getMaxNumberOfTransactionsWithTse(maxNumberTransactions: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.get_tse_info(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    ffi::set_u32_ptr(maxNumberTransactions, tse_info.max_number_of_started_transactions as u32);
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
pub unsafe extern "C" fn getCurrentNumberOfTransactions(currentNumberTransactions: *mut u32) -> i32 {
    log::info!("{}", "getCurrentNumberOfTransactions");

    getCurrentNumberOfTransactionsWithTse(currentNumberTransactions, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub unsafe extern "C" fn getCurrentNumberOfTransactionsWithTse(currentNumberTransactions: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.get_tse_info(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    ffi::set_u32_ptr(currentNumberTransactions, tse_info.current_number_of_started_transactions as u32);
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
pub extern "C" fn getSupportedTransactionUpdateVariants(supportedUpdateVariants: *mut UpdateVariants) -> i32 {
    log::info!("{}", "getSupportedTransactionUpdateVariants");

    getSupportedTransactionUpdateVariantsWithTse(supportedUpdateVariants, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub extern "C" fn getSupportedTransactionUpdateVariantsWithTse(supportedUpdateVariants: *mut UpdateVariants, configEntry: *const i8, configEntryLength: u32) -> i32 {
    unsafe { ffi::set_u32_ptr(supportedUpdateVariants as *mut u32, UpdateVariants::SignedAndUnsigned.into()) }
    ReturnCode::ExecutionOk.into()
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn deleteStoredData() -> i32 {
    log::info!("{}", "deleteStoredData");

    deleteStoredDataWithTse(b"default".as_ptr() as *const i8, "default".len() as u32)
}

/// `not implemented`
#[no_mangle]
#[cfg(feature = "not_implemented")]
#[cfg_attr(docsrs, doc(cfg(feature = "not_implemented")))]
pub extern "C" fn deleteStoredDataWithTse(configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented.into()
}

#[no_mangle]
pub extern "C" fn authenticateUser(userId: *const i8, userIdLength: u32, pin: *const u8, pinLength: u32, authenticationResult: *mut AuthenticationResult, remainingRetries: *mut i16) -> i32 {
    log::info!("{}", "authenticateUser");

    authenticateUserWithTse(userId, userIdLength, pin, pinLength, authenticationResult, remainingRetries, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub extern "C" fn authenticateUserWithTse(userId: *const i8, userIdLength: u32, pin: *const u8, pinLength: u32, authenticationResult: *mut AuthenticationResult, remainingRetries: *mut i16, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
pub extern "C" fn logOut(userId: *const i8, userIdLength: u32) -> i32 {
    log::info!("{}", "logOut");

    logOutWithTse(userId, userIdLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub extern "C" fn logOutWithTse(userId: *const i8, userIdLength: u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
pub extern "C" fn unblockUser(userId: *const i8, userIdLength: u32, puk: *const i8, pukLength: u32, newPin: *const i8, newPinLength: u32, unblockResult: *mut UnblockResult) -> i32 {
    log::info!("{}", "unblockUser");

    unblockUserWithTse(userId, userIdLength, puk, pukLength, newPin, newPinLength, unblockResult, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
pub extern "C" fn unblockUserWithTse(userId: *const i8, userIdLength: u32, puk: *const i8, pukLength: u32, newPin: *const i8, newPinLength: u32, unblockResult: *mut UnblockResult, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::ExecutionOk.into()
}
