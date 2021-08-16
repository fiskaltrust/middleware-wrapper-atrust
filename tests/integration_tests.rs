use fake::{Fake, Faker};
use httpmock::prelude::*;
use middleware_wrapper_atrust::helpers::ffi;
use middleware_wrapper_atrust::idesscd::TseInfo;
use middleware_wrapper_atrust::return_codes::ReturnCode;
use std::convert::TryFrom;

const CONFIG_FILE: &str = "./tests/asigntseonline.conf";
const CONFIG_FILE_TARGET: &str = "./target/asigntseonline.conf";

fn setup_mock_server() {
    let server = MockServer::start();
    let config = std::fs::read_to_string(CONFIG_FILE).unwrap();

    std::fs::write(CONFIG_FILE_TARGET, config.replace("{{ scu_url }}", &server.base_url())).unwrap();

    server.mock(|when, then| {
        when.method(GET).path("/v1/tseinfo");
        then.status(200).header("content-type", "application/json; charset=UTF-8").body(serde_json::ser::to_string(&Faker.fake::<TseInfo>()).unwrap());
    });
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
}
