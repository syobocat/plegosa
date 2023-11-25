use log::info;
use megalodon::SNS;

mod logger;
mod streamer;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Read options from .env file
    let sns = match dotenvy::var("SOFTWARE") {
        Err(_) => {
            eprintln!("* SOFTWARE is not set; Please specify SOFTWARE to listen to.");
            return;
        }
        Ok(software) => match software.to_lowercase().as_str() {
            "pleroma" => SNS::Pleroma,
            "mastodon" => {
                eprintln!("* Software other than Pleroma is not tested!");
                SNS::Mastodon
            }
            "firefish" => {
                eprintln!("* Software other than Pleroma is not tested!");
                SNS::Firefish
            }
            "friendica" => {
                eprintln!("* Software other than Pleroma is not tested!");
                SNS::Friendica
            }
            unsupported => {
                eprintln!("* Software {} is unknown!", unsupported);
                return;
            }
        },
    };

    let Ok(url) = dotenvy::var("INSTANCE_URL") else {
        eprintln!("* Please specify INSTANCE_URL to listen to.");
        return
    };

    let Ok(token) = dotenvy::var("ACCESS_TOKEN") else {
        // If ACCSESS_TOKEN is not set, generate one
        eprintln!("* ACCESS_TOKEN is not set. Generating...");
        streamer::oath(sns, url.as_str()).await;
        return
    };

    let logging_method = match dotenvy::var("LOGGER") {
        Ok(l) => l,
        Err(_) => {
            eprintln!("* LOGGER is not set. Falling back to stdout.");
            "stdout".to_string()
        }
    };

    let logging_url: Option<String> = match dotenvy::var("LOGGER_URL") {
        Ok(url) => Some(url),
        Err(_) => None,
    };

    let include: Vec<String> = match dotenvy::var("INCLUDE") {
        Ok(include) => include.split(',').map(|x| x.to_string()).collect(),
        Err(_) => vec![],
    };

    let exclude: Vec<String> = match dotenvy::var("EXCLUDE") {
        Ok(exclude) => exclude.split(',').map(|x| x.to_string()).collect(),
        Err(_) => vec![],
    };

    let user_include: Vec<String> = match dotenvy::var("USER_INCLUDE") {
        Ok(include) => include.split(',').map(|x| x.to_string()).collect(),
        Err(_) => vec![],
    };

    let user_exclude: Vec<String> = match dotenvy::var("USER_EXCLUDE") {
        Ok(exclude) => exclude.split(',').map(|x| x.to_string()).collect(),
        Err(_) => vec![],
    };

    let filter = logger::Filter::new(include, exclude, user_include, user_exclude);

    info!("{:?}", filter);

    streamer::streaming(
        sns,
        url.as_str(),
        token,
        logging_method,
        logging_url,
        filter,
    )
    .await;
}
