use megalodon::entities::StatusVisibility;
use nanohtml2text::html2text;

#[derive(Debug)]
pub enum LoggerType {
    Stdout,
    Discord,
}

pub fn log(message: megalodon::entities::status::Status) -> Result<(), &'static str> {
    let logger = crate::config::LOGGER.get().unwrap();

    match logger.logger_type {
        LoggerType::Stdout => {
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
        LoggerType::Discord => {
            let Some(webhook) = logger.logger_url.clone() else {
                    return Err("* Please set Webhook URL to LOGGER_URL.");
                };
            let json = if message.visibility == StatusVisibility::Private
                || message.visibility == StatusVisibility::Direct
            {
                ureq::json!({
                    "username": message.account.display_name,
                    "avatar_url": message.account.avatar,
                    "content": format!("{}\n=====\nLink: <{}>", message.plain_content.unwrap_or(html2text(&message.content)), message.uri),
                })
            } else {
                ureq::json!({
                    "content": message.uri,
                })
            };
            if ureq::post(&webhook).send_json(json).is_err() {
                Err("* Something happend executing Webhook.")
            } else {
                Ok(())
            }
        }
    }
}
