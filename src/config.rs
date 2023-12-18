use crate::logger::LoggerType;
use kanaria::string::UCSStr;
use lexical_bool::LexicalBool;
use log::info;
use megalodon::SNS;
use regex::Regex;
use std::ops::Deref;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Config {
    pub software: SNS,
    pub instance_url: String,
    pub token: String,
}

impl Config {
    pub fn new(
        software_name: String,
        instance_url: String,
        token: String,
    ) -> Result<Config, String> {
        let software = match software_name.to_lowercase().as_str() {
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
                return Err(format!("* Software {} is unknown!", unsupported));
            }
        };
        if token.is_empty() {
            eprintln!("* ACCESS_TOKEN is not set. Generating...");
            let _ = crate::streamer::oath(software, instance_url.as_str());
            return Err(String::new());
        }
        Ok(Config {
            software,
            instance_url,
            token,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Filter {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub include_regex: Vec<Regex>,
    pub exclude_regex: Vec<Regex>,
    pub user_include: Vec<String>,
    pub user_exclude: Vec<String>,
    pub is_case_sensitive: bool,
}

impl Filter {
    pub fn new(
        include: Vec<String>,
        exclude: Vec<String>,
        user_include: Vec<String>,
        user_exclude: Vec<String>,
        is_case_sensitive: bool,
        is_regex: bool,
    ) -> Result<Filter, &'static str> {
        let include_plain: Vec<String>;
        let exclude_plain: Vec<String>;
        let mut include_regex: Vec<Regex>;
        let mut exclude_regex: Vec<Regex>;
        if is_regex {
            include_plain = vec![];
            exclude_plain = vec![];
            include_regex = vec![];
            exclude_regex = vec![];
            for i in include {
                let Ok(re) = Regex::new(i.as_str()) else {
                    return Err("Invalid Regex");
                };
                include_regex.push(re);
            }
            for i in exclude {
                let Ok(re) = Regex::new(i.as_str()) else {
                    return Err("Invalid Regex");
                };
                exclude_regex.push(re);
            }
        } else {
            include_plain = include;
            exclude_plain = exclude;
            include_regex = vec![];
            exclude_regex = vec![];
        }
        Ok(Filter {
            include: include_plain,
            exclude: exclude_plain,
            include_regex,
            exclude_regex,
            user_include,
            user_exclude,
            is_case_sensitive,
        })
    }
}

#[derive(Debug)]
pub struct Logger {
    pub logger_type: LoggerType,
    pub logger_url: Option<String>,
}

impl Logger {
    pub fn new(logger_name: String, logger_url: Option<String>) -> Logger {
        let logger_type = match logger_name.to_lowercase().as_str() {
            "stdout" => LoggerType::Stdout,
            "discord" => LoggerType::Discord,
            _ => {
                eprintln!("* LOGGER is not set. Falling back to stdout.");
                LoggerType::Stdout
            }
        };
        Logger {
            logger_type,
            logger_url,
        }
    }
}

#[derive(Debug)]
pub struct TimelineSetting {
    pub home: bool,
    pub local: bool,
    pub public: bool,
}

impl TimelineSetting {
    pub fn new(home: bool, local: bool, public: bool) -> Self {
        TimelineSetting {
            home,
            local,
            public,
        }
    }
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();
pub static FILTER: OnceLock<Filter> = OnceLock::new();
pub static LOGGER: OnceLock<Logger> = OnceLock::new();
pub static TIMELINES: OnceLock<TimelineSetting> = OnceLock::new();

// Read options from .env file
pub async fn load_config() -> Result<(), String> {
    // Parse CONFIG
    let Ok(software) = dotenvy::var("SOFTWARE") else {
            return Err("* SOFTWARE is not set; Please specify SOFTWARE to listen to.".to_string());
    };
    let Ok(instance_url) = dotenvy::var("INSTANCE_URL") else {
        return Err("* Please specify INSTANCE_URL to listen to.".to_string());
    };
    let token = dotenvy::var("ACCESS_TOKEN").unwrap_or_default();

    // Parse LOGGER
    let logging_method = dotenvy::var("LOGGER").unwrap_or_default();
    let logging_url = dotenvy::var("LOGGER_URL").ok();

    // Parse FILTER
    // TODO?: Parse in new() function instead of here?
    let is_regex: bool = if let Ok(regex) = dotenvy::var("USE_REGEX") {
        if let Ok(lb) = regex.parse::<LexicalBool>() {
            *lb.deref()
        } else {
            return Err("* The value of USE_REGEX doesn't match expected pattern!".to_string());
        }
    } else {
        false
    };

    let is_case_sensitive: bool = !is_regex
        && if let Ok(case_sensitive) = dotenvy::var("CASE_SENSITIVE") {
            if let Ok(lb) = case_sensitive.parse::<LexicalBool>() {
                *lb.deref()
            } else {
                return Err(
                    "* the value of case_sensitive doesn't match expected pattern!".to_string(),
                );
            }
        } else {
            true
        };

    let timelines = match dotenvy::var("TIMELINES") {
        Ok(tl) => {
            let mut home = false;
            let mut local = false;
            let mut public = false;
            for tl in tl.split(',') {
                match tl.to_lowercase().as_str() {
                    "home" => home = true,
                    "local" => local = true,
                    "public" => public = true,
                    invalid => {
                        eprintln!("* Timeline type {} is unkown!", invalid);
                    }
                }
            }
            if !(home || local || public) {
                eprintln!("* No valid timeline type found. Falling back to Home...");
                TimelineSetting::new(true, false, false)
            } else {
                TimelineSetting::new(home, local, public)
            }
        }
        Err(_) => {
            eprintln!("* No timelines specified. Falling back to Home...");

            TimelineSetting::new(true, false, false)
        }
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

    // Setting
    let config = Config::new(software, instance_url, token)?;
    let _config = CONFIG.get_or_init(|| config);

    let filter = Filter::new(
        include,
        exclude,
        user_include,
        user_exclude,
        is_case_sensitive,
        is_regex,
    )?;
    let _filter = FILTER.get_or_init(|| filter);

    let logger = Logger::new(logging_method, logging_url);
    let _logger = LOGGER.get_or_init(|| logger);

    let _timelines = TIMELINES.get_or_init(|| timelines);
    info!("{:?}", _config);
    info!("{:?}", _filter);
    info!("{:?}", _logger);
    info!("{:?}", _timelines);

    Ok(())
}
