use std::ops::Deref;

use kanaria::string::UCSStr;
use lexical_bool::LexicalBool;
use log::info;
use megalodon::SNS;
use streamer::ExtraTimeline;

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

    let extra_tl = match dotenvy::var("EXTRA_TIMELINE") {
        Err(_) => None,
        Ok(tl_type) => match tl_type.to_lowercase().as_str() {
            "public" => Some(ExtraTimeline::Public),
            "local" => Some(ExtraTimeline::Local),
            _ => {
                eprintln!("* EXTRA_TIMELINE is invalid. Valid values are Public or Local.");
                return;
            }
        },
    };

    let Ok(url) = dotenvy::var("INSTANCE_URL") else {
        eprintln!("* Please specify INSTANCE_URL to listen to.");
        return;
    };

    let Ok(token) = dotenvy::var("ACCESS_TOKEN") else {
        eprintln!("* ACCESS_TOKEN is not set. Generating...");
        streamer::oath(sns, url.as_str()).await;
        return;
    };

    let logging_method = match dotenvy::var("LOGGER") {
        Ok(l) => l,
        Err(_) => {
            eprintln!("* LOGGER is not set. Falling back to stdout.");
            "stdout".to_string()
        }
    };

    let logging_url = dotenvy::var("LOGGER_URL").ok();

    let is_regex: bool = if let Ok(regex) = dotenvy::var("USE_REGEX") {
        if let Ok(lb) = regex.parse::<LexicalBool>() {
            *lb.deref()
        } else {
            eprintln!("* The value of USE_REGEX doesn't match expected pattern!");
            return;
        }
    } else {
        false
    };

    let is_case_sensitive: bool = !is_regex
        && if let Ok(case_sensitive) = dotenvy::var("CASE_SENSITIVE") {
            if let Ok(lb) = case_sensitive.parse::<LexicalBool>() {
                *lb.deref()
            } else {
                eprintln!("* The value of CASE_SENSITIVE doesn't match expected pattern!");
                return;
            }
        } else {
            true
        };

    let include: Vec<String> = match dotenvy::var("INCLUDE") {
        Ok(include) => {
            if is_case_sensitive {
                include.split(',').map(|x| x.to_string()).collect()
            } else {
                include
                    .split(',')
                    .map(|x| UCSStr::from_str(x).lower_case().hiragana().to_string())
                    .collect()
            }
        }
        Err(_) => vec![],
    };

    let exclude: Vec<String> = match dotenvy::var("EXCLUDE") {
        Ok(exclude) => {
            if is_case_sensitive {
                exclude.split(',').map(|x| x.to_string()).collect()
            } else {
                exclude
                    .split(',')
                    .map(|x| UCSStr::from_str(x).lower_case().hiragana().to_string())
                    .collect()
            }
        }
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

    let Ok(filter) = logger::Filter::new(
        extra_tl.clone(),
        include,
        exclude,
        user_include,
        user_exclude,
        is_case_sensitive,
        is_regex,
    ) else {
        eprintln!("* Invalid regex syntax!");
        return;
    };

    info!("{:?}", token);
    info!("{:?}", filter);

    // Extra Timeline
    let extra_tl_handle = if let Some(tl) = extra_tl {
        tokio::spawn(streamer::streaming(
            sns.clone(),
            url.clone(),
            token.clone(),
            logging_method.clone(),
            logging_url.clone(),
            filter.clone(),
            Some(tl),
        ))
    } else {
        tokio::spawn(async {})
    };
    // Home Timeline
    let home_tl_handle = tokio::spawn(streamer::streaming(
        sns.clone(),
        url.clone(),
        token.clone(),
        logging_method.clone(),
        logging_url.clone(),
        filter.clone(),
        None,
    ));

    let _ = tokio::join!(home_tl_handle, extra_tl_handle);
}
