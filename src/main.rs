use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::{sqlx::SqlitePool, SqliteStore};

mod guess;
mod index;

const GUESSES_KEY: &str = "guesses";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect("sqlite://sessions.db").await.unwrap();
    let session_store = SqliteStore::new(pool);
    session_store.migrate().await.unwrap();

    // TODO: generate daily
    let user = User {
        color: "#00FF7F".to_string(),
        name: "matthewde".to_string(),
        message: "link".to_string(),
    };

    let app = Router::new()
        .route("/", get(index::index))
        .route("/guess", post(guess::guess))
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
    message: String,
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
