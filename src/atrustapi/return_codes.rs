use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum ReturnCode {
    ExecutionOk = 0,
    RetrieveLogMessageFailed = -5001,
    StorageFailure = -5002,
    UpdateTimeFailed = -5003,
    ParameterMismatch = -5004,
    IdNotFound = -5005,
    TransactionNumberNotFound = -5006,
    NoDataAvailable = -5007,
    TooManyRecords = -5008,
    StartTransactionFailed = -5009,
    UpdateTransactionFailed = -5010,
    FinishTransactionFailed = -5011,
    RestoreFailed = -5012,
    StoringInitDataFailed = -5013,
    ExportCertFailed = -5014,
    NoLogMessage = -5015,
    ReadingLogMessage = -5016,
    NoTransaction = -5017,
    SeApiNotInitialized = -5018,
    TimeNotSet = -5019,
    CertificateExpired = -5020,
    SecureElementDisabled = -5021,
    UserNotAuthorized = -5022,
    UserNotAuthenticated = -5023,
    DescriptionNotSetByManufacturer = -5024,
    DescriptionSetByManufacturer = -5025,
    ExportSerialNumbersFailed = -5026,
    GetMaxNumberOfClientsFailed = -5027,
    GetCurrentNumberOfClientsFailed = -5028,
    GetMaxNumberTransactionsFailed = -5029,
    GetCurrentNumberOfTransactionsFailed = -5030,
    GetSupportedUpdateVariantsFailed = -5031,
    DeleteStoredDataFailed = -5032,
    UnexportedStoredData = -5033,
    SigningSystemOperationDataFailed = -5034,
    UserIdNotManaged = -5035,
    DisableSecureElementFailed = -5037,
    ConfigValueNotFound = -5038,
    InvalidConfig = -5039,
    SuspendSecureElementFailed = -5040,
    UnsuspendSecureElementFailed = -5041,
    GetOpenTransactionsFailed = -5042,
    GetLifecycleStateFailed = -5043,
    GetTransactionCounterFailed = -5044,
    GetSignatureCounterFailed = -5045,
    GetTotalLogMemory = -5046,
    GetLogTimeFormat = -5047,
    ExportPublicKeyFailed = -5048,
    ExportCertificateFailed = -5049,
    TpmConnect = -5050,
    InvalidClientId = -5051,
    ClientIdNotRegistered = -5052,
    ClientIdRegistrationFailed = -5053,
    CannotRetrieveRegisteredClientIds = -5054,
    CorruptedRegisteredClientIds = -5055,
    CorruptedAppData = -5056,
    SetPinsFailed = -5057,
    SeAlreadyInitialized = -5058,
    GetSignatureAlgorithmFailed = -5059,
    AuthenticationFailed = -4000,
    UnblockFailed = -4001,
    MissingParameter = -3000,
    FunctionNotSupported = -3001,
    Io = -3002,
    TseTimeout = -3003,
    AllocationFailed = -3004,
    ConfigFileNotFound = -3005,
    SeCommunicationFailed = -3006,
    TseCommandDataInvalid = -3007,
    TseResponseDataInvalid = -3008,
    ErsAlreadyMapped = -3009,
    NoErs = -3010,
    TseUnknownError = -3011,
    StreamWrite = -3012,
    BufferTooSmall = -3013,
    NoSuchKey = -3014,
    NoKey = -3015,
    SeApiDeactivated = -3016,
    SeApiNotDeactivated = -3017,
    AtLoadNotCalled = -3018,
    SeIpcProtocolError = -3019,
    AtSetPinsFailed = -3020,
    SeNotProvisionedError = -3021,
    //ERROR_SE_INSUFFICIENT_USER_ENTROPY_ERROR = -3021,
    SeAlreadyProvisioned = -3022,
    SeInSecureState = -3023,
    Unknown = -3100,
    UnsupportedPremiumFeature = -6000,
    NotImplemented = -6001,
}

impl ToString for ReturnCode {
    fn to_string(&self) -> String {
        let s = match self {
            ReturnCode::ExecutionOk => "EXECUTION_OK",
            ReturnCode::RetrieveLogMessageFailed => "ERROR_RETRIEVE_LOG_MESSAGE_FAILED",
            ReturnCode::StorageFailure => "ERROR_STORAGE_FAILURE",
            ReturnCode::UpdateTimeFailed => "ERROR_UPDATE_TIME_FAILED",
            ReturnCode::ParameterMismatch => "ERROR_PARAMETER_MISMATCH",
            ReturnCode::IdNotFound => "ERROR_ID_NOT_FOUND",
            ReturnCode::TransactionNumberNotFound => "ERROR_TRANSACTION_NUMBER_NOT_FOUND",
            ReturnCode::NoDataAvailable => "ERROR_NO_DATA_AVAILABLE",
            ReturnCode::TooManyRecords => "ERROR_TOO_MANY_RECORDS",
            ReturnCode::StartTransactionFailed => "ERROR_START_TRANSACTION_FAILED",
            ReturnCode::UpdateTransactionFailed => "ERROR_UPDATE_TRANSACTION_FAILED",
            ReturnCode::FinishTransactionFailed => "ERROR_FINISH_TRANSACTION_FAILED",
            ReturnCode::RestoreFailed => "ERROR_RESTORE_FAILED",
            ReturnCode::StoringInitDataFailed => "ERROR_STORING_INIT_DATA_FAILED",
            ReturnCode::ExportCertFailed => "ERROR_EXPORT_CERT_FAILED",
            ReturnCode::NoLogMessage => "ERROR_NO_LOG_MESSAGE",
            ReturnCode::ReadingLogMessage => "ERROR_READING_LOG_MESSAGE",
            ReturnCode::NoTransaction => "ERROR_NO_TRANSACTION",
            ReturnCode::SeApiNotInitialized => "ERROR_SE_API_NOT_INITIALIZED",
            ReturnCode::TimeNotSet => "ERROR_TIME_NOT_SET",
            ReturnCode::CertificateExpired => "ERROR_CERTIFICATE_EXPIRED",
            ReturnCode::SecureElementDisabled => "ERROR_SECURE_ELEMENT_DISABLED",
            ReturnCode::UserNotAuthorized => "ERROR_USER_NOT_AUTHORIZED",
            ReturnCode::UserNotAuthenticated => "ERROR_USER_NOT_AUTHENTICATED",
            ReturnCode::DescriptionNotSetByManufacturer => "ERROR_DESCRIPTION_NOT_SET_BY_MANUFACTURER",
            ReturnCode::DescriptionSetByManufacturer => "ERROR_DESCRIPTION_SET_BY_MANUFACTURER",
            ReturnCode::ExportSerialNumbersFailed => "ERROR_EXPORT_SERIAL_NUMBERS_FAILED",
            ReturnCode::GetMaxNumberOfClientsFailed => "ERROR_GET_MAX_NUMBER_OF_CLIENTS_FAILED",
            ReturnCode::GetCurrentNumberOfClientsFailed => "ERROR_GET_CURRENT_NUMBER_OF_CLIENTS_FAILED",
            ReturnCode::GetMaxNumberTransactionsFailed => "ERROR_GET_MAX_NUMBER_TRANSACTIONS_FAILED",
            ReturnCode::GetCurrentNumberOfTransactionsFailed => "ERROR_GET_CURRENT_NUMBER_OF_TRANSACTIONS_FAILED",
            ReturnCode::GetSupportedUpdateVariantsFailed => "ERROR_GET_SUPPORTED_UPDATE_VARIANTS_FAILED",
            ReturnCode::DeleteStoredDataFailed => "ERROR_DELETE_STORED_DATA_FAILED",
            ReturnCode::UnexportedStoredData => "ERROR_UNEXPORTED_STORED_DATA",
            ReturnCode::SigningSystemOperationDataFailed => "ERROR_SIGNING_SYSTEM_OPERATION_DATA_FAILED",
            ReturnCode::UserIdNotManaged => "ERROR_USER_ID_NOT_MANAGED",
            ReturnCode::DisableSecureElementFailed => "ERROR_DISABLE_SECURE_ELEMENT_FAILED",
            ReturnCode::ConfigValueNotFound => "ERROR_CONFIG_VALUE_NOT_FOUND",
            ReturnCode::InvalidConfig => "ERROR_INVALID_CONFIG",
            ReturnCode::SuspendSecureElementFailed => "ERROR_SUSPEND_SECURE_ELEMENT_FAILED",
            ReturnCode::UnsuspendSecureElementFailed => "ERROR_UNSUSPEND_SECURE_ELEMENT_FAILED",
            ReturnCode::GetOpenTransactionsFailed => "ERROR_GET_OPEN_TRANSACTIONS_FAILED",
            ReturnCode::GetLifecycleStateFailed => "ERROR_GET_LIFECYCLE_STATE_FAILED",
            ReturnCode::GetTransactionCounterFailed => "ERROR_GET_TRANSACTION_COUNTER_FAILED",
            ReturnCode::GetSignatureCounterFailed => "ERROR_GET_SIGNATURE_COUNTER_FAILED",
            ReturnCode::GetTotalLogMemory => "ERROR_GET_TOTAL_LOG_MEMORY",
            ReturnCode::GetLogTimeFormat => "ERROR_GET_LOG_TIME_FORMAT",
            ReturnCode::ExportPublicKeyFailed => "ERROR_EXPORT_PUBLIC_KEY_FAILED",
            ReturnCode::ExportCertificateFailed => "ERROR_EXPORT_CERTIFICATE_FAILED",
            ReturnCode::TpmConnect => "ERROR_TPM_CONNECT",
            ReturnCode::InvalidClientId => "ERROR_INVALID_CLIENT_ID",
            ReturnCode::ClientIdNotRegistered => "ERROR_CLIENT_ID_NOT_REGISTERED",
            ReturnCode::ClientIdRegistrationFailed => "ERROR_CLIENT_ID_REGISTRATION_FAILED",
            ReturnCode::CannotRetrieveRegisteredClientIds => "ERROR_CANNOT_RETRIEVE_REGISTERED_CLIENT_IDS",
            ReturnCode::CorruptedRegisteredClientIds => "ERROR_CORRUPTED_REGISTERED_CLIENT_IDS",
            ReturnCode::CorruptedAppData => "ERROR_CORRUPTED_APP_DATA",
            ReturnCode::SetPinsFailed => "ERROR_SET_PINS_FAILED",
            ReturnCode::SeAlreadyInitialized => "ERROR_SE_ALREADY_INITIALIZED",
            ReturnCode::GetSignatureAlgorithmFailed => "ERROR_GET_SIGNATURE_ALGORITHM_FAILED",
            ReturnCode::AuthenticationFailed => "ERROR_AUTHENTICATION_FAILED",
            ReturnCode::UnblockFailed => "ERROR_UNBLOCK_FAILED",
            ReturnCode::MissingParameter => "ERROR_MISSING_PARAMETER",
            ReturnCode::FunctionNotSupported => "ERROR_FUNCTION_NOT_SUPPORTED",
            ReturnCode::Io => "ERROR_IO",
            ReturnCode::TseTimeout => "ERROR_TSE_TIMEOUT",
            ReturnCode::AllocationFailed => "ERROR_ALLOCATION_FAILED",
            ReturnCode::ConfigFileNotFound => "ERROR_CONFIG_FILE_NOT_FOUND",
            ReturnCode::SeCommunicationFailed => "ERROR_SE_COMMUNICATION_FAILED",
            ReturnCode::TseCommandDataInvalid => "ERROR_TSE_COMMAND_DATA_INVALID",
            ReturnCode::TseResponseDataInvalid => "ERROR_TSE_RESPONSE_DATA_INVALID",
            ReturnCode::ErsAlreadyMapped => "ERROR_ERS_ALREADY_MAPPED",
            ReturnCode::NoErs => "ERROR_NO_ERS",
            ReturnCode::TseUnknownError => "ERROR_TSE_UNKNOWN_ERROR",
            ReturnCode::StreamWrite => "ERROR_STREAM_WRITE",
            ReturnCode::BufferTooSmall => "ERROR_BUFFER_TOO_SMALL",
            ReturnCode::NoSuchKey => "ERROR_NO_SUCH_KEY",
            ReturnCode::NoKey => "ERROR_NO_KEY",
            ReturnCode::SeApiDeactivated => "ERROR_SE_API_DEACTIVATED",
            ReturnCode::SeApiNotDeactivated => "ERROR_SE_API_NOT_DEACTIVATED",
            ReturnCode::AtLoadNotCalled => "ERROR_AT_LOAD_NOT_CALLED",
            ReturnCode::SeIpcProtocolError => "ERROR_SE_IPC_PROTOCOL_ERROR",
            ReturnCode::AtSetPinsFailed => "ERROR_AT_SET_PINS_FAILED",
            ReturnCode::SeNotProvisionedError => "ERROR_SE_NOT_PROVISIONED_ERROR",
            // Error::ERROR_SE_INSUFFICIENT_USER_ENTROPY_ERROR => "ERROR_SE_INSUFFICIENT_USER_ENTROPY_ERROR",
            ReturnCode::SeAlreadyProvisioned => "ERROR_SE_ALREADY_PROVISIONED",
            ReturnCode::SeInSecureState => "ERROR_SE_IN_SECURE_STATE",
            ReturnCode::Unknown => "ERROR_UNKNOWN",
            ReturnCode::UnsupportedPremiumFeature => "ERROR_UNSUPPORTED_PREMIUM_FEATURE",
            ReturnCode::NotImplemented => "ERROR_NOT_IMPLEMENTED",
        };

        format!("{} ({})", s, *self as i32)
    }
}
