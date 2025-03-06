use anyhow::Result;
use config::{Config as ConfigBuilder, Environment, File};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub log: LogConfig,
    pub http: HttpConfig,
    pub data: DataConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct LogConfig {
    pub console: bool,
    pub color: bool,
    pub file: bool,
    pub filedir: String,
    pub target: bool,
    pub targets: Vec<String>,
    pub level: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            console: true,
            color: true,
            file: false,
            filedir: "logs".to_string(),
            target: false,
            targets: vec![],
            level: "trace".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HttpConfig {
    pub connect_timeout: u64,
    pub idle_timeout: u64,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            connect_timeout: 5,
            idle_timeout: 5,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct DataConfig {
    pub dir: String,
}

impl Default for DataConfig {
    fn default() -> Self {
        Self {
            dir: "data".to_string(),
        }
    }
}

pub fn load() -> Result<Config> {
    let config = ConfigBuilder::builder()
        .add_source(
            Environment::with_prefix("DEEP_QUANT")
                .try_parsing(true)
                .separator("_")
                .list_separator(" "),
        )
        .add_source(File::with_name("config.yml").required(false))
        .build()?;
    let result = config.try_deserialize()?;
    Ok(result)
}
