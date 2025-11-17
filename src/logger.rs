// SPDX-FileCopyrightText: 2023-2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use anyhow::Result;
use discord::DiscordLogger;
use megalodon::entities::Status;
use stdout::StdoutLogger;

use crate::config::LoggerConfig;

pub mod discord;
pub mod stdout;

pub trait Logger {
    async fn log(&self, status: &Status) -> Result<()>;
}

pub struct Loggers {
    stdout: Option<StdoutLogger>,
    discord: Option<DiscordLogger>,
}

impl Loggers {
    pub fn new(config: LoggerConfig) -> Self {
        let stdout = if config.stdout.enable {
            Some(StdoutLogger::new())
        } else {
            None
        };
        let discord = if config.discord.enable {
            Some(DiscordLogger::new(config.discord))
        } else {
            None
        };
        Self { stdout, discord }
    }

    pub async fn log(&self, status: &Status) {
        if let Some(stdout) = &self.stdout {
            if let Err(e) = stdout.log(status).await {
                log::error!("{e}");
            };
        }
        if let Some(discord) = &self.discord {
            if let Err(e) = discord.log(status).await {
                log::error!("{e}");
            };
        }
    }
}
