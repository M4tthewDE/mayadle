use anyhow::{anyhow, Result};
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DailyMessage {
    pub color: String,
    pub display_name: String,
    pub text: String,
    pub badges: Vec<String>,
}

// TODO: generate daily
// maybe pick message on startup and then restart once a day?
pub async fn pick_message() -> Result<DailyMessage> {
    let chatter = pick_chatter().await?;
    let msg = get_message(chatter).await?;

    Ok(DailyMessage {
        color: msg.tags.color,
        display_name: msg.display_name,
        text: msg.text,
        // TODO:
        badges: vec![
            "https://static-cdn.jtvnw.net/badges/v1/b817aba4-fad8-49e2-b88a-7cc744dfa6ec/1"
                .to_string(),
            "https://static-cdn.jtvnw.net/badges/v1/a8f2084e-46b9-4bb9-ae5e-00d594aafc64/1"
                .to_string(),
        ],
    })
}

const CHATTERS_URL: &str = "https://api.streamelements.com/kappa/v2/chatstats/maya/stats?limit=100";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Chatter {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatterResponse {
    chatters: Vec<Chatter>,
}

async fn pick_chatter() -> Result<Chatter> {
    let resp = reqwest::get(CHATTERS_URL)
        .await?
        .json::<ChatterResponse>()
        .await?;

    resp.chatters
        .choose(&mut rand::thread_rng())
        .cloned()
        .ok_or_else(|| anyhow!("no chatter found"))
}

#[derive(Serialize, Deserialize, Clone)]
struct Tags {
    badges: String,
    color: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    #[serde(alias = "displayName")]
    display_name: String,
    tags: Tags,
    text: String,
}
#[derive(Serialize, Deserialize)]
struct Messages {
    messages: Vec<Message>,
}

async fn get_message(chatter: Chatter) -> Result<Message> {
    reqwest::get(format!(
        "https://logs.ivr.fi/channel/maya/user/{}/random?json",
        chatter.name
    ))
    .await?
    .json::<Messages>()
    .await?
    .messages
    .first()
    .cloned()
    .ok_or_else(|| anyhow!("no message found"))
}
