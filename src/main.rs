use config::CONFIG;
use streamer::Timeline;
use utils::die_with_error;

mod config;
mod filter;
mod logger;
mod streamer;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();

    config::validate().await;

    let timelines = &CONFIG.timelines;

    // Home Timeline
    let home_tl_handle = if timelines.home {
        println!("* Connecting to Home timeline...");
        tokio::spawn(streamer::streaming(Timeline::Home))
    } else {
        tokio::spawn(async {})
    };

    // Local Timeline
    let local_tl_handle = if timelines.local {
        println!("* Connecting to Local timeline...");
        tokio::spawn(streamer::streaming(Timeline::Local))
    } else {
        tokio::spawn(async {})
    };

    // Public Timeline
    let public_tl_handle = if timelines.public {
        println!("* Connecting to Public timeline...");
        tokio::spawn(streamer::streaming(Timeline::Public))
    } else {
        tokio::spawn(async {})
    };

    tokio::try_join!(home_tl_handle, local_tl_handle, public_tl_handle)
        .unwrap_or_else(|e| die_with_error("Failed to spawn processes", e));
}
