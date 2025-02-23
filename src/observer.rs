use std::sync::Arc;

use megalodon::{entities::StatusVisibility, streaming::Message, Megalodon};

use crate::{config::Timeline, filter::Filters, logger::Loggers};

pub async fn observe(
    client: Arc<Box<dyn Megalodon + Send + Sync>>,
    filters: Arc<Filters>,
    loggers: Arc<Loggers>,
    timeline: Timeline,
    need_dedup: bool,
) {
    let stream = match timeline {
        Timeline::Home => client.user_streaming(),
        Timeline::Local => client.local_streaming(),
        Timeline::Public => client.public_streaming(),
    }
    .await;
    let dedup = timeline == Timeline::Home && need_dedup;
    stream
        .listen(Box::new(move |message| {
            let filters = Arc::clone(&filters);
            let loggers = Arc::clone(&loggers);
            Box::pin(async move {
                if let Message::Update(status) = message {
                    if dedup && status.visibility == StatusVisibility::Public {
                        return;
                    }
                    if let Err(e) = filters.filter(&status) {
                        log::info!("{e}");
                    } else {
                        loggers.log(&status).await;
                    }
                }
            })
        }))
        .await;
}
