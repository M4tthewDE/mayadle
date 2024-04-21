use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};
use tracing::debug;

const GUESSES_KEY: &str = "guesses";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let session_store = MemoryStore::default();

    // TODO: generate daily
    let user = User {
        color: "#00FF7F".to_string(),
        name: "matthewde".to_string(),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/guess", post(guess))
        .nest_service("/static", ServeDir::new("static"))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .layer(SessionManagerLayer::new(session_store))
        .with_state(Arc::new(user));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

struct User {
    color: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Guess {
    name: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn root() -> Result<Html<String>, AppError> {
    Ok(Html(IndexTemplate {}.render()?))
}

#[derive(Template)]
#[template(path = "guess1.html")]
struct Guess1Template<'a> {
    color: &'a str,
    name_placeholder: &'a str,
}

async fn guess(
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

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
