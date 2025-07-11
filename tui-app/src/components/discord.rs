use std::env;
use reqwest::Client;
use serde_json::json;

struct DiscordConfig {
    token: String,
    channel_id: String,
}

fn full_endpoint(path: &str) -> String {
    let discord_api: &str = "https://discord.com/api/v10";
    format!("{}{}", discord_api, path)
}

async fn generate_invite(
    client: &Client,
    token: &str,
    channel_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = full_endpoint(&format!("/channels/{}/invites", channel_id));

    let payload = json!({
        "max_uses": 1,
        "max_age": 60,
        "temporary": false,
        "unique": true
    });

    let res = client
        .post(&url)
        .header("Authorization", token)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send().await?;

    if res.status().is_success() {
        let body: serde_json::Value = res.json().await?;

        if let Some(code) = body["code"].as_str() {
            Ok(format!("https://discord.gg/{}", code))
        } else {
            Err(format!("Invite code not found in response: {:?}", body).into())
        }
    } else {
        let err_status = &res.status();
        let err_text = res.text().await.unwrap_or_else(|_| "<unable to read body>".into());
        Err(format!("Discord API returned error {}: {}", err_status, err_text).into())
    }
}

fn discord_config() -> Result<DiscordConfig, Box<dyn std::error::Error>> {
    Ok(DiscordConfig {
        token: env::var("DISCORD_BOT_TOKEN")?,
        channel_id: env::var("DISCORD_CHANNEL_ID")?,
    })
}

pub async fn get_invite_link() -> Result<String, String> {
    log::info!("get_invite_link: started");

    let client = Client::new();
    let config = match discord_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            log::error!("discord_config error: {}", e);
            return Err(e.to_string());
        }
    };

    log::warn!("Using channel: {}", config.channel_id);

    match generate_invite(&client, &config.token, &config.channel_id).await {
        Ok(link) => {
            log::info!("Generated invite: {}", link);
            Ok(link)
        }
        Err(e) => {
            log::error!("generate_invite failed: {}", e);
            Err(e.to_string())
        }
    }
}
