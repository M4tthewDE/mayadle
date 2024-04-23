use anyhow::Result;
use tower_sessions::Session;
use tower_sessions_sqlx_store::{sqlx::SqlitePool, SqliteStore};

const GUESSES_KEY: &str = "guesses";

pub async fn get_store() -> Result<SqliteStore> {
    let pool = SqlitePool::connect("sqlite://sessions.db").await?;
    let session_store = SqliteStore::new(pool);
    session_store.migrate().await?;
    Ok(session_store)
}

pub async fn get_guesses(s: &Session) -> Result<Vec<String>> {
    Ok(s.get(GUESSES_KEY).await?.unwrap_or_default())
}

pub async fn save_guesses(s: &Session, guesses: Vec<String>) -> Result<()> {
    Ok(s.insert(GUESSES_KEY, guesses).await?)
}
