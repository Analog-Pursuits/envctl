use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub project_name: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        //println!("config/{}.json", run_mode);
        let s = Config::builder()
            .add_source(File::with_name(&format!("config/{}.json", run_mode)).required(false))
            .add_source(Environment::with_prefix("envctl"))
            .build()?;
        return s.try_deserialize();
    }
}
