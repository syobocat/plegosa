// SPDX-FileCopyrightText: 2023-2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use std::{env, fs, process};

use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use megalodon::SNS;
use regex::Regex;
use serde::Deserialize;
use url::Url;

use crate::auth;

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub enum Timeline {
    Home,
    Local,
    Public,
}

#[derive(Deserialize)]
pub struct Config {
    pub instance: InstanceConfig,
    #[serde(default)]
    pub timeline: TimelineConfig,
    #[serde(default)]
    pub filter: FilterConfig,
    #[serde(default)]
    pub logger: LoggerConfig,
}

#[derive(Deserialize)]
pub struct InstanceConfig {
    pub software: SNS,
    pub url: Url,
    pub token: Option<String>,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct TimelineConfig {
    pub targets: Vec<Timeline>,
}

impl Default for TimelineConfig {
    fn default() -> Self {
        Self {
            targets: vec![Timeline::Home],
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct FilterConfig {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub user_include: Vec<String>,
    pub user_exclude: Vec<String>,
    pub case_sensitive: bool,
    pub use_regex: bool,
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            include: Vec::new(),
            exclude: Vec::new(),
            user_include: Vec::new(),
            user_exclude: Vec::new(),
            case_sensitive: true,
            use_regex: false,
        }
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct LoggerConfig {
    pub stdout: StdoutConfig,
    pub discord: DiscordConfig,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct StdoutConfig {
    pub enable: bool,
}

impl Default for StdoutConfig {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct DiscordConfig {
    pub enable: bool,
    pub webhook: Option<Url>,
    pub use_embed: bool,
}

impl Default for DiscordConfig {
    fn default() -> Self {
        Self {
            enable: false,
            webhook: None,
            use_embed: true,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let file_path = env::var("PLEGOSA_CONFIG").unwrap_or_else(|_| "config.toml".to_owned());
        let toml = fs::read_to_string(file_path)?;

        toml::from_str(&toml).context("Failed to parse config file")
    }

    pub async fn validate(&self) -> Result<()> {
        if self.timeline.targets.contains(&Timeline::Home) && self.instance.token.is_none() {
            println!(
            "{}",
            "* timelines.home is set, but instance.token is not provided. Generating new token..."
                .yellow()
        );
            auth::oauth(self.instance.software.clone(), self.instance.url.clone()).await?;
            process::exit(0);
        }
        if self.logger.discord.enable && self.logger.discord.webhook.is_none() {
            return Err(anyhow!(
                "logger.discord.enable is set, but logger.discord.webhook is not provided."
            ));
        }
        if self.filter.use_regex {
            for exp in &self.filter.include {
                let _ = Regex::new(exp)?;
            }
            for exp in &self.filter.exclude {
                let _ = Regex::new(exp)?;
            }
        }
        Ok(())
    }
}
