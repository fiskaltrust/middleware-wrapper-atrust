use std::convert::TryFrom;

use chrono::{TimeZone, Utc};
use fake::{Fake, Faker};
use middleware_wrapper_atrust::{
    atrustapi::return_codes::ReturnCode,
    helpers::ffi,
    idesscd::{EndExportSessionResponse, ExportDataResponse, FinishTransactionResponse, RegisterClientIdResponse, ScuDeEchoRequest, ScuDeEchoResponse, StartExportSessionResponse, StartTransactionResponse, TseInfo, TseState, UnregisterClientIdResponse, UpdateTransactionResponse},
};
use once_cell::sync::Lazy;
use serial_test::serial;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, Request, Respond, ResponseTemplate,
};

#[macro_use]
mod helpers;

static SCU_URL: Lazy<Option<String>> = Lazy::new(|| std::env::var("SCU_URL").ok());

const CONFIG_FILE: &str = "./tests/asigntseonline.conf";
const CONFIG_FILE_TARGET: &str = "./target/asigntseonline.conf";

pub struct FakerResponder(Box<dyn Fn(String) -> String + Send + Sync>);

impl FakerResponder {
    fn respond_with_reqest<REQ: Send + Sync + for<'de> serde::Deserialize<'de>, RES: Send + Sync + serde::Serialize, C: Fn(REQ) -> RES + 'static + Send + Sync>(mock: C) -> FakerResponder {
        FakerResponder(Box::new(move |req: String| {
            let de = serde_json::de::from_str(&req).unwrap();
            let res = mock(de);
            serde_json::to_string(&res).unwrap()
        }))
    }

    fn respond<RES: Send + Sync + serde::Serialize, C: Fn() -> RES + 'static + Send + Sync>(mock: C) -> FakerResponder {
        FakerResponder(Box::new(move |_: String| {
            let res = mock();
            serde_json::to_string(&res).unwrap()
        }))
    }
}

impl Respond for FakerResponder {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_string(self.0(String::from_utf8(request.body.clone()).unwrap()))
    }
}

static SETUP_MOCK_SERVER: Lazy<MockServer> = Lazy::new(|| {
    async_std::task::block_on(async {
        let mock_server = MockServer::start().await;

        let config = std::fs::read_to_string(CONFIG_FILE).unwrap();

        if let Some(scu_url) = SCU_URL.as_ref() {
            std::fs::write(CONFIG_FILE_TARGET, config.replace("{{ scu_url }}", scu_url)).unwrap();
            return mock_server;
        }

        std::fs::write(CONFIG_FILE_TARGET, config.replace("{{ scu_url }}", &mock_server.uri())).unwrap();

        Mock::given(method("POST")).and(path("/v1/starttransaction")).respond_with(FakerResponder::respond(|| Faker.fake::<StartTransactionResponse>())).mount(&mock_server).await;

        Mock::given(method("POST")).and(path("/v1/updatetransaction")).respond_with(FakerResponder::respond(|| Faker.fake::<UpdateTransactionResponse>())).mount(&mock_server).await;

        Mock::given(method("POST")).and(path("/v1/finishtransaction")).respond_with(FakerResponder::respond(|| Faker.fake::<FinishTransactionResponse>())).mount(&mock_server).await;

        Mock::given(method("GET")).and(path("/v1/tseinfo")).respond_with(FakerResponder::respond(|| Faker.fake::<TseInfo>())).mount(&mock_server).await;

        Mock::given(method("POST")).and(path("/v1/tsestate")).respond_with(FakerResponder::respond(|| Faker.fake::<TseState>())).mount(&mock_server).await;

        Mock::given(method("POST")).and(path("/v1/registerclientid")).respond_with(FakerResponder::respond(|| Faker.fake::<RegisterClientIdResponse>())).mount(&mock_server).await;

        Mock::given(method("POST")).and(path("/v1/unregisterclientid")).respond_with(FakerResponder::respond(|| Faker.fake::<UnregisterClientIdResponse>())).mount(&mock_server).await;

        Mock::given(method("POST")).and(path("/v1/executeselftest")).respond_with(ResponseTemplate::new(200)).mount(&mock_server).await;

        Mock::given(method("POST")).and(path("/v1/executesettsetime")).respond_with(ResponseTemplate::new(200)).mount(&mock_server).await;

        Mock::given(method("POST")).and(path("/v1/startexportsession")).respond_with(FakerResponder::respond(|| Faker.fake::<StartExportSessionResponse>())).mount(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/v1/startexportsessionbytimestamp"))
            .respond_with(FakerResponder::respond(|| Faker.fake::<StartExportSessionResponse>()))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/v1/startexportsessionbytransaction"))
            .respond_with(FakerResponder::respond(|| Faker.fake::<StartExportSessionResponse>()))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST")).and(path("/v1/exportdata")).respond_with(FakerResponder::respond(|| Faker.fake::<ExportDataResponse>())).mount(&mock_server).await;

        Mock::given(method("POST")).and(path("/v1/endexportsession")).respond_with(FakerResponder::respond(|| Faker.fake::<EndExportSessionResponse>())).mount(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/v1/echo"))
            .respond_with(FakerResponder::respond_with_reqest(|req: ScuDeEchoRequest| ScuDeEchoResponse { message: req.message }))
            .mount(&mock_server)
            .await;

        mock_server
    })
});

static SETUP_ATRUSTAPI: Lazy<dlopen::symbor::Library> = Lazy::new(|| {
    let dylib_path = test_cdylib::build_current_project();
    let dylib = dlopen::symbor::Library::open(&dylib_path).unwrap();

    let cfg_set_config_file = unsafe { dylib.symbol::<extern "C" fn(*const i8, u32) -> i32>("cfgSetConfigFile").unwrap() };
    assert_eq!(0, cfg_set_config_file(CONFIG_FILE_TARGET.as_ptr() as *const i8, CONFIG_FILE_TARGET.len() as u32));

    let at_load = unsafe { dylib.symbol::<extern "C" fn() -> i32>("at_load").unwrap() };
    assert_eq!(0, at_load());

    dylib
});

#[test]
#[ignore = "dotnet-test"]
#[serial]
fn only_setup_mocks() {
    Lazy::<MockServer>::force(&SETUP_MOCK_SERVER);

    let _ = std::io::Read::read(&mut std::io::stdin(), &mut [0u8]).unwrap();
}

#[test]
#[serial]
fn at_run_self_tests() {
    Lazy::<MockServer>::force(&SETUP_MOCK_SERVER);
    let dylib = &SETUP_ATRUSTAPI;

    let at_run_self_tests = unsafe { dylib.symbol::<extern "C" fn() -> i32>("at_runSelfTests").unwrap() };

    let result: ReturnCode = ReturnCode::try_from(at_run_self_tests()).unwrap();

    assert_eq!(result, ReturnCode::ExecutionOk);
}

#[test]
#[serial]
fn at_get_public_key_with_tse() {
    Lazy::<MockServer>::force(&SETUP_MOCK_SERVER);
    let dylib = &SETUP_ATRUSTAPI;

    let at_get_public_key_with_tse = unsafe { dylib.symbol::<extern "C" fn(*mut *mut u8, *mut u32, *const i8, u32) -> i32>("at_getPublicKeyWithTse").unwrap() };

    let mut pub_key = std::mem::MaybeUninit::<*mut u8>::uninit();
    let mut pub_key_length = std::mem::MaybeUninit::<u32>::uninit();

    let tse_id = "default";
    let result: ReturnCode = ReturnCode::try_from(at_get_public_key_with_tse(pub_key.as_mut_ptr(), pub_key_length.as_mut_ptr(), tse_id.as_ptr() as *const i8, tse_id.len() as u32)).unwrap();

    assert_eq!(result, ReturnCode::ExecutionOk);

    println!("pub_key: {}", unsafe { ffi::from_cstr(*pub_key.as_ptr() as *const i8, *pub_key_length.as_ptr()) });

    unsafe { ffi::free_ptr(pub_key.as_mut_ptr() as *mut *mut std::os::raw::c_void) };
}

#[test]
#[serial]
fn at_register_client_id() {
    at_register_client_id_internal();
}

fn at_register_client_id_internal() -> String {
    Lazy::<MockServer>::force(&SETUP_MOCK_SERVER);
    let dylib = &SETUP_ATRUSTAPI;

    let at_register_client_id = unsafe { dylib.symbol::<extern "C" fn(*const i8, u32) -> i32>("at_registerClientId").unwrap() };

    let client_id: String = Faker.fake();

    let result: ReturnCode = ReturnCode::try_from(at_register_client_id(client_id.as_ptr() as *const i8, client_id.len() as u32)).unwrap();

    assert_eq!(result, ReturnCode::ExecutionOk);

    client_id
}

#[test]
#[serial]
fn start_transaction() {
    Lazy::<MockServer>::force(&SETUP_MOCK_SERVER);
    let dylib = &SETUP_ATRUSTAPI;

    let start_transaction = unsafe {
        dylib
            .symbol::<extern "C" fn(*const i8, u32, *const u8, u32, *const i8, u32, *const u8, u32, *mut u32, *mut i64, *mut *mut u8, *mut u32, *mut u32, *mut *mut u8, *mut u32) -> i32>("startTransaction")
            .unwrap()
    };

    let client_id = at_register_client_id_internal();
    let mut transaction_number = std::mem::MaybeUninit::<u32>::uninit();
    let mut log_time = std::mem::MaybeUninit::<i64>::uninit();
    let mut serial_number = std::mem::MaybeUninit::<*mut u8>::uninit();
    let mut serial_number_length = std::mem::MaybeUninit::<u32>::uninit();
    let mut signature_counter = std::mem::MaybeUninit::<u32>::uninit();
    let mut signature_value = std::mem::MaybeUninit::<*mut u8>::uninit();
    let mut signature_value_length = std::mem::MaybeUninit::<u32>::uninit();

    let result: ReturnCode = ReturnCode::try_from(start_transaction(
        client_id.as_ptr() as *const i8,
        client_id.len() as u32,
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
    ))
    .unwrap();

    assert_eq!(result, ReturnCode::ExecutionOk);

    println!("transaction_number: {}", unsafe { *transaction_number.as_ptr() });
    println!("log_time: {}", Utc.timestamp(unsafe { *log_time.as_ptr() }, 0));
    println!("serial_number: {}", unsafe { ffi::from_cstr(*serial_number.as_ptr() as *const i8, *serial_number_length.as_ptr()) });
    println!("serial_number_length: {}", unsafe { *serial_number_length.as_ptr() });
    println!("signature_counter: {}", unsafe { *signature_counter.as_ptr() });
    println!("signature_value: {}", unsafe { ffi::from_cstr(*signature_value.as_ptr() as *const i8, *signature_value_length.as_ptr()) });
    println!("signature_value_length: {}", unsafe { *signature_value_length.as_ptr() });

    unsafe { ffi::free_ptr(serial_number.as_mut_ptr() as *mut *mut std::os::raw::c_void) };
    unsafe { ffi::free_ptr(signature_value.as_mut_ptr() as *mut *mut std::os::raw::c_void) };
}

#[test]
#[serial]
fn export_data_with_client_id() {
    Lazy::<MockServer>::force(&SETUP_MOCK_SERVER);
    let dylib = &SETUP_ATRUSTAPI;

    let export_data_with_client_id = unsafe { dylib.symbol::<extern "C" fn(*const i8, u32, *mut *mut u8, *mut u32) -> i32>("exportDataWithClientId").unwrap() };

    let client_id = at_register_client_id_internal();

    let mut exported_data = std::mem::MaybeUninit::<*mut u8>::uninit();
    let mut exported_data_length = std::mem::MaybeUninit::<u32>::uninit();

    let result: ReturnCode = ReturnCode::try_from(export_data_with_client_id(client_id.as_bytes().as_ptr() as *const i8, client_id.len() as u32, exported_data.as_mut_ptr(), exported_data_length.as_mut_ptr())).unwrap();

    assert_eq!(result, ReturnCode::ExecutionOk);

    println!("exported_data: {}", base64::encode(unsafe { ffi::from_cba(*exported_data.as_ptr(), *exported_data_length.as_ptr()) }));
    println!("exported_data_length: {}", unsafe { *exported_data_length.as_ptr() });

    unsafe { ffi::free_ptr(exported_data.as_mut_ptr() as *mut *mut std::os::raw::c_void) };
}
