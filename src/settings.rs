use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Target {
    pub url: String,
    pub queries: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    // pub interval_seconds: u64,
    // database: Database,
    // sparkpost: Sparkpost,
    // twitter: Twitter,
    // braintree: Braintree,
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
