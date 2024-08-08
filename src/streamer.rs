use crate::config::CONFIG;
use crate::filter;
use crate::logger;
use crate::utils::{die, die_with_error, success};
use log::error;
use log::info;
use megalodon::{default::NO_REDIRECT, generator, megalodon::AppInputOptions, streaming::Message};
use std::io;

#[derive(Clone, Debug)]
pub enum Timeline {
    Home,
    Local,
    Public,
}

pub async fn streaming(tl: Timeline) {
    let config = &CONFIG.instance;

    let client = generator(
        config.software.clone(),
        format!("https://{}", config.url),
        config.token.clone(),
        None,
    );
    if matches!(tl, Timeline::Home)
        && client.verify_account_credentials().await.is_err()
        && client.verify_app_credentials().await.is_err()
    {
        die("Token is invalid. Aborting...");
    }

    let (streaming, timeline_type) = match tl {
        Timeline::Public => (client.public_streaming().await, "Public"),
        Timeline::Local => (client.local_streaming().await, "Local"),
        Timeline::Home => (client.user_streaming().await, "Home"),
    };

    success(format!(
        "Successfully connected to {timeline_type} timeline!"
    ));

    streaming
        .listen(Box::new(move |message| {
            if let Message::Update(mes) = message {
                info!("Message received.");
                let result = filter::filter(&mes, &tl);
                if let Err(reason) = result {
                    info!("Message did not pass the filter. Reason: {reason}");
                } else {
                    info!("Message passed the filter.");
                    let _ = logger::log(mes).inspect_err(|e| error!("* {e}"));
                }
            }
        }))
        .await;
}

// Generate access token
pub async fn oauth(sns: megalodon::SNS, url: &str) {
    let client = generator(sns, format!("https://{url}"), None, None);
    let options = AppInputOptions {
        scopes: Some([String::from("read")].to_vec()),
        ..Default::default()
    };

    // Below is a copypasta from crate's example code
    match client.register_app(String::from("Plegosa"), &options).await {
        Ok(app_data) => {
            let client_id = app_data.client_id;
            let client_secret = app_data.client_secret;
            success("Authorization URL is generated.\n");
            println!("{}", app_data.url.unwrap());

            println!("\nEnter authorization code from website: ");
            let mut code = String::new();
            io::stdin().read_line(&mut code).ok();

            match client
                .fetch_access_token(
                    client_id,
                    client_secret,
                    code.trim().to_owned(),
                    NO_REDIRECT.to_owned(),
                )
                .await
            {
                Ok(token_data) => {
                    println!();
                    success("Access token has generated. Please add this to config.toml file.\n");
                    println!("token = {}", token_data.access_token);
                    /*
                    if let Some(refresh) = token_data.refresh_token {
                        println!("refresh_token: {}", refresh);
                    }
                    */
                }
                Err(err) => {
                    die_with_error("Error", err);
                }
            }
        }
        Err(err) => {
            die_with_error("Error", err);
        }
    }
}
