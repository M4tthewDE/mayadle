pub struct Message {
    pub color: String,
    pub name: String,
    pub message: String,
}

// TODO: generate daily
// maybe pick message on startup and then restart once a day?
pub fn pick_message() -> Message {
    Message {
        color: "#00FF7F".to_string(),
        name: "matthewde".to_string(),
        message: "link".to_string(),
    }
}
