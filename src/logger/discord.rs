// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use anyhow::{Context, Result};
use megalodon::entities::{Status, StatusVisibility, attachment::AttachmentType};
use reqwest::{Client, RequestBuilder};
use serde_json::json;

use crate::config::DiscordConfig;

use super::Logger;

pub struct DiscordLogger {
    client: RequestBuilder,
    use_embed: bool,
}

impl DiscordLogger {
    pub fn new(config: DiscordConfig) -> Self {
        Self {
            client: Client::new().post(config.webhook.unwrap()),
            use_embed: config.use_embed,
        }
    }
}

impl Logger for DiscordLogger {
    async fn log(&self, status: &Status) -> Result<()> {
        let use_embed = self.use_embed
            || status.visibility == StatusVisibility::Private
            || status.visibility == StatusVisibility::Direct;
        let json = if use_embed {
            let mut images = status
                .media_attachments
                .iter()
                .filter(|attachment| attachment.r#type == AttachmentType::Image)
                .take(10)
                .map(|image| json!({"url": image.url}));
            let mut embeds = Vec::new();
            embeds.push(json!({
                "description": html2md::rewrite_html(&status.content, false).replace("[https://", "[").replace("\\#", "#").replace("\\_", "_"), // Workarounds for Discord's stupid Markdown parser
                "url": status.uri,
                "timestamp": status.created_at.to_rfc3339(),
                "title": status.uri,
                "image": images.next().unwrap_or(serde_json::Value::Null)
            }));
            for image in images {
                embeds.push(json!({
                "url": status.uri,
                "image": image
                }));
            }
            json!({
                "username": status.account.display_name,
                "avatar_url": status.account.avatar,
                "embeds": embeds
            })
        } else {
            json!({"content": status.uri})
        };
        self.client
            .try_clone()
            .context("Failed to clone RequestBuilder")?
            .json(&json)
            .send()
            .await?;
        Ok(())
    }
}
