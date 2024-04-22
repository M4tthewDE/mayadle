use anyhow::{anyhow, Result};
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};

pub struct Message {
    pub color: String,
    pub name: String,
    pub message: String,
    pub badges: Vec<String>,
}

// TODO: generate daily
// maybe pick message on startup and then restart once a day?
pub async fn pick_message() -> Result<Message> {
    let _chatter = pick_chatter().await?;

    Ok(Message {
        color: "#00FF7F".to_string(),
        name: "matthewde".to_string(),
        message: "link".to_string(),
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
