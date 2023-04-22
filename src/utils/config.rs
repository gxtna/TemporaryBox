use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;


#[derive(Debug, Serialize, Deserialize,Default,Clone)]
pub struct PostgreSQLConfig {
    url: String,
}


impl PostgreSQLConfig {
    pub fn url(self) -> String {
        self.url
    }
}

#[derive(Debug, Serialize, Deserialize,Default,Clone)]
pub struct MinioConfig {
    access_key: String,
    secret_key: String,
    region: String,
    endpoint: String,
    bucket_name: String,
}

impl MinioConfig{
    
    pub fn access_key(self) -> String  {
        self.access_key
    }
    pub fn secret_key(self) -> String  {
        self.secret_key
    }
    pub fn region(self) -> String  {
        self.region
    }
    pub fn enpoint(self) -> String  {
        self.endpoint
    }
    pub fn bucket_name(self) -> String  {
        self.bucket_name
    }
}


#[derive(Debug, Serialize, Deserialize,Default,Clone)]
pub struct WebConfig{
    cros:String,
    address:String,
}

impl WebConfig {
    pub fn cros(self)->String { self.cros }
    pub fn address(self) -> String { self.address }
}

#[derive(Debug, Serialize, Deserialize,Default,Clone)]
pub struct TimerConfig{
    cron:String,
}

impl TimerConfig {
    pub fn cros(self)->String {self.cron}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    postgres: PostgreSQLConfig,
    minio: MinioConfig,
    web:WebConfig,
    timer:TimerConfig,
}

impl Config {
    pub fn postgres(self) -> PostgreSQLConfig {
        self.postgres
    }
    pub fn minio(self) -> MinioConfig {
        self.minio
    }
    pub fn web(self) -> WebConfig{
        self.web
    }
    pub fn timer(self) -> TimerConfig{
        self.timer
    }
}

pub fn read_conf() -> Config{

    let file_path = "config.toml";
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", file_path, e),
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e),
    };
    let config: Config = toml::from_str(&str_val).unwrap();
    config
}
#[warn(dead_code)]
fn test() -> Result<Config,()>{
    let x = test().unwrap();
    let file_path = "config.toml";
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", file_path, e),
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e),
    };
    let config: Config = toml::from_str(&str_val).unwrap();
    Ok(config)
    
}
