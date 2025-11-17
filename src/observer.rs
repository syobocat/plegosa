// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use std::sync::Arc;

use megalodon::{Megalodon, entities::StatusVisibility, streaming::Message};

use crate::{
    config::Timeline,
    filter::{FilterResult, Filters},
    logger::Loggers,
};

pub async fn observe(
    client: Arc<Box<dyn Megalodon + Send + Sync>>,
    filters: Arc<Filters>,
    loggers: Arc<Loggers>,
    timeline: Timeline,
    dedup: bool,
) {
    let stream = match timeline {
        Timeline::Home => client.user_streaming(),
        Timeline::Local => client.local_streaming(),
        Timeline::Public => client.public_streaming(),
    }
    .await;

    log::info!("Successfully connected to the {timeline:?} timeline!");

    stream
        .listen(Box::new(move |message| {
            let filters = Arc::clone(&filters);
            let loggers = Arc::clone(&loggers);
            Box::pin(async move {
                if let Message::Update(status) = message {
                    if dedup && status.visibility == StatusVisibility::Public {
                        let url = status.uri;
                        log::debug!("Deduplicated: {url}");
                        return;
                    }
                    if let FilterResult::Block(reason) = filters.filter(&status) {
                        let url = status.uri;
                        log::debug!("{reason}: {url}");
                    } else {
                        loggers.log(&status).await;
                    }
                }
            })
        }))
        .await;
}
