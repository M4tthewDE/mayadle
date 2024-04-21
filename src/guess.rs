use std::sync::Arc;

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

pub async fn guess(
    State(user): State<Arc<User>>,
    session: Session,
    Form(guess): Form<Guess>,
) -> Result<Html<String>, AppError> {
    let mut guesses: Vec<String> = session.get(GUESSES_KEY).await?.unwrap_or_default();
    guesses.push(guess.name);
    session.insert(GUESSES_KEY, guesses).await?;

    Ok(Html(
        Guess1Template {
            color: &user.color,
            name_placeholder: &user.name.chars().map(|_| "*").collect::<String>(),
        }
        .render()?,
    ))
}
