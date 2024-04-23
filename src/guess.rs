use std::sync::Arc;

use anyhow::anyhow;
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Form,
};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{api::DailyMessage, session, AppError};

const MAX_GUESSES: usize = 3;

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

#[derive(Template)]
#[template(path = "guess3.html")]
pub struct Guess3Template<'a> {
    pub color: &'a str,
    pub name_placeholder: &'a str,
    pub message: &'a str,
    pub badges: &'a Vec<String>,
}

pub async fn guess(
    State(msg): State<Arc<DailyMessage>>,
    session: Session,
    Form(guess): Form<Guess>,
) -> Result<Response, AppError> {
    let mut guesses = session::get_guesses(&session).await?;
    if guesses.len() == MAX_GUESSES {
        return Ok(StatusCode::NO_CONTENT.into_response());
    }

    guesses.push(guess.name);
    let guess_count = guesses.len();
    session::save_guesses(&session, guesses).await?;

    match guess_count {
        1 => Ok(Html(
            Guess1Template {
                color: &msg.color,
                name_placeholder: &msg.display_name.chars().map(|_| "*").collect::<String>(),
            }
            .render()?,
        )
        .into_response()),
        2 => Ok(Html(
            Guess2Template {
                color: &msg.color,
                name_placeholder: &msg.display_name.chars().map(|_| "*").collect::<String>(),
                message: &msg.text,
            }
            .render()?,
        )
        .into_response()),
        3 => Ok(Html(
            Guess3Template {
                color: &msg.color,
                name_placeholder: &msg.display_name.chars().map(|_| "*").collect::<String>(),
                message: &msg.text,
                badges: &msg.badges,
            }
            .render()?,
        )
        .into_response()),
        g => Err(AppError(anyhow!("guess {} is not implemented", g))),
    }
}
