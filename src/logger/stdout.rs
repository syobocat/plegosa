// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use anyhow::Result;
use megalodon::entities::Status;
use nanohtml2text::html2text;

use super::Logger;

pub struct StdoutLogger {}

impl StdoutLogger {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Logger for StdoutLogger {
    async fn log(&self, status: &Status) -> Result<()> {
        let url = &status.uri;
        let display_name = &status.account.display_name;
        let acct = &status.account.acct;
        let content = status
            .plain_content
            .clone()
            .unwrap_or_else(|| html2text(&status.content));
        println!("==========\nName: {display_name} ({acct})\nContent:\n{content}\nURL: {url}");

        Ok(())
    }
}
