use std::io::{self, Write};

use anyhow::{anyhow, Result};
use colored::Colorize;
use megalodon::{default::NO_REDIRECT, megalodon::AppInputOptions, SNS};
use url::Url;

use crate::USER_AGENT;

pub async fn oauth(sns: SNS, url: Url) -> Result<()> {
    let client = megalodon::generator(sns, url.to_string(), None, Some(USER_AGENT.to_owned()))?;
    let options = AppInputOptions {
        scopes: Some(vec!["read".to_owned()]),
        ..Default::default()
    };
    let app_data = client.register_app("Plegosa".to_owned(), &options).await?;
    let client_id = app_data.client_id;
    let client_secret = app_data.client_secret;
    let authorization_url = app_data
        .url
        .ok_or_else(|| anyhow!("Authorization URL is missing"))?;
    println!("{}", "* Authorization URL has been generated.".green());
    println!("\n{authorization_url}\n");
    print!("Enter authorization code from website: ");
    io::stdout().flush()?;

    let mut code = String::new();
    io::stdin().read_line(&mut code)?;

    let token_data = client
        .fetch_access_token(
            client_id,
            client_secret,
            code.trim().to_owned(),
            NO_REDIRECT.to_owned(),
        )
        .await?;
    let token = token_data.access_token;
    println!(
        "{}",
        "* Access token has been generated. Please add the following to your config.toml:".green()
    );
    println!("token = '{token}'");
    Ok(())
}
