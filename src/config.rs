use crate::streamer;
use log::info;
use megalodon::SNS;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub instance: InstanceSetting,
    #[serde(default)]
    pub timelines: TimelineSetting,
    #[serde(default)]
    pub filter: FilterSetting,
    #[serde(default)]
    pub logger: LoggerSetting,
}

#[derive(Debug, Deserialize)]
pub struct InstanceSetting {
    pub software: SNS,
    pub url: String,
    pub token: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct TimelineSetting {
    pub home: bool,
    pub local: bool,
    pub public: bool,
}

impl Default for TimelineSetting {
    fn default() -> Self {
        Self {
            home: true,
            local: false,
            public: false,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct FilterSetting {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub user_include: Vec<String>,
    pub user_exclude: Vec<String>,
    pub case_sensitive: bool,
    pub use_regex: bool,
}

impl Default for FilterSetting {
    fn default() -> Self {
        Self {
            include: vec![],
            exclude: vec![],
            user_include: vec![],
            user_exclude: vec![],
            case_sensitive: true,
            use_regex: false,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct LoggerSetting {
    pub stdout: Stdout,
    pub discord: Discord,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Stdout {
    pub enable: bool,
}

impl Default for Stdout {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Discord {
    pub enable: bool,
    pub webhook: String,
    pub use_embed: bool,
}

impl Default for Discord {
    fn default() -> Self {
        Self {
            enable: false,
            webhook: String::new(),
            use_embed: true,
        }
    }
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub async fn load() -> Result<(), String> {
    // Read options from config.toml file
    let Ok(toml) = fs::read_to_string("config.toml") else {
        return Err(if Path::new(".env").exists() {
            "* Obsolete .env config file found. Please migrate to config.toml."
        } else {
            "* config.toml is not found."
        }
        .to_owned());
    };
    let config: Config = match toml::from_str(&toml) {
        Ok(c) => c,
        Err(e) => return Err(format!("* Failed to load config.toml: {:?}", e.message())),
    };

    // Validate options
    if config.timelines.home && config.instance.token.is_none() {
        eprintln!("* timelines.home is set, but instance.token is empty. Generating a token...");
        streamer::oath(config.instance.software, &config.instance.url).await;
        return Err(String::new());
    }
    if config.filter.use_regex {
        for exp in &config.filter.include {
            if Regex::new(exp).is_err() {
                return Err("* filter.include contains a invalid regex.".to_owned());
            }
        }
        for exp in &config.filter.exclude {
            if Regex::new(exp).is_err() {
                return Err("* filter.exclude contains a invalid regex.".to_owned());
            }
        }
    }
    if config.logger.discord.enable && config.logger.discord.webhook.is_empty() {
        return Err(
            "* logger.discord.enable is set, but logger.discord.webhook is empty.".to_owned(),
        );
    }

    // Store options
    let config = CONFIG.get_or_init(|| config);
    info!("{:?}", config);

    Ok(())
}
