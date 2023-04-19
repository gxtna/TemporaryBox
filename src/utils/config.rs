use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
#[derive(Debug, Serialize, Deserialize)]
struct PostgreSQLConfig {
    url: Option<String>,
}

impl PostgreSQLConfig {
    /* pub fn url(&self) -> Option<String> {
        self.url
    } */
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinioConfig {
    access_key: Option<String>,
    secret_key: Option<String>,
    region: Option<String>,
    enpoint: Option<String>,
    bucket_name: Option<String>,
}
impl MinioConfig{
    pub fn access_key(self) -> Option<String>  {
        self.access_key
    }
    /* pub fn secret_key(&self) -> Option<String>  {
        self.secret_key
    }
    pub fn region(&self) -> Option<String>  {
        self.region
    }
    pub fn enpoint(&self) -> Option<String>  {
        self.enpoint
    }
    pub fn bucket_name(&self) -> Option<String>  {
        self.bucket_name
    } */
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    postgres: Option<PostgreSQLConfig>,
    minio: Option<MinioConfig>,
}

impl Config {
    pub fn postgres(self) -> Option<PostgreSQLConfig> {
        self.postgres
    }
    pub fn minio(self) -> Option<MinioConfig> {
        self.minio
    }
}

pub fn read_conf() -> Config {
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
