#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use log::error;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::helpers::ffi;
use crate::return_codes::ReturnCode;

static LOGGER: Lazy<Mutex<Option<flexi_logger::LoggerHandle>>> = Lazy::new(|| Mutex::from(None));

#[no_mangle]
extern "C" fn cfgSetConfigFile(path: *const i8, pathLength: u32) -> i32 {
    if crate::config::set_config_file(&ffi::from_cstr(path, pathLength)) {
        ReturnCode::ExecutionOk as i32
    } else {
        ReturnCode::ConfigFileNotFound as i32
    }
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
    unimplemented!();
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
    unimplemented!();
}

#[no_mangle]
extern "C" fn cfgTseRemove(tseID: *const i8, tseIDLength: u32) -> i32 {
    unimplemented!();
}

#[no_mangle]
extern "C" fn cfgSetLoggingEnabled(enabled: bool) -> i32 {
    if enabled {
        if let Some(logger) = &mut *try_or_return!(|| LOGGER.lock(), |err| {
            error!("{}", err);
            ReturnCode::Unknown as i32
        }) {
            try_or_return!(
                || {
                    logger.set_new_spec(flexi_logger::LogSpecification::parse("info")?);
                    Ok(())
                },
                |err: flexi_logger::FlexiLoggerError| {
                    error!("{}", err);
                    ReturnCode::Unknown as i32
                }
            );
        } else {
            cfgSetLoggingStderr(true);
        }
    }

    ReturnCode::ExecutionOk as i32
}

#[no_mangle]
extern "C" fn cfgSetLoggingStderr(enabled: bool) -> i32 {
    if enabled {
        let mut lock = try_or_return!(|| LOGGER.lock(), |err| {
            error!("{}", err);
            ReturnCode::Unknown as i32
        });
        if lock.is_none() {
            try_or_return!(
                || {
                    *lock = Some(flexi_logger::Logger::try_with_str("info")?.start()?);
                    Ok(())
                },
                |err: flexi_logger::FlexiLoggerError| {
                    error!("{}", err);
                    ReturnCode::Unknown as i32
                }
            );
        }
    } else {
        let mut lock = try_or_return!(|| LOGGER.lock(), |err| {
            error!("{}", err);
            ReturnCode::Unknown as i32
        });

        if let Some(logger) = &*lock {
            logger.shutdown();
        } else {
            *lock = None;
        }
    }
    ReturnCode::ExecutionOk as i32
}

#[no_mangle]
extern "C" fn cfgSetLoggingFile(enabled: bool) -> i32 {
    unimplemented!();
}

#[no_mangle]
extern "C" fn cfgSetLogDir(path: *const i8, pathLength: u32) -> i32 {
    unimplemented!();
}

#[no_mangle]
extern "C" fn cfgSetLogLevel(logLevel: *const i8, logLevelLength: u32) -> i32 {
    unimplemented!();
}

#[no_mangle]
extern "C" fn cfgSetLogAppend(enabled: bool) -> i32 {
    unimplemented!();
}

#[no_mangle]
extern "C" fn cfgSetLogColors(enabled: bool) -> i32 {
    unimplemented!();
}

#[no_mangle]
extern "C" fn cfgSetLogDetails(enabled: bool) -> i32 {
    unimplemented!();
}

#[no_mangle]
extern "C" fn cfgSetLogStderrColors(enabled: bool) -> i32 {
    unimplemented!();
}

#[no_mangle]
extern "C" fn cfgSetHttpProxy(proxyUrl: *const i8, proxyUrlLength: u32) -> i32 {
    unimplemented!();
}

#[no_mangle]
extern "C" fn cfgSetHttpProxyWithUsernameAndPassword(proxyUrl: *const i8, proxyUrlLength: u32, proxyUsername: *const i8, proxyUsernameLength: u32, proxyPassword: *const i8, proxyPasswordLength: u32) -> i32 {
    unimplemented!();
}

#[no_mangle]
extern "C" fn cfgSetTimeout(timeout: u64) -> i32 {
    unimplemented!();
}

#[no_mangle]
extern "C" fn cfgSetRetries(retries: u64) -> i32 {
    unimplemented!();
}

#[no_mangle]
extern "C" fn cfgSetUploadMessageInterval(interval: u32) -> i32 {
    unimplemented!();
}

#[no_mangle]
extern "C" fn cfgSetMaxAuditLogSize(maximum: u32) -> i32 {
    unimplemented!();
}
