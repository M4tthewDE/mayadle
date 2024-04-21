use std::sync::Arc;

use anyhow::anyhow;
use askama::Template;
use axum::{extract::State, response::Html};
use tower_sessions::Session;

use crate::{
    api::Message,
    guess::{Guess1Template, Guess2Template},
    AppError, GUESSES_KEY,
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    guesses: String,
}

pub async fn index(
    State(msg): State<Arc<Message>>,
    session: Session,
) -> Result<Html<String>, AppError> {
    let guesses: Vec<String> = session.get(GUESSES_KEY).await?.unwrap_or_default();

    let mut content = String::new();
    for (i, _) in guesses.iter().enumerate() {
        match i + 1 {
            1 => {
                content.push_str(
                    &Guess1Template {
                        color: &msg.color,
                        name_placeholder: &msg.name.chars().map(|_| "*").collect::<String>(),
                    }
                    .render()?,
                );
            }
            2 => {
                content.push_str(
                    &Guess2Template {
                        color: &msg.color,
                        name_placeholder: &msg.name.chars().map(|_| "*").collect::<String>(),
                        message: &msg.message,
                    }
                    .render()?,
                );
            }
            g => return Err(AppError(anyhow!("guess {} is not implemented", g))),
        }
    }

    Ok(Html(IndexTemplate { guesses: content }.render()?))
}
