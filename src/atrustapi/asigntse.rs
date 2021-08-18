#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use log::error;

use super::return_codes::ReturnCode;
use crate::{
    client::{self, Client},
    helpers::ffi,
    idesscd::*,
    logging,
};

#[repr(u32)]
pub enum LifecycleState {
    Unknown = 0,
    NotInitialized = 1,
    Active = 2,
    Suspended = 3,
    Disabled = 4,
}

#[no_mangle]
extern "C" fn at_getLifecycleState(state: *mut LifecycleState) -> i32 {
    at_getLifecycleStateWithTse(state, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_getLifecycleStateWithTse(state: *mut LifecycleState, tseId: *const i8, tseIdLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(tseId, tseIdLength))?.get_tse_info(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    let lifecycle_state: LifecycleState = match tse_info.current_state {
        TseStates::Uninitialized => LifecycleState::NotInitialized,
        TseStates::Initialized => LifecycleState::Active,
        TseStates::Terminated => LifecycleState::Disabled, // CLARIFY
    };

    unsafe { ffi::set_u32_ptr(state as *mut u32, tse_info.current_state as u32) };
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_unsuspendSecureElement() -> i32 {
    at_unsuspendSecureElementWithTse(b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_unsuspendSecureElementWithTse(tseId: *const i8, tseIdLength: u32) -> i32 {
    ReturnCode::ExecutionOk.into() // CLARIFY
}

#[no_mangle]
extern "C" fn at_suspendSecureElement() -> i32 {
    at_suspendSecureElementWithTse(b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_suspendSecureElementWithTse(tseId: *const i8, tseIdLength: u32) -> i32 {
    ReturnCode::ExecutionOk.into() // CLARIFY
}

#[no_mangle]
extern "C" fn at_getCertificate(cert: *mut *mut u8, certLength: *mut u32) -> i32 {
    at_getCertificateWithTse(cert, certLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_getCertificateWithTse(cert: *mut *mut u8, certLength: *mut u32, tseId: *const i8, tseIdLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(tseId, tseIdLength))?.get_tse_info(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    unsafe { ffi::set_cstr_array(cert, certLength, &tse_info.certificates_base64.into_iter().map(|c| String::from(c.as_str())).collect::<Vec<String>>()) };
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_getPublicKey(pubKey: *mut *mut u8, pubKeyLength: *mut u32) -> i32 {
    at_getPublicKeyWithTse(pubKey, pubKeyLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_getPublicKeyWithTse(pubKey: *mut *mut u8, pubKeyLength: *mut u32, tseId: *const i8, tseIdLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(tseId, tseIdLength))?.get_tse_info(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    unsafe { ffi::set_cstr_from_str(pubKey, pubKeyLength, tse_info.public_key_base64.as_str()) };
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_getOpenTransactions(transactionNumbers: *mut *mut u32, transactionNumbersLength: *mut u32) -> i32 {
    at_getOpenTransactionsWithTse(transactionNumbers, transactionNumbersLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_getOpenTransactionsWithTse(transactionNumbers: *mut *mut u32, transactionNumbersLength: *mut u32, tseId: *const i8, tseIdLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(tseId, tseIdLength))?.get_tse_info(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    unsafe {
        ffi::set_u32_buf(transactionNumbers, tse_info.current_started_transaction_numbers.iter().map(|t| t.to_owned() as u32).collect::<Vec<u32>>().as_slice());
        ffi::set_u32_ptr(transactionNumbersLength, tse_info.current_number_of_started_transactions as u32);
    }
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_getTransactionCounter(counter: *mut u32) -> i32 {
    at_getTransactionCounterWithTse(counter, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_getTransactionCounterWithTse(counter: *mut u32, tseId: *const i8, tseIdLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(tseId, tseIdLength))?.get_tse_info(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    unsafe { ffi::set_u32_ptr(counter, tse_info.current_number_of_started_transactions as u32) }; // CLARIFY
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_getSignatureCounter(counter: *mut u32) -> i32 {
    at_getSignatureCounterWithTse(counter, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_getSignatureCounterWithTse(counter: *mut u32, tseId: *const i8, tseIdLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(tseId, tseIdLength))?.get_tse_info(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    unsafe { ffi::set_u32_ptr(counter, tse_info.current_number_of_signatures as u32) };
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_getSignatureAlgorithm(signatureAlgorithm: *mut *mut i8, signatureAlgorithmLength: *mut u32) -> i32 {
    at_getSignatureAlgorithmWithTse(signatureAlgorithm, signatureAlgorithmLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_getSignatureAlgorithmWithTse(signatureAlgorithm: *mut *mut i8, signatureAlgorithmLength: *mut u32, tseId: *const i8, tseIdLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(tseId, tseIdLength))?.get_tse_info(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    unsafe { ffi::set_cstr(signatureAlgorithm as *mut *mut u8, signatureAlgorithmLength, tse_info.signature_algorithm) };
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_getLogTimeFormat(logTimeFormat: *mut *mut i8, logTimeFormatLength: *mut u32) -> i32 {
    at_getLogTimeFormatWithTse(logTimeFormat, logTimeFormatLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_getLogTimeFormatWithTse(logTimeFormat: *mut *mut i8, logTimeFormatLength: *mut u32, tseId: *const i8, tseIdLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(tseId, tseIdLength))?.get_tse_info(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    unsafe { ffi::set_cstr(logTimeFormat as *mut *mut u8, logTimeFormatLength, tse_info.log_time_format) };
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_getVersion(version: *mut *mut i8, versionLength: *mut u32) -> i32 {
    ReturnCode::Unknown.into() // CLARIFY
}

#[no_mangle]
extern "C" fn at_getServiceVersion(version: *mut *mut i8, versionLength: *mut u32) -> i32 {
    ReturnCode::Unknown.into() // CLARIFY
}

#[no_mangle]
extern "C" fn at_getVersionDetails(versionDetails: *mut *mut i8, versionDetailsLength: *mut u32) -> i32 {
    ReturnCode::Unknown.into() // CLARIFY
}

#[no_mangle]
extern "C" fn at_getSerialNumber(serial: *mut *mut u8, serialLength: *mut u32) -> i32 {
    at_getSerialNumberWithTse(serial, serialLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_getSerialNumberWithTse(serial: *mut *mut u8, serialLength: *mut u32, tseId: *const i8, tseIdLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(tseId, tseIdLength))?.get_tse_info(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    unsafe { ffi::set_cstr(serial, serialLength, tse_info.serial_number_octet) };
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_preload() -> i32 {
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_load() -> i32 {
    if !crate::config::has_read_config() {
        return ReturnCode::ConfigFileNotFound.into();
    }

    match logging::configure_logging() {
        Ok(_) => {}
        Err(logging::Error::LoggerAlreadyConfigured) => {}
        Err(err) => {
            println!("{}", err);
            return ReturnCode::Unknown.into();
        }
    }

    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_unload() -> i32 {
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_verifyConfigEntry() -> i32 {
    at_verifyConfigEntryWithTse(b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_verifyConfigEntryWithTse(configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented.into()
}

#[no_mangle]
extern "C" fn at_free(ptr: *mut *mut ::std::os::raw::c_void) {
    unsafe { ffi::free_ptr(ptr) };
}

#[cfg(not(feature = "stdcall"))]
#[no_mangle]
extern "C" fn asigntse_free(ptr: *mut *mut ::std::os::raw::c_void) {
    unsafe { ffi::free_ptr(ptr) };
}

#[no_mangle]
extern "C" fn at_registerClientId(clientId: *const i8, clientIdLength: u32) -> i32 {
    at_registerClientIdWithTse(clientId, clientIdLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_registerClientIdWithTse(clientId: *const i8, clientIdLength: u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    let register_client_id_request = RegisterClientIdRequest { client_id: ffi::from_cstr(clientId, clientIdLength) };

    let register_client_id_response = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.register_client_id(&register_client_id_request), |err: client::Error| {
        error!("{}", err);
        match err {
            client::Error::Unsuccessful(_) => ReturnCode::ClientIdNotRegistered,
            err => Into::<ReturnCode>::into(err),
        }
        .into()
    });

    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_getMaxLicencedClients(maxNumberClients: *mut u32) -> i32 {
    at_getMaxLicencedClientsWithTse(maxNumberClients, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_getMaxLicencedClientsWithTse(maxNumberClients: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    unsafe { super::seapi::getMaxNumberOfClientsWithTse(maxNumberClients, configEntry, configEntryLength) }
}

#[no_mangle]
extern "C" fn at_getRegisteredClients(clients: *mut *mut u8, clientsLength: *mut u32) -> i32 {
    at_getRegisteredClientsWithTse(clients, clientsLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_getRegisteredClientsWithTse(clients: *mut *mut u8, clientsLength: *mut u32, configEntry: *const i8, configEntryLength: u32) -> i32 {
    let tse_info = try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.get_tse_info(), |err: client::Error| {
        error!("{}", err);
        match err {
            client::Error::Unsuccessful(_) => ReturnCode::CannotRetrieveRegisteredClientIds,
            err => Into::<ReturnCode>::into(err),
        }
        .into()
    });

    unsafe { ffi::set_cstr_array(clients, clientsLength, &tse_info.current_client_ids) };
    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_setPace(paceUser: *const i8, paceUserLength: u32, pacePin: *const i8, pacePinLength: u32, paceApiKey: *const i8, paceApiKeyLength: u32) -> i32 {
    at_setPaceWithTse(paceUser, paceUserLength, pacePin, pacePinLength, paceApiKey, paceApiKeyLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_setPaceWithTse(paceUser: *const i8, paceUserLength: u32, pacePin: *const i8, pacePinLength: u32, paceApiKey: *const i8, paceApiKeyLength: u32, tseId: *const i8, tseIdLength: u32) -> i32 {
    ReturnCode::NotImplemented.into() // CLARIFY
}

#[no_mangle]
extern "C" fn at_addUserEntropy(entropyString: *const i8, entropyStringLength: u32) -> i32 {
    at_addUserEntropyWithTse(entropyString, entropyStringLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_addUserEntropyWithTse(entropyString: *const i8, entropyStringLength: u32, tseId: *const i8, tseIdLength: u32) -> i32 {
    ReturnCode::NotImplemented.into() // CLARIFY
}

#[no_mangle]
extern "C" fn at_setPins(adminPin: *const u8, adminPinLength: u32, adminPuk: *const u8, adminPukLength: u32) -> i32 {
    at_setPinsWithTse(adminPin, adminPinLength, adminPuk, adminPukLength, b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_setPinsWithTse(adminPin: *const u8, adminPinLength: u32, adminPuk: *const u8, adminPukLength: u32, tseId: *const i8, tseIdLength: u32) -> i32 {
    ReturnCode::NotImplemented.into() // CLARIFY
}

#[no_mangle]
extern "C" fn at_checkCompatibility(startIndex: u32, indexCnt: u32) -> i32 {
    ReturnCode::NotImplemented.into() // CLARIFY
}

#[no_mangle]
extern "C" fn at_runSelfTests() -> i32 {
    at_runSelfTestsWithTse(b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_runSelfTestsWithTse(configEntry: *const i8, configEntryLength: u32) -> i32 {
    try_or_return!(|| Client::get(ffi::from_cstr(configEntry, configEntryLength))?.execute_self_test(), |err: client::Error| {
        error!("{}", err);
        Into::<ReturnCode>::into(err).into()
    });

    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn at_checkSecureState() -> i32 {
    at_checkSecureStateWithTse(b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_checkSecureStateWithTse(configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::NotImplemented.into() // CLARIFY
}

#[no_mangle]
extern "C" fn at_reloadSecureElement() -> i32 {
    at_reloadSecureElementWithTse(b"default".as_ptr() as *const i8, "default".len() as u32)
}

#[no_mangle]
extern "C" fn at_reloadSecureElementWithTse(configEntry: *const i8, configEntryLength: u32) -> i32 {
    ReturnCode::ExecutionOk.into() // CLARIFY
}

#[no_mangle]
extern "C" fn at_install() -> i32 {
    ReturnCode::ExecutionOk.into() // CLARIFY
}

#[no_mangle]
extern "C" fn at_uninstall() -> i32 {
    ReturnCode::ExecutionOk.into() // CLARIFY
}
