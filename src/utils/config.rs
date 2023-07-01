use anyhow::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use lazy_static::lazy_static;


lazy_static! {
    static ref CONFIG: String = read_config().unwrap();
    pub static ref APPCONFIG: Config = load_config().unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub postgres: PostgreSQLConfig,
    pub minio: MinioConfig,
    pub web: WebConfig,
    pub timer: TimerConfig,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PostgreSQLConfig {
    pub url: String,
}
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MinioConfig {
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
    pub endpoint: String,
    pub bucket_name: String,
}
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WebConfig {
    pub cros: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TimerConfig {
    pub cron: String,
}
fn read_config() -> Result<String> {
    let file_path = "application.yml";
    let mut file = File::open(file_path)?;
    let mut str_val = String::new();
    file.read_to_string(&mut str_val).unwrap();
    Ok(str_val)
}

pub fn load_config() -> Result<Config> {
    let config: Config = serde_yaml::from_str(&CONFIG.to_owned()).unwrap();
    Ok(config)
}