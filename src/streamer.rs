use log::error;
use megalodon::{generator, streaming::Message};

#[derive(Clone, Debug)]
pub enum ExtraTimeline {
    Public,
    Local,
}

pub async fn streaming(
    sns: megalodon::SNS,
    url: String,
    token: String,
    output_dest: String,
    logging_url: Option<String>,
    filter: crate::logger::Filter,
    tl: Option<ExtraTimeline>,
) {
    let client = generator(sns, format!("https://{}", url), Some(token), None);

    let streaming = match tl {
        Some(ExtraTimeline::Public) => client.public_streaming(format!("wss://{}", url)),
        Some(ExtraTimeline::Local) => client.local_streaming(format!("wss://{}", url)),
        None => client.user_streaming(format!("wss://{}", url)),
    };

    let logger = crate::logger::Logger::new(output_dest, logging_url);

    streaming
        .listen(Box::new(move |message| {
            if let Message::Update(mes) = message {
                if crate::logger::egosa(mes.clone(), filter.clone(), tl.clone()) {
                    if let Err(e) = logger.clone().log(mes) {
                        error!("{}", e);
                    };
                }
            }
        }))
        .await;
}

// Generate access token
pub async fn oath(sns: megalodon::SNS, url: &str) {
    let client = generator(sns, format!("https://{}", url), None, None);
    let options = megalodon::megalodon::AppInputOptions {
        scopes: Some([String::from("read")].to_vec()),
        ..Default::default()
    };

    // Below is a copypasta from crate's example code
    match client.register_app(String::from("Plegosa"), &options).await {
        Ok(app_data) => {
            let client_id = app_data.client_id;
            let client_secret = app_data.client_secret;
            println!("* Authorization URL is generated.\n");
            println!("{}", app_data.url.unwrap());

            println!("\nEnter authorization code from website: ");
            let mut code = String::new();
            std::io::stdin().read_line(&mut code).ok();

            match client
                .fetch_access_token(
                    client_id,
                    client_secret,
                    code.trim().to_string(),
                    megalodon::default::NO_REDIRECT.to_string(),
                )
                .await
            {
                Ok(token_data) => {
                    println!("\n* Access token has generated. Please add this to .env file.\n");
                    println!("ACCESS_TOKEN={}", token_data.access_token);
                    /*
                    if let Some(refresh) = token_data.refresh_token {
                        println!("refresh_token: {}", refresh);
                    }
                    */
                }
                Err(err) => {
                    println!("{:#?}", err);
                }
            }
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}
