#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use crate::{config, helpers::ffi, logging, return_codes::ReturnCode};

#[no_mangle]
extern "C" fn cfgSetConfigFile(path: *const i8, pathLength: u32) -> i32 {
    log::info!("{}", "cfgSetConfigFile");

    if !crate::config::set_config_file(&ffi::from_cstr(path, pathLength)) {
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
extern "C" fn cfgTseAdd(
    tseID: *const i8,
    tseIDLength: u32,
    tseType: u32,
    connParam: *const i8,
    connParamLength: u32,
    atrustTseID: *const i8,
    atrustTseIDLength: u32,
    atrustApiKey: *const i8,
    atrustApiKeyLength: u32,
    timeAdminID: *const i8,
    timeAdminIDLength: u32,
    timeAdminPwd: *const i8,
    timeAdminPwdLength: u32,
) -> i32 {
    log::info!("{}", "cfgTseAdd");

    ReturnCode::NotImplemented.into()
}

#[no_mangle]
extern "C" fn cfgTseAddPremium(
    tseID: *const i8,
    tseIDLength: u32,
    tseType: u32,
    connParam: *const i8,
    connParamLength: u32,
    atrustTseID: *const i8,
    atrustTseIDLength: u32,
    atrustApiKey: *const i8,
    atrustApiKeyLength: u32,
    timeAdminID: *const i8,
    timeAdminIDLength: u32,
    timeAdminPwd: *const i8,
    timeAdminPwdLength: u32,
    licenceKey: *const i8,
    licenceKeyLength: u32,
) -> i32 {
    log::info!("{}", "cfgTseAddPremium");

    ReturnCode::NotImplemented.into()
}

#[no_mangle]
extern "C" fn cfgTseRemove(tseID: *const i8, tseIDLength: u32) -> i32 {
    log::info!("{}", "cfgTseRemove");

    ReturnCode::NotImplemented.into()
}

#[no_mangle]
extern "C" fn cfgSetLoggingEnabled(enabled: bool) -> i32 {
    log::info!("{}", "cfgSetLoggingEnabled");

    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn cfgSetLoggingStderr(enabled: bool) -> i32 {
    log::info!("{}", "cfgSetLoggingStderr");

    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn cfgSetLoggingFile(enabled: bool) -> i32 {
    log::info!("{}", "cfgSetLoggingFile");

    ReturnCode::NotImplemented.into()
}

#[no_mangle]
extern "C" fn cfgSetLogDir(path: *const i8, pathLength: u32) -> i32 {
    log::info!("{}", "cfgSetLogDir");

    ReturnCode::NotImplemented.into()
}

#[no_mangle]
extern "C" fn cfgSetLogLevel(logLevel: *const i8, logLevelLength: u32) -> i32 {
    log::info!("{}", "cfgSetLogLevel");

    ReturnCode::NotImplemented.into()
}

#[no_mangle]
extern "C" fn cfgSetLogAppend(enabled: bool) -> i32 {
    log::info!("{}", "cfgSetLogAppend");

    ReturnCode::NotImplemented.into()
}

#[no_mangle]
extern "C" fn cfgSetLogColors(enabled: bool) -> i32 {
    log::info!("{}", "cfgSetLogColors");

    ReturnCode::NotImplemented.into()
}

#[no_mangle]
extern "C" fn cfgSetLogDetails(enabled: bool) -> i32 {
    log::info!("{}", "cfgSetLogDetails");

    ReturnCode::NotImplemented.into()
}

#[no_mangle]
extern "C" fn cfgSetLogStderrColors(enabled: bool) -> i32 {
    log::info!("{}", "cfgSetLogStderrColors");

    ReturnCode::NotImplemented.into()
}

#[no_mangle]
extern "C" fn cfgSetHttpProxy(proxyUrl: *const i8, proxyUrlLength: u32) -> i32 {
    log::info!("{}", "cfgSetHttpProxy");

    let mut general_config = ok_or_return!(config::GENERAL_CONFIG.lock(), |err| ReturnCode::Unknown.into());

    general_config.http_proxy = Some(ffi::from_cstr(proxyUrl, proxyUrlLength));

    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn cfgSetHttpProxyWithUsernameAndPassword(proxyUrl: *const i8, proxyUrlLength: u32, proxyUsername: *const i8, proxyUsernameLength: u32, proxyPassword: *const i8, proxyPasswordLength: u32) -> i32 {
    log::info!("{}", "cfgSetHttpProxyWithUsernameAndPassword");

    let mut general_config = ok_or_return!(config::GENERAL_CONFIG.lock(), |err| ReturnCode::Unknown.into());

    general_config.http_proxy = Some(ffi::from_cstr(proxyUrl, proxyUrlLength));
    general_config.http_proxy_username = Some(ffi::from_cstr(proxyUsername, proxyUsernameLength));
    general_config.http_proxy_password = Some(ffi::from_cstr(proxyPassword, proxyPasswordLength));

    ReturnCode::ExecutionOk.into()
}

#[no_mangle]
extern "C" fn cfgSetTimeout(timeout: u64) -> i32 {
    log::info!("{}", "cfgSetTimeout");

    ReturnCode::NotImplemented.into()
}

#[no_mangle]
extern "C" fn cfgSetRetries(retries: u64) -> i32 {
    log::info!("{}", "cfgSetRetries");

    ReturnCode::NotImplemented.into()
}

#[no_mangle]
extern "C" fn cfgSetUploadMessageInterval(interval: u32) -> i32 {
    log::info!("{}", "cfgSetUploadMessageInterval");

    ReturnCode::NotImplemented.into()
}

#[no_mangle]
extern "C" fn cfgSetMaxAuditLogSize(maximum: u32) -> i32 {
    log::info!("{}", "cfgSetMaxAuditLogSize");

    ReturnCode::NotImplemented.into()
}
