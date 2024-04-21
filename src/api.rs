pub struct Message {
    pub color: String,
    pub name: String,
    pub message: String,
    pub badges: Vec<String>,
}

// TODO: generate daily
// maybe pick message on startup and then restart once a day?
pub fn pick_message() -> Message {
    Message {
        color: "#00FF7F".to_string(),
        name: "matthewde".to_string(),
        message: "link".to_string(),
        badges: vec![
            "https://static-cdn.jtvnw.net/badges/v1/b817aba4-fad8-49e2-b88a-7cc744dfa6ec/1"
                .to_string(),
            "https://static-cdn.jtvnw.net/badges/v1/a8f2084e-46b9-4bb9-ae5e-00d594aafc64/1"
                .to_string(),
        ],
    }
}
