use crate::config::CONFIG;
use html2md::parse_html;
use megalodon::entities::{attachment::AttachmentType, Status, StatusVisibility};
use nanohtml2text::html2text;
use ureq::json;

pub fn log(message: Status) -> Result<(), &'static str> {
    let logger = &CONFIG.get().unwrap().logger;

    if logger.stdout.enable {
        log::debug!("{:?}", message);
        println!("==========");
        println!(
            "Name: {} ({})",
            message.account.display_name, message.account.acct
        );
        println!("Content:");
        println!(
            "{}",
            message
                .plain_content
                .unwrap_or_else(|| html2text(&message.content))
        );
        println!("URL: {}", message.uri);
    }
    if logger.discord.enable {
        let json = if message.visibility == StatusVisibility::Private
            || message.visibility == StatusVisibility::Direct
            || logger.discord.use_embed
        {
            let mut images = vec![];
            for attachment in message
                .media_attachments
                .iter()
                .filter(|x| matches!(x.r#type, AttachmentType::Image))
                // Discord accepts up to 10 embeds
                .take(10)
            {
                images.push(json!({
                    "url": attachment.url,
                }));
            }

            let mut embeds = vec![];

            // Create an embed containing the message's content
            embeds.push(json!({
                "description": parse_html(&message.content).replace("[https://", "[").replace("\\#", "#").replace("\\_", "_"), // Workarounds for Discord's stupid Markdown parser
                "url": message.uri,
                "timestamp": message.created_at.to_rfc3339(),
                /*
                // while using author field looks better,
                // they aren't rendered as hyperlink on the mobile app.
                "author": {
                    "name": message.uri,
                    "url": message.uri,
                },
                */
                // So let's use title field instead.
                "title": message.uri,
                // Set first image if exist, leave empty if not
                "image": images.get(0).unwrap_or(&json!({})),
            }));

            // Create an embed for each remaining images
            for image in images.into_iter().skip(1) {
                embeds.push(json!({
                    "url": message.uri,
                    "image": image,
                }));
            }
            ureq::json!({
                "username": message.account.display_name,
                "avatar_url": message.account.avatar,
                "embeds": embeds,
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
