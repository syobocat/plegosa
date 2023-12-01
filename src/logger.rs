use crate::streamer::ExtraTimeline;
use nanohtml2text::html2text;

#[derive(Clone)]
pub struct Logger {
    dest: String,
    url: Option<String>,
}

impl Logger {
    pub fn new(dest: String, url: Option<String>) -> Logger {
        Logger { dest, url }
    }

    pub fn log(self, message: megalodon::entities::status::Status) -> Result<(), &'static str> {
        match self.dest.to_lowercase().as_str() {
            "stdout" => {
                log::debug!("{:?}", message);
                println!("==========");
                println!(
                    "Name: {} ({})",
                    message.account.display_name, message.account.acct
                );
                println!("Content:");
                println!(
                    "{}",
                    message.plain_content.unwrap_or(html2text(&message.content))
                );
                println!("URL: {}", message.uri);
                Ok(())
            }
            "discord" => {
                let Some(webhook) = self.url else {
                    return Err("* Please set Webhook URL to LOGGER_URL.");
                };
                if ureq::post(&webhook)
                    .send_json(ureq::json!({
                        //"username": message.account.display_name,
                        //"avatar_url": message.account.avatar,
                        //"content": message.plain_content.unwrap_or(html2text(&message.content)),
                        "content": message.uri,
                    }))
                    .is_err()
                {
                    Err("* Something happend executing Webhook.")
                } else {
                    Ok(())
                }
            }
            _ => Err("* Unkown logger."),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Filter {
    extra_tl: Option<ExtraTimeline>,
    include: Vec<String>,
    exclude: Vec<String>,
    user_include: Vec<String>,
    user_exclude: Vec<String>,
    is_case_sensitive: bool,
}

impl Filter {
    pub fn new(
        extra_tl: Option<ExtraTimeline>,
        include: Vec<String>,
        exclude: Vec<String>,
        user_include: Vec<String>,
        user_exclude: Vec<String>,
        is_case_sensitive: bool,
    ) -> Filter {
        Filter {
            extra_tl,
            include,
            exclude,
            user_include,
            user_exclude,
            is_case_sensitive,
        }
    }
}

pub fn egosa(
    message: megalodon::entities::status::Status,
    settings: Filter,
    tl: Option<ExtraTimeline>,
) -> bool {
    // Remove dupicates from Home Timeline
    if tl.is_none() && message.visibility == megalodon::entities::StatusVisibility::Public {
        match settings.extra_tl {
            Some(ExtraTimeline::Public) => return false,
            Some(ExtraTimeline::Local) => return message.account.acct.contains('@'),
            _ => {}
        };
    }

    if !settings.user_include.is_empty() && !settings.user_include.contains(&message.account.acct) {
        return false;
    }
    if settings.user_exclude.contains(&message.account.acct) {
        return false;
    }
    let content = if settings.is_case_sensitive {
        message.content
    } else {
        message.content.to_lowercase()
    };
    if !settings.include.is_empty()
        && settings
            .include
            .into_iter()
            .filter(|x| content.contains(x))
            .collect::<Vec<String>>()
            .is_empty()
    {
        return false;
    }
    if !settings
        .exclude
        .into_iter()
        .filter(|x| content.contains(x))
        .collect::<Vec<String>>()
        .is_empty()
    {
        return false;
    }
    true
}
