use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::net::{Ipv4Addr};

#[derive(Debug, Deserialize)]
pub struct Target {
    pub url: String,
    pub queries: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub ip_address: Ipv4Addr,
    pub port: u16,
    pub metrics_path: String,
    pub targets: Vec<Target>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        debug!("Reading settings.");
        let mut s = Config::new();
        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("web_exporter"))?;
        s.merge(Environment::with_prefix("web_exporter"))?;
        s.try_into()
    }
}
