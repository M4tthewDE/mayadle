use std::sync::Arc;

use anyhow::anyhow;
use askama::Template;
use axum::{extract::State, response::Html, Form};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{AppError, User, GUESSES_KEY};

#[derive(Debug, Deserialize)]
pub struct Guess {
    name: String,
}

#[derive(Template)]
#[template(path = "guess1.html")]
struct Guess1Template<'a> {
    color: &'a str,
    name_placeholder: &'a str,
}

#[derive(Template)]
#[template(path = "guess2.html")]
struct Guess2Template<'a> {
    color: &'a str,
    name_placeholder: &'a str,
    message: &'a str,
}

pub async fn guess(
    State(user): State<Arc<User>>,
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
                color: &user.color,
                name_placeholder: &user.name.chars().map(|_| "*").collect::<String>(),
            }
            .render()?,
        )),
        2 => Ok(Html(
            Guess2Template {
                color: &user.color,
                name_placeholder: &user.name.chars().map(|_| "*").collect::<String>(),
                message: &user.message,
            }
            .render()?,
        )),
        g => Err(AppError(anyhow!("guess {} is not implemented", g))),
    }
}
