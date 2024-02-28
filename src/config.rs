use crate::streamer;
use crate::utils::{die, die_with_error};
use log::info;
use megalodon::SNS;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::sync::OnceLock;
use unicode_normalization::UnicodeNormalization;

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

pub async fn load() {
    // Read options from config.toml file
    let toml = fs::read_to_string("config.toml")
        .unwrap_or_else(|e| die_with_error("Failed to read config.toml", e));
    let mut config: Config =
        toml::from_str(&toml).unwrap_or_else(|e| die_with_error("Failed to parse config.toml", e));

    // Validate options
    if config.timelines.home && config.instance.token.is_none() {
        eprintln!("* timelines.home is set, but instance.token is empty. Generating a token...");
        streamer::oauth(config.instance.software, &config.instance.url).await;
        std::process::exit(0);
    }
    if config.filter.use_regex {
        for exp in &config.filter.include {
            if Regex::new(exp).is_err() {
                die("filter.include contains a invalid regex.");
            }
        }
        for exp in &config.filter.exclude {
            if Regex::new(exp).is_err() {
                die("filter.exclude contains a invalid regex.");
            }
        }
    }
    if config.logger.discord.enable && config.logger.discord.webhook.is_empty() {
        die("logger.discord.enable is set, but logger.discord.webhook is empty.");
    }

    // Normalize
    if config.filter.case_sensitive {
        config.filter.include = config
            .filter
            .include
            .iter()
            .map(|x| x.nfc().collect())
            .collect();
        config.filter.exclude = config
            .filter
            .exclude
            .iter()
            .map(|x| x.nfc().collect())
            .collect();
    } else {
        config.filter.include = config
            .filter
            .include
            .iter()
            .map(|x| x.nfkc().collect())
            .collect();
        config.filter.exclude = config
            .filter
            .exclude
            .iter()
            .map(|x| x.nfkc().collect())
            .collect();
    }

    // Store options
    let config = CONFIG.get_or_init(|| config);
    info!("{:?}", config);
}
