use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::net::Ipv4Addr;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TargetMethod {
    Get,
    Post,
}

impl fmt::Display for TargetMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetMethod::Get => write!(f, "GET"),
            TargetMethod::Post => write!(f, "POST"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Target {
    pub name: Option<String>,
    pub url: String,
    pub method: Option<TargetMethod>,
    pub body: Option<String>,
    pub formdata: Option<Vec<(String, String)>>,
    pub queryparameters: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<String, String>>,
    pub queries: Option<Vec<String>>,
    pub extra_labels: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

impl From<Config> for Settings {
    fn from(config: Config) -> Self {
        let default: Settings = Settings::default();
        let parsed_settings: Settings = config.try_deserialize().unwrap_or_else(|err| {
            eprintln!("Error deserializing config: {}", err);
            default.clone() // Return default settings on error
        });

        Settings {
            ip_address: parsed_settings.ip_address.or(default.ip_address),
            port: parsed_settings.port.or(default.port),
            metrics_path: parsed_settings.metrics_path.or(default.metrics_path),
            targets: parsed_settings.targets.or(default.targets),
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        debug!("Reading settings.");
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("web_exporter"))
            .add_source(Environment::with_prefix("web_exporter"))
            .build()?;

        Ok(s.into())
    }
}
