use std::sync::Arc;

use anyhow::{Context, Result};
use config::{Config, Timeline};
use env_logger::Env;
use filter::Filters;
use logger::Loggers;
use tokio::task::JoinSet;

mod auth;
mod config;
mod filter;
mod logger;
mod observer;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("plegosa=info"));

    let config = Config::load().context("Failed to load config")?;
    config.validate().await.context("Invalid config")?;

    let client = Arc::new(megalodon::generator(
        config.instance.software,
        config.instance.url.to_string(),
        config.instance.token,
        Some(USER_AGENT.to_owned()),
    )?);

    if config.timeline.targets.contains(&Timeline::Home) {
        client
            .verify_app_credentials()
            .await
            .context("Failed to verify token")?;
    }

    let filters = Arc::new(Filters::new(config.filter));
    let loggers = Arc::new(Loggers::new(config.logger));

    let mut handles = JoinSet::new();
    let need_dedup = config.timeline.targets.contains(&Timeline::Local)
        || config.timeline.targets.contains(&Timeline::Public);
    for timeline in config.timeline.targets {
        let client = Arc::clone(&client);
        let filters = Arc::clone(&filters);
        let loggers = Arc::clone(&loggers);
        let dedup = timeline == Timeline::Home && need_dedup;
        handles.spawn(observer::observe(client, filters, loggers, timeline, dedup));
    }
    handles.join_all().await;

    Ok(())
}
