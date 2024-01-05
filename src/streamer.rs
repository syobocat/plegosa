use crate::config::CONFIG;
use crate::filter;
use crate::logger;
use log::error;
use log::info;
use megalodon::{generator, streaming::Message};
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Debug)]
pub enum Timeline {
    Home,
    Local,
    Public,
}

// Workaround for #10
static HISTORY: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
fn get_history() -> &'static Mutex<Vec<String>> {
    HISTORY.get_or_init(|| {
        let mut vec = Vec::with_capacity(20);
        for _ in 0..20 {
            vec.push(String::new());
        }
        Mutex::new(vec)
    })
}

pub async fn streaming(tl: Timeline) {
    let config = &CONFIG.get().unwrap().instance;

    let client = generator(
        config.software.clone(),
        format!("https://{}", config.url),
        config.token.clone(),
        None,
    );
    if matches!(tl, Timeline::Home) && client.verify_app_credentials().await.is_err() {
        eprintln!("* Token is invalid. Aborting...");
        return;
    }

    let (streaming, timeline_type) = match tl {
        Timeline::Public => (
            client.public_streaming(format!("wss://{}", config.url)),
            "Public",
        ),
        Timeline::Local => (
            client.local_streaming(format!("wss://{}", config.url)),
            "Local",
        ),
        Timeline::Home => (
            client.user_streaming(format!("wss://{}", config.url)),
            "Home",
        ),
    };

    println!("* Successfully connected to {} timeline!", timeline_type);

    streaming
        .listen(Box::new(move |message| {
            if let Message::Update(mes) = message {
                info!("Message received.");
                let mut history = get_history().lock().unwrap();
                if filter::filter(mes.clone(), tl.clone()) && !history.contains(&mes.clone().id) {
                    info!("Filter passed.");
                    history.rotate_right(1);
                    history[0] = mes.clone().id;
                    info!("History: {history:?}");
                    if let Err(e) = logger::log(mes) {
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
