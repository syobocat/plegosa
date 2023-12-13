use streamer::Timeline;

mod config;
mod filter;
mod logger;
mod streamer;

#[tokio::main]
async fn main() {
    env_logger::init();
    if let Err(e) = config::load_config().await {
        eprintln!("{}", e);
        return;
    };

    let timelines = config::TIMELINES.get().unwrap();

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

    let _ = tokio::join!(home_tl_handle, local_tl_handle, public_tl_handle);
}
