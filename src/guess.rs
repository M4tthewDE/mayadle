use std::sync::Arc;

use anyhow::anyhow;
use askama::Template;
use axum::{extract::State, response::Html, Form};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{api::Message, AppError, GUESSES_KEY};

#[derive(Debug, Deserialize)]
pub struct Guess {
    name: String,
}

#[derive(Template)]
#[template(path = "guess1.html")]
pub struct Guess1Template<'a> {
    pub color: &'a str,
    pub name_placeholder: &'a str,
}

#[derive(Template)]
#[template(path = "guess2.html")]
pub struct Guess2Template<'a> {
    pub color: &'a str,
    pub name_placeholder: &'a str,
    pub message: &'a str,
}

pub async fn guess(
    State(msg): State<Arc<Message>>,
    session: Session,
    Form(guess): Form<Guess>,
) -> Result<Html<String>, AppError> {
    let mut guesses: Vec<String> = session.get(GUESSES_KEY).await?.unwrap_or_default();
    guesses.push(guess.name);
    let guess_count = guesses.len();
    session.insert(GUESSES_KEY, guesses).await?;

    match guess_count {
        1 => Ok(Html(
            Guess1Template {
                color: &msg.color,
                name_placeholder: &msg.name.chars().map(|_| "*").collect::<String>(),
            }
            .render()?,
        )),
        2 => Ok(Html(
            Guess2Template {
                color: &msg.color,
                name_placeholder: &msg.name.chars().map(|_| "*").collect::<String>(),
                message: &msg.message,
            }
            .render()?,
        )),
        g => Err(AppError(anyhow!("guess {} is not implemented", g))),
    }
}
