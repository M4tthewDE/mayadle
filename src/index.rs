use std::sync::Arc;

use anyhow::anyhow;
use askama::Template;
use axum::{extract::State, response::Html};
use tower_sessions::Session;

use crate::{
    guess::{Guess1Template, Guess2Template},
    AppError, User, GUESSES_KEY,
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    guesses: String,
}

pub async fn index(
    State(user): State<Arc<User>>,
    session: Session,
) -> Result<Html<String>, AppError> {
    let guesses: Vec<String> = session.get(GUESSES_KEY).await?.unwrap_or_default();

    let mut content = String::new();
    for (i, _) in guesses.iter().enumerate() {
        match i + 1 {
            1 => {
                content.push_str(
                    &Guess1Template {
                        color: &user.color,
                        name_placeholder: &user.name.chars().map(|_| "*").collect::<String>(),
                    }
                    .render()?,
                );
            }
            2 => {
                content.push_str(
                    &Guess2Template {
                        color: &user.color,
                        name_placeholder: &user.name.chars().map(|_| "*").collect::<String>(),
                        message: &user.message,
                    }
                    .render()?,
                );
            }
            g => return Err(AppError(anyhow!("guess {} is not implemented", g))),
        }
    }

    Ok(Html(IndexTemplate { guesses: content }.render()?))
}
