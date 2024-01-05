use crate::config::CONFIG;
use megalodon::entities::StatusVisibility;
use nanohtml2text::html2text;

pub fn log(message: megalodon::entities::status::Status) -> Result<(), &'static str> {
    let logger = &CONFIG.get().unwrap().logger;

    if logger.stdout.enable {
        let message = message.clone();
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
    }
    if logger.discord.enable {
        let message = message.clone();
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
        if ureq::post(&logger.discord.webhook).send_json(json).is_err() {
            return Err("* Something happend executing Webhook.");
        }
    }
    Ok(())
}
