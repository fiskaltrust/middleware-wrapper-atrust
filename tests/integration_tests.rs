use fake::{Fake, Faker};
use httpmock::prelude::*;
use log::info;
use middleware_wrapper_atrust::helpers::ffi;
use middleware_wrapper_atrust::idesscd::*;
use middleware_wrapper_atrust::return_codes::ReturnCode;
use std::convert::TryFrom;

#[macro_use]
mod helpers;

const CONFIG_FILE: &str = "./tests/asigntseonline.conf";
const CONFIG_FILE_TARGET: &str = "./target/asigntseonline.conf";

fn setup_mock_server() {
    let server = MockServer::start();
    let config = std::fs::read_to_string(CONFIG_FILE).unwrap();

    std::fs::write(CONFIG_FILE_TARGET, config.replace("{{ scu_url }}", &server.base_url())).unwrap();

    let mut mock_idesscd = MockIDeSscd::new();

    mock_idesscd.expect_get_tse_info().returning(|| Ok(Faker.fake::<TseInfo>()));
    mock_idesscd.expect_start_transaction().returning(|_| Ok(Faker.fake::<StartTransactionResponse>()));
    mock_idesscd.expect_update_transaction().returning(|_| Ok(Faker.fake::<UpdateTransactionResponse>()));
    mock_idesscd.expect_finish_transaction().returning(|_| Ok(Faker.fake::<FinishTransactionResponse>()));
    mock_idesscd.expect_set_tse_state().returning(|_| Ok(Faker.fake::<TseState>()));
    mock_idesscd.expect_register_client_id().returning(|_| Ok(Faker.fake::<RegisterClientIdResponse>()));
    mock_idesscd.expect_unregister_client_id().returning(|_| Ok(Faker.fake::<UnregisterClientIdResponse>()));
    mock_idesscd.expect_execute_set_tse_time().returning(|| Ok(()));
    mock_idesscd.expect_execute_self_test().returning(|| Ok(()));
    mock_idesscd.expect_start_export_session().returning(|_| Ok(Faker.fake::<StartExportSessionResponse>()));
    mock_idesscd.expect_start_export_session_by_time_stamp().returning(|_| Ok(Faker.fake::<StartExportSessionResponse>()));
    mock_idesscd.expect_start_export_session_by_transaction().returning(|_| Ok(Faker.fake::<StartExportSessionResponse>()));
    mock_idesscd.expect_export_data().returning(|_| Ok(Faker.fake::<ExportDataResponse>()));
    mock_idesscd.expect_end_export_session().returning(|_| Ok(Faker.fake::<EndExportSessionResponse>()));
    mock_idesscd.expect_echo().returning(|request| Ok(ScuDeEchoResponse { message: request.message }));

    server.mock(mock_call!(mock_idesscd, "/v1/tseinfo", get_tse_info));
    server.mock(mock_call!(mock_idesscd, "/v1/starttransaction", start_transaction, Faker.fake::<StartTransactionRequest>()));
    server.mock(mock_call!(mock_idesscd, "/v1/updatetransaction", update_transaction, Faker.fake::<UpdateTransactionRequest>()));
    server.mock(mock_call!(mock_idesscd, "/v1/finishtransaction", finish_transaction, Faker.fake::<FinishTransactionRequest>()));
    server.mock(mock_call!(mock_idesscd, "/v1/tsestate", set_tse_state, Faker.fake::<TseState>()));
    server.mock(mock_call!(mock_idesscd, "/v1/registerclientid", register_client_id, Faker.fake::<RegisterClientIdRequest>()));
    server.mock(mock_call!(mock_idesscd, "/v1/unregisterclientid", unregister_client_id, Faker.fake::<UnregisterClientIdRequest>()));
    server.mock(mock_call!(mock_idesscd, "/v1/executesettsetime", execute_set_tse_time));
    server.mock(mock_call!(mock_idesscd, "/v1/executeselftest", execute_self_test));
    server.mock(mock_call!(mock_idesscd, "/v1/startexportsession", start_export_session, Faker.fake::<StartExportSessionRequest>()));
    server.mock(mock_call!(mock_idesscd, "/v1/startexportsessionbytimestamp", start_export_session_by_time_stamp, Faker.fake::<StartExportSessionByTimeStampRequest>()));
    server.mock(mock_call!(mock_idesscd, "/v1/startexportsessionbytransaction", start_export_session_by_transaction, Faker.fake::<StartExportSessionByTransactionRequest>()));
    server.mock(mock_call!(mock_idesscd, "/v1/exportdata", export_data, Faker.fake::<ExportDataRequest>()));
    server.mock(mock_call!(mock_idesscd, "/v1/endexportsession", end_export_session, Faker.fake::<EndExportSessionRequest>()));
    server.mock(mock_call!(mock_idesscd, "/v1/echo", echo, Faker.fake::<ScuDeEchoRequest>()));
}

#[test]
fn api_test() {
    let dylib_path = test_cdylib::build_current_project();
    let dylib = dlopen::symbor::Library::open(&dylib_path).unwrap();

    setup_mock_server();

    let cfg_set_logging_stderr = unsafe { dylib.symbol::<extern "C" fn(bool) -> i32>("cfgSetLoggingStderr").unwrap() };
    assert_eq!(0, cfg_set_logging_stderr(true));

    let cfg_set_config_file = unsafe { dylib.symbol::<extern "C" fn(*const i8, u32) -> i32>("cfgSetConfigFile").unwrap() };
    assert_eq!(0, cfg_set_config_file(CONFIG_FILE_TARGET.as_ptr() as *const i8, CONFIG_FILE_TARGET.len() as u32));

    let at_get_public_key_with_tse = unsafe { dylib.symbol::<extern "C" fn(*mut *mut u8, *mut u32, *const i8, u32) -> i32>("at_getPublicKeyWithTse").unwrap() };

    let mut pub_key = std::mem::MaybeUninit::<*mut u8>::uninit();
    let mut pub_key_length = std::mem::MaybeUninit::<u32>::uninit();

    let tse_id = "default";
    let result: ReturnCode = ReturnCode::try_from(at_get_public_key_with_tse(pub_key.as_mut_ptr(), pub_key_length.as_mut_ptr(), tse_id.as_ptr() as *const i8, tse_id.len() as u32)).unwrap();

    assert_eq!(result, ReturnCode::ExecutionOk);

    unsafe { ffi::free_ptr(pub_key.as_mut_ptr() as *mut *mut std::os::raw::c_void) };

    let start_transaction = unsafe {
        dylib
            .symbol::<extern "C" fn(*const i8, u32, *const u8, u32, *const i8, u32, *const u8, u32, *mut u32, *mut i64, *mut *mut u8, *mut u32, *mut u32, *mut *mut u8, *mut u32) -> i32>("startTransaction")
            .unwrap()
    };

    // clientId: *const i8,
    // clientIdLength: u32,
    // processData: *const u8,
    // processDataLength: u32,
    // processType: *const i8,
    // processTypeLength: u32,
    // additionalData: *const u8,
    // additionalDataLength: u32,
    // transactionNumber: *mut u32,
    // logTime: *mut i64,
    // serialNumber: *mut *mut u8,
    // serialNumberLength: *mut u32,
    // signatureCounter: *mut u32,
    // signatureValue: *mut *mut u8,
    // signatureValueLength: *mut u32,

    let mut transaction_number = std::mem::MaybeUninit::<u32>::uninit();
    let mut log_time = std::mem::MaybeUninit::<i64>::uninit();
    let mut serial_number = std::mem::MaybeUninit::<*mut u8>::uninit();
    let mut serial_number_length = std::mem::MaybeUninit::<u32>::uninit();
    let mut signature_counter = std::mem::MaybeUninit::<u32>::uninit();
    let mut signature_value = std::mem::MaybeUninit::<*mut u8>::uninit();
    let mut signature_value_length = std::mem::MaybeUninit::<u32>::uninit();


    let result: ReturnCode = ReturnCode::try_from(
        start_transaction(
            "clientId".as_ptr() as *const i8,
            "clientId".len() as u32,
            "processData".as_bytes().as_ptr(),
            "processData".len() as u32,
            "processType".as_ptr() as *const i8,
            "processType".len() as u32,
            "additionalData".as_bytes().as_ptr(),
            "additionalData".len() as u32,
            transaction_number.as_mut_ptr(),
            log_time.as_mut_ptr(),
            serial_number.as_mut_ptr(),
            serial_number_length.as_mut_ptr(),
            signature_counter.as_mut_ptr(),
            signature_value.as_mut_ptr(),
            signature_value_length.as_mut_ptr(),
        )
    ).unwrap();

    assert_eq!(result, ReturnCode::ExecutionOk);

    info!("transaction_number: {:?}", transaction_number);
    info!("log_time: {:?}", log_time);
    info!("serial_number: {:?}", serial_number);
    info!("serial_number_length: {:?}", serial_number_length);
    info!("signature_counter: {:?}", signature_counter);
    info!("signature_value: {:?}", signature_value);
    info!("signature_value_length: {:?}", signature_value_length);

    unsafe { ffi::free_ptr(serial_number.as_mut_ptr() as *mut *mut std::os::raw::c_void) };
    unsafe { ffi::free_ptr(signature_value.as_mut_ptr() as *mut *mut std::os::raw::c_void) };
    
}
