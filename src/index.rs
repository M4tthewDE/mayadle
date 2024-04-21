use askama::Template;
use axum::response::Html;

use crate::AppError;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub async fn index() -> Result<Html<String>, AppError> {
    Ok(Html(IndexTemplate {}.render()?))
}
