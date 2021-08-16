use log::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use tini::Ini;

#[derive(Debug, Clone, PartialEq)]
pub enum TssType {
    AsignOnline,
    CryptoVision,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub name: String,
    pub tss_type: TssType,
    pub scu_url: String,
    pub atrust_vtss_id: Option<String>,
    pub atrust_api_key: Option<String>,
    pub time_admin_id: Option<String>,
    pub time_admin_pwd: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GeneralConfig {
    pub http_proxy: Option<String>,
    pub http_proxy_username: Option<String>,
    pub http_proxy_password: Option<String>,
    pub timeout: u64,
    pub retries: u64,
    pub logging_enabled: bool,
    pub logging_stderr: bool,
    pub logging_file: bool,
    pub log_dir: String,
    pub log_level: String,
    pub log_append: bool,
    pub log_colors: bool,
    pub log_details: bool,
    pub log_stderr_colors: bool,
    pub msg_upload_interval: u64,
    pub max_audit_log_size: u32,
}

pub const DEFAULT_TIMEOUT_VALUE: u64 = 1_500;
pub const DEFAULT_NUMBER_OF_RETRIES: u64 = 1;
pub const DEFAULT_MSG_UPLOAD_INTERVAL: u64 = 60 * 60 * 24;
pub const DEFAULT_MAX_AUDIT_LOG_SIZE: u32 = 128;

impl GeneralConfig {
    fn default() -> GeneralConfig {
        GeneralConfig {
            logging_enabled: false,
            logging_stderr: false,
            logging_file: false,
            log_dir: String::from("."),
            log_level: String::from("trace"),
            log_append: true,
            log_colors: false,
            log_details: true,
            log_stderr_colors: false,
            http_proxy: None,
            http_proxy_username: None,
            http_proxy_password: None,
            timeout: DEFAULT_TIMEOUT_VALUE,
            retries: DEFAULT_NUMBER_OF_RETRIES,
            msg_upload_interval: DEFAULT_MSG_UPLOAD_INTERVAL,
            max_audit_log_size: DEFAULT_MAX_AUDIT_LOG_SIZE,
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            name: String::from(""),
            tss_type: TssType::AsignOnline,
            scu_url: String::from(""),
            atrust_vtss_id: None,
            atrust_api_key: None,
            time_admin_id: None,
            time_admin_pwd: None,
        }
    }
}

pub static CONFIG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("asigntseonline.conf")));
pub static GENERAL_CONFIG: Lazy<Mutex<GeneralConfig>> = Lazy::new(|| Mutex::new(GeneralConfig::default()));

fn set_ini() -> Option<Ini> {
    if let Ok(cfg) = CONFIG_FILE.lock() {
        match Ini::from_file(&cfg.to_string()) {
            Ok(i) => Some(i),
            Err(_) => None,
        }
    } else {
        None
    }
}

static INI: Lazy<Mutex<Option<Ini>>> = Lazy::new(|| Mutex::new(set_ini()));

fn set_configs() -> HashMap<String, Config> {
    let ini = INI.lock().unwrap();
    let mut gconf = GENERAL_CONFIG.lock().unwrap();
    parse_config(ini.as_ref(), &mut *gconf)
}

pub static CONFIGS: Lazy<Mutex<HashMap<String, Config>>> = Lazy::new(|| Mutex::new(set_configs()));

pub fn get_tss(name: &str) -> Option<Config> {
    trace!("get_tss: {}", name.to_string());
    let c = some_or_return!(ok_or_return!(CONFIGS.lock(), |_| None).get(name), None).clone();

    trace!("use config: {}", c.name);

    Some(Config {
        name: c.name.to_string(),
        tss_type: c.tss_type,
        scu_url: c.scu_url.to_string(),
        atrust_vtss_id: c.atrust_vtss_id,
        atrust_api_key: c.atrust_api_key,
        time_admin_id: c.time_admin_id,
        time_admin_pwd: c.time_admin_pwd,
    })
}

pub fn read_config() -> bool {
    !CONFIGS.lock().unwrap().is_empty()
}

pub fn set_config_file(path: &str) -> bool {
    *try_or_return!(|| CONFIG_FILE.lock(), |_| false) = path.to_string();
    *try_or_return!(|| INI.lock(), |_| false) = set_ini();
    *try_or_return!(|| CONFIGS.lock(), |_| false) = set_configs();
    true
}

fn get_default_entry(ini_handle: Option<&Ini>) -> Option<Config> {
    let ini = some_or_return!(ini_handle.iter().next(), None);

    if ini.iter().count() == 0 {
        return None;
    }

    let mut default = String::from("default");
    if ini.iter().count() == 1 {
        let (n, _) = some_or_return!(ini.iter().next(), None);
        default = n.to_string();
    } else {
        for (section, _) in ini.iter() {
            if section == "default" {
                default = section.to_string();
            }
        }
    }

    let scu_url: String = some_or_return!(ini.get(&default, "scu_url"), None);

    let t: Option<String> = ini.get(&default, &String::from("tss_type"));
    t.as_ref()?;

    let tss_type = some_or_return!(
        match some_or_return!(t, None).as_str() {
            "1" => Some(TssType::AsignOnline),
            "2" => Some(TssType::CryptoVision),
            _ => {
                None
            }
        },
        None
    );

    let atrust_vtss_id: Option<String> = ini.get(&default, "atrust_vtss_id");
    let atrust_api_key: Option<String> = ini.get(&default, "atrust_api_key");

    let time_admin_id: Option<String> = ini.get(&default, "time_admin_id");
    let time_admin_pwd: Option<String> = ini.get(&default, "time_admin_pwd");

    Some(Config {
        tss_type,
        name: default,
        scu_url,
        atrust_api_key,
        atrust_vtss_id,
        time_admin_id,
        time_admin_pwd,
    })
}

fn parse_config(ini_handle: Option<&Ini>, gconf: &mut GeneralConfig) -> HashMap<String, Config> {
    let mut hm: HashMap<String, Config> = HashMap::new();

    if let Some(default_cfg) = get_default_entry(ini_handle) {
        hm.insert("default".to_string(), default_cfg);
    }

    if let Some(ini) = ini_handle.iter().next() {
        for (s_name, section) in ini.iter() {
            let sec: HashMap<&String, &String> = section.collect();

            if s_name == "default" {
                continue;
            }

            if s_name == "config" {
                let http_proxy = sec.get(&String::from("http_proxy")).map(|s| s.to_string());
                let http_proxy_username = sec.get(&String::from("http_proxy_username")).map(|s| s.to_string());
                let http_proxy_password = sec.get(&String::from("http_proxy_password")).map(|s| s.to_string());
                let timeout = sec.get(&String::from("timeout")).map(|s| s.parse().unwrap_or(DEFAULT_TIMEOUT_VALUE)).unwrap_or(DEFAULT_TIMEOUT_VALUE);
                let retries = sec.get(&String::from("retries")).map(|s| s.parse().unwrap_or(DEFAULT_NUMBER_OF_RETRIES)).unwrap_or(DEFAULT_NUMBER_OF_RETRIES);
                let logging_enabled = sec.get(&String::from("logging_enabled")).map(|s| s.to_string().parse().unwrap_or(false)).unwrap_or(false);
                let logging_stderr = sec.get(&String::from("logging_stderr")).map(|s| s.to_string().parse().unwrap_or(false)).unwrap_or(false);
                let logging_file = sec.get(&String::from("logging_file")).map(|s| s.to_string().parse().unwrap_or(false)).unwrap_or(false);
                let log_dir = sec.get(&String::from("log_dir")).map(|s| s.to_string()).unwrap_or_else(|| String::from("."));
                let log_level = sec.get(&String::from("log_level")).map(|s| s.to_string()).unwrap_or_else(|| String::from("trace"));
                let log_append = sec.get(&String::from("log_append")).map(|s| s.to_string().parse().unwrap_or(true)).unwrap_or(true);
                let log_colors = sec.get(&String::from("log_colors")).map(|s| s.to_string().parse().unwrap_or(false)).unwrap_or(false);
                let log_details = sec.get(&String::from("log_details")).map(|s| s.to_string().parse().unwrap_or(false)).unwrap_or(false);
                let log_stderr_colors = sec.get(&String::from("log_stderr_colors")).map(|s| s.to_string().parse().unwrap_or(false)).unwrap_or(false);
                let msg_upload_interval = sec.get(&String::from("msg_upload_interval")).map(|s| s.to_string().parse().unwrap_or(DEFAULT_MSG_UPLOAD_INTERVAL)).unwrap_or(DEFAULT_MSG_UPLOAD_INTERVAL);
                let max_audit_log_size = sec.get(&String::from("max_audit_log_size")).map(|s| s.to_string().parse().unwrap_or(DEFAULT_MAX_AUDIT_LOG_SIZE)).unwrap_or(DEFAULT_MAX_AUDIT_LOG_SIZE);

                gconf.http_proxy = http_proxy;
                gconf.http_proxy_username = http_proxy_username;
                gconf.http_proxy_password = http_proxy_password;
                gconf.timeout = timeout;
                gconf.retries = retries;
                gconf.logging_enabled = logging_enabled;
                gconf.logging_stderr = logging_stderr;
                gconf.logging_file = logging_file;
                gconf.log_dir = log_dir;
                gconf.log_level = log_level;
                gconf.log_append = log_append;
                gconf.log_colors = log_colors;
                gconf.log_details = log_details;
                gconf.log_stderr_colors = log_stderr_colors;
                gconf.msg_upload_interval = msg_upload_interval;
                gconf.max_audit_log_size = max_audit_log_size;
            }

            if !sec.contains_key(&"tss_type".to_string()) {
                continue;
            }

            if !sec.contains_key(&"scu_url".to_string()) {
                continue;
            }

            let mut tss_type: Option<TssType> = None;
            if let Some(t) = sec.get(&String::from("tss_type")) {
                tss_type = match t.as_str() {
                    "1" => Some(TssType::AsignOnline),
                    "2" => Some(TssType::CryptoVision),
                    _ => None,
                };

                if tss_type.is_none() {
                    continue;
                }
            }

            let name = match sec.get(&"name".to_string()) {
                Some(n) => n,
                None => s_name,
            };

            let vtss_id = sec.get(&String::from("atrust_vtss_id")).map(|s| s.to_string());
            let atrust_api_key = sec.get(&String::from("atrust_api_key")).map(|s| s.to_string());

            let time_admin_id = sec.get(&String::from("time_admin_id")).map(|s| s.to_string());
            let time_admin_pwd = sec.get(&String::from("time_admin_pwd")).map(|s| s.to_string());

            let scu_url = sec.get(&String::from("scu_url")).map(|s| s.to_string());

            hm.insert(
                name.to_string(),
                Config {
                    name: name.to_string(),
                    tss_type: tss_type.unwrap_or(TssType::AsignOnline),
                    scu_url: scu_url.unwrap_or_else(|| String::from("")),
                    atrust_vtss_id: vtss_id,
                    atrust_api_key,
                    time_admin_id,
                    time_admin_pwd,
                },
            );
        }
    }

    hm
}
