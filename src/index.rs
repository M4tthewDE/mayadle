use anyhow::anyhow;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::Html};
use tower_sessions::Session;

use crate::{
    api::DailyMessage,
    guess::{Guess1Template, Guess2Template, Guess3Template},
    AppError, GUESSES_KEY,
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    guesses: String,
}

pub async fn index(
    State(msg): State<Arc<DailyMessage>>,
    session: Session,
) -> Result<Response, AppError> {
    let guesses: Vec<String> = session.get(GUESSES_KEY).await?.unwrap_or_default();

    let mut content = String::new();
    for (i, _) in guesses.iter().enumerate() {
        match i + 1 {
            1 => {
                content.push_str(
                    &Guess1Template {
                        color: &msg.color,
                        name_placeholder: &msg
                            .display_name
                            .chars()
                            .map(|_| "*")
                            .collect::<String>(),
                    }
                    .render()?,
                );
            }
            2 => {
                content.push_str(
                    &Guess2Template {
                        color: &msg.color,
                        name_placeholder: &msg
                            .display_name
                            .chars()
                            .map(|_| "*")
                            .collect::<String>(),
                        message: &msg.text,
                    }
                    .render()?,
                );
            }
            3 => {
                content.push_str(
                    &Guess3Template {
                        color: &msg.color,
                        name_placeholder: &msg
                            .display_name
                            .chars()
                            .map(|_| "*")
                            .collect::<String>(),
                        message: &msg.text,
                        badges: &msg.badges,
                    }
                    .render()?,
                );
            }
            g => return Err(AppError(anyhow!("guess {} is not implemented", g))),
        }
    }

    Ok(Html(IndexTemplate { guesses: content }.render()?).into_response())
}
