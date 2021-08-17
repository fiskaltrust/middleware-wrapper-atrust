#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use log::error;
use num_enum::IntoPrimitive;

use crate::{client::Client, helpers::ffi, idesscd::*, return_codes::ReturnCode};

#[repr(u32)]
pub enum TssType {
    AsignTseOnline = 0,
    CryptoVision = 1,
}

#[repr(u32)]
pub enum UpdateVariants {
    Update = 0,
    Unsigned = 1,
    AignedAndUnsigned = 2,
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

#[no_mangle]
extern "C" fn initializeDescriptionNotSet(description: *const i8, description_length: u32) -> i32 {
    initializeDescriptionNotSetWithTse(description, description_length, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn initializeDescriptionNotSetWithTse(description: *const i8, description_length: u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented as i32
}

#[no_mangle]
extern "C" fn initializeDescriptionSet() -> i32 {
    initializeDescriptionSetWithTse(b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn initializeDescriptionSetWithTse(configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented as i32
}

#[no_mangle]
extern "C" fn updateTime(newDateTime: i64) -> i32 {
    updateTimeWithTse(newDateTime, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn updateTimeWithTse(newDateTime: i64, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::ExecutionOk as i32 // CLARIFY
}

#[no_mangle]
extern "C" fn updateTimeWithTimeSync() -> i32 {
    updateTimeWithTimeSyncWithTse(b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn updateTimeWithTimeSyncWithTse(configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::ExecutionOk as i32 // CLARIFY
}

#[no_mangle]
extern "C" fn disableSecureElement() -> i32 {
    disableSecureElementWithTse(b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn disableSecureElementWithTse(configEntry: *const i8, configEntryLength: u32) -> i32 {
    todo!(); // CLARIFY
}

#[no_mangle]
extern "C" fn startTransaction(
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
extern "C" fn startTransactionWithTse(
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
        process_data_base64: Base64::from(unsafe { ffi::from_cba(processData, processDataLength) }),
        queue_item_id: uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, "fiskaltrust.eu".as_bytes()),
        is_retry: false,
    };

    let StartTransactionResponse {
        transaction_number,
        time_stamp,
        tse_serial_number_octet,
        client_id,
        signature_data,
    } = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.start_transaction(start_transaction_request), |err| {
        error!("{}", err);
        ReturnCode::StartTransactionFailed as i32
    });

    unsafe {
        ffi::set_u32_ptr(transactionNumber, transaction_number as u32);
        ffi::set_i64_ptr(logTime, time_stamp.timestamp());
        ffi::set_u32_ptr(signatureCounter, signature_data.signature_counter as u32);
        ffi::set_cstr(serialNumber, serialNumberLength, tse_serial_number_octet);
        let signature_value = try_or_return!(|| signature_data.signature_base64.decode(), |err| {
            error!("{}", err);
            ReturnCode::RetrieveLogMessageFailed as i32
        });
        ffi::set_byte_buf(signatureValue, signature_value.as_slice());
        ffi::set_u32_ptr(signatureValueLength, signature_value.len() as u32);
    }

    ReturnCode::ExecutionOk as i32
}

#[no_mangle]
extern "C" fn updateTransaction(
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
extern "C" fn updateTransactionWithTse(
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
        process_data_base64: Base64::from(unsafe { ffi::from_cba(processData, processDataLength) }),
        queue_item_id: uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, "fiskaltrust.eu".as_bytes()),
        is_retry: false,
        transaction_number: transactionNumber as u64,
    };

    let update_transaction_response = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.update_transaction(update_transaction_request), |err| {
        error!("{}", err);
        ReturnCode::UpdateTransactionFailed as i32
    });

    unsafe {
        ffi::set_i64_ptr(logTime, update_transaction_response.time_stamp.timestamp());
        ffi::set_u32_ptr(signatureCounter, update_transaction_response.signature_data.signature_counter as u32);
        let signature_value = try_or_return!(|| update_transaction_response.signature_data.signature_base64.decode(), |err| {
            error!("{}", err);
            ReturnCode::RetrieveLogMessageFailed as i32
        });
        ffi::set_byte_buf(signatureValue, signature_value.as_slice());
        ffi::set_u32_ptr(signatureValueLength, signature_value.len() as u32);
    }
    ReturnCode::ExecutionOk as i32
}

#[no_mangle]
extern "C" fn finishTransaction(
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
extern "C" fn finishTransactionWithTse(
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
        process_data_base64: Base64::from(unsafe { ffi::from_cba(processData, processDataLength) }),
        queue_item_id: uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, "fiskaltrust.eu".as_bytes()),
        is_retry: false,
        transaction_number: transactionNumber as u64,
    };

    let finish_transaction_response = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.finish_transaction(finish_transaction_request), |err| {
        error!("{}", err);
        ReturnCode::FinishTransactionFailed as i32
    });

    unsafe {
        ffi::set_i64_ptr(logTime, finish_transaction_response.time_stamp.timestamp());
        ffi::set_u32_ptr(signatureCounter, finish_transaction_response.signature_data.signature_counter as u32);
        let signature_value = try_or_return!(|| finish_transaction_response.signature_data.signature_base64.decode(), |err| {
            error!("{}", err);
            ReturnCode::RetrieveLogMessageFailed as i32
        });
        ffi::set_byte_buf(signatureValue, signature_value.as_slice());
        ffi::set_u32_ptr(signatureValueLength, signature_value.len() as u32);
    }

    ReturnCode::ExecutionOk as i32
}

#[no_mangle]
extern "C" fn exportDataFilteredByTransactionNumberAndClientId(transactionNumber: u32, clientId: *const i8, clientIdLength: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
    exportDataFilteredByTransactionNumberAndClientIdWithTse(transactionNumber, clientId, clientIdLength, exportedData, exportedDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn exportDataFilteredByTransactionNumberAndClientIdWithTse(transactionNumber: u32, clientId: *const i8, clientIdLength: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    todo!()
}

#[no_mangle]
extern "C" fn exportDataFilteredByTransactionNumber(transactionNumber: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
    exportDataFilteredByTransactionNumberWithTse(transactionNumber, exportedData, exportedDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn exportDataFilteredByTransactionNumberWithTse(transactionNumber: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    todo!()
}

#[no_mangle]
extern "C" fn exportDataFilteredByTransactionNumberInterval(startTransactionNumber: u32, endTransactionNumber: u32, maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
    exportDataFilteredByTransactionNumberIntervalWithTse(startTransactionNumber, endTransactionNumber, maximumNumberRecords, exportedData, exportedDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn exportDataFilteredByTransactionNumberIntervalWithTse(startTransactionNumber: u32, endTransactionNumber: u32, maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    todo!()
}

#[no_mangle]
extern "C" fn exportDataFilteredByTransactionNumberIntervalAndClientId(startTransactionNumber: u32, endTransactionNumber: u32, clientId: *const i8, clientIdLength: u32, maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
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

#[no_mangle]
extern "C" fn exportDataFilteredByTransactionNumberIntervalAndClientIdWithTse(
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
    todo!()
}

#[no_mangle]
extern "C" fn exportDataFilteredByPeriodOfTime(startDate: i64, endDate: i64, maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
    exportDataFilteredByPeriodOfTimeWithTse(startDate, endDate, maximumNumberRecords, exportedData, exportedDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn exportDataFilteredByPeriodOfTimeWithTse(startDate: i64, endDate: i64, maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    todo!()
}

#[no_mangle]
extern "C" fn exportDataFilteredByPeriodOfTimeAndClientId(startDate: i64, endDate: i64, clientId: *const i8, clientIdLength: u32, maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
    exportDataFilteredByPeriodOfTimeAndClientIdWithTse(startDate, endDate, clientId, clientIdLength, maximumNumberRecords, exportedData, exportedDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn exportDataFilteredByPeriodOfTimeAndClientIdWithTse(startDate: i64, endDate: i64, clientId: *const i8, clientIdLength: u32, maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    todo!()
}

#[no_mangle]
extern "C" fn exportData(maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32) -> i32 {
    exportDataWithTse(maximumNumberRecords, exportedData, exportedDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn exportDataWithTse(maximumNumberRecords: u32, exportedData: *mut *mut u8, exportedDataLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    todo!()
}

#[no_mangle]
extern "C" fn exportCertificates(certificates: *mut *mut u8, certificatesLength: *mut u32) -> i32 {
    exportCertificatesWithTse(certificates, certificatesLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn exportCertificatesWithTse(certificates: *mut *mut u8, certificatesLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    todo!()
}

#[no_mangle]
extern "C" fn restoreFromBackup(restoreData: *mut u8, restoreDataLength: u32) -> i32 {
    restoreFromBackupWithTse(restoreData, restoreDataLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn restoreFromBackupWithTse(restoreData: *mut u8, restoreDataLength: u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented as i32
}

#[no_mangle]
extern "C" fn readLogMessage(logMessage: *mut *mut u8, logMessageLength: *mut u32) -> i32 {
    readLogMessageWithTse(logMessage, logMessageLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn readLogMessageWithTse(logMessage: *mut *mut u8, logMessageLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    todo!()
}

#[no_mangle]
extern "C" fn exportSerialNumbers(serialNumbers: *mut *mut u8, serialNumbersLength: *mut u32) -> i32 {
    exportSerialNumbersWithTse(serialNumbers, serialNumbersLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn exportSerialNumbersWithTse(serialNumbers: *mut *mut u8, serialNumbersLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    todo!()
}

#[no_mangle]
extern "C" fn getMaxNumberOfClients(maxNumberClients: *mut u32) -> i32 {
    unsafe { getMaxNumberOfClientsWithTse(maxNumberClients, b"default".as_ptr() as *const i8, "default".len() as u32) }
}

#[no_mangle]
pub unsafe extern "C" fn getMaxNumberOfClientsWithTse(maxNumberClients: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.get_tse_info(), |err| {
        error!("{}", err);
        ReturnCode::Unknown as i32
    });

    ffi::set_u32_ptr(maxNumberClients, tse_info.max_number_of_clients as u32);
    ReturnCode::ExecutionOk as i32
}

#[no_mangle]
extern "C" fn getCurrentNumberOfClients(currentNumberClients: *mut u32) -> i32 {
    getCurrentNumberOfClientsWithTse(currentNumberClients, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn getCurrentNumberOfClientsWithTse(currentNumberClients: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.get_tse_info(), |err| {
        error!("{}", err);
        ReturnCode::Unknown as i32
    });

    unsafe { ffi::set_u32_ptr(currentNumberClients, tse_info.current_number_of_clients as u32) };
    ReturnCode::ExecutionOk as i32
}

#[no_mangle]
extern "C" fn getMaxNumberOfTransactions(maxNumberTransactions: *mut u32) -> i32 {
    getMaxNumberOfTransactionsWithTse(maxNumberTransactions, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn getMaxNumberOfTransactionsWithTse(maxNumberTransactions: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.get_tse_info(), |err| {
        error!("{}", err);
        ReturnCode::Unknown as i32
    });

    unsafe { ffi::set_u32_ptr(maxNumberTransactions, tse_info.max_number_of_started_transactions as u32) }; // CLARIFY
    ReturnCode::ExecutionOk as i32
}

#[no_mangle]
extern "C" fn getCurrentNumberOfTransactions(currentNumberTransactions: *mut u32) -> i32 {
    getCurrentNumberOfTransactionsWithTse(currentNumberTransactions, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn getCurrentNumberOfTransactionsWithTse(currentNumberTransactions: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.get_tse_info(), |err| {
        error!("{}", err);
        ReturnCode::Unknown as i32
    });

    unsafe { ffi::set_u32_ptr(currentNumberTransactions, tse_info.current_number_of_started_transactions as u32) }; // CLARIFY
    ReturnCode::ExecutionOk as i32
}

#[no_mangle]
extern "C" fn getSupportedTransactionUpdateVariants(supportedUpdateVariants: *mut UpdateVariants) -> i32 {
    getSupportedTransactionUpdateVariantsWithTse(supportedUpdateVariants, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn getSupportedTransactionUpdateVariantsWithTse(supportedUpdateVariants: *mut UpdateVariants, configEntry: *const i8, configEntryLength: u32) -> i32 {
    todo!()
}

#[no_mangle]
extern "C" fn deleteStoredData() -> i32 {
    deleteStoredDataWithTse(b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn deleteStoredDataWithTse(configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented as i32
}

#[no_mangle]
extern "C" fn authenticateUser(userId: *const i8, userIdLength: u32, pin: *const u8, pinLength: u32, authenticationResult: *mut AuthenticationResult, remainingRetries: *mut i16) -> i32 {
    authenticateUserWithTse(userId, userIdLength, pin, pinLength, authenticationResult, remainingRetries, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn authenticateUserWithTse(userId: *const i8, userIdLength: u32, pin: *const u8, pinLength: u32, authenticationResult: *mut AuthenticationResult, remainingRetries: *mut i16, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented as i32
}

#[no_mangle]
extern "C" fn logOut(userId: *const i8, userIdLength: u32) -> i32 {
    logOutWithTse(userId, userIdLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn logOutWithTse(userId: *const i8, userIdLength: u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented as i32
}

#[no_mangle]
extern "C" fn unblockUser(userId: *const i8, userIdLength: u32, puk: *const i8, pukLength: u32, newPin: *const i8, newPinLength: u32, unblockResult: *mut UnblockResult) -> i32 {
    unblockUserWithTse(userId, userIdLength, puk, pukLength, newPin, newPinLength, unblockResult, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn unblockUserWithTse(userId: *const i8, userIdLength: u32, puk: *const i8, pukLength: u32, newPin: *const i8, newPinLength: u32, unblockResult: *mut UnblockResult, configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented as i32
}
