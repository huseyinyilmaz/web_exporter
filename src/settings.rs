use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::Ipv4Addr;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub enum TargetMethod {
    GET,
    POST,
}

impl fmt::Display for TargetMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetMethod::GET => write!(f, "GET"),
            TargetMethod::POST => write!(f, "POST"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Target {
    pub url: String,
    pub method: Option<TargetMethod>,
    pub body: Option<String>,
    pub formdata: Option<Vec<(String, String)>>,
    pub queryparameters: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<String, String>>,
    pub queries: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub ip_address: Option<Ipv4Addr>,
    pub port: Option<u16>,
    pub metrics_path: Option<String>,
    pub targets: Option<Vec<Target>>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            ip_address: Some("0.0.0.0".parse().unwrap()),
            port: Some(3030),
            metrics_path: Some(String::from("metrics")),
            targets: Some(Vec::new()),
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        debug!("Reading settings.");
        let mut s = Config::new();
        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("web_exporter"))?;
        s.merge(Environment::with_prefix("web_exporter"))?;
        let default: Settings = Settings::default();
        let parsed_settings: Settings = s.try_into()?;
        Ok(Settings {
            ip_address: parsed_settings.ip_address.or(default.ip_address),
            port: parsed_settings.port.or(default.port),
            metrics_path: parsed_settings.metrics_path.or(default.metrics_path),
            targets: parsed_settings.targets.or(default.targets),
        })
    }
}
