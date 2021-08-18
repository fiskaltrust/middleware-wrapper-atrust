use once_cell::sync::Lazy;
use std::sync::Mutex;
use flexi_logger::{LogSpecification, FileSpec, Logger};

static LOGGER: Lazy<Mutex<Option<flexi_logger::LoggerHandle>>> = Lazy::new(|| Mutex::from(None));

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Lock for general config is poisoned")]
  LockingConfig,
  #[error("Lock for logger is poisoned")]
  LockingLogger,
  #[error("Could not parse log_level: {0}")]
  ParsingLogLevel(#[source] flexi_logger::FlexiLoggerError),
  #[error("Logger error: {0}")]
  Logger(#[from] flexi_logger::FlexiLoggerError),
  #[error("Logger already configured")]
  LoggerAlreadyConfigured,
}

pub fn configure_logging() -> Result<(), Error> {
    let general_config = crate::config::GENERAL_CONFIG.lock().map_err(|_| Error::LockingConfig)?;
    let mut logger = LOGGER.lock().map_err(|_| Error::LockingLogger)?;

    if !general_config.logging_enabled {
      *logger = None;
    } else {
      if logger.is_some() {
        return Err(Error::LoggerAlreadyConfigured);
      }

      let log_spec = LogSpecification::parse(&general_config.log_level).map_err(Error::ParsingLogLevel)?;

      let mut logger_builder = Logger::with(log_spec);

      if !general_config.logging_file && !general_config.logging_stderr {
        logger_builder = logger_builder.do_not_log();
      } else if general_config.logging_file {
        let file_spec = FileSpec::default().directory(&general_config.log_dir);

        logger_builder = logger_builder.log_to_file(file_spec);

        if general_config.logging_stderr {
          logger_builder = logger_builder.duplicate_to_stderr(flexi_logger::Duplicate::All);
        }
      }

      *logger = Some(logger_builder.start()?)
    }

    Ok(())
}
