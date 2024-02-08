use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateProject {
    title: String,
    text: String,
    url: Option<String>,
}

pub async fn add_project(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProject>,
) -> impl IntoResponse {
    let query = sqlx::query!(
        r#"
        INSERT INTO "project" (title, text, url)
          VALUES ($1, $2, $3)
        "#,
        payload.title,
        payload.text,
        payload.url,
    )
    .execute(&state.pool)
    .await;

    match query {
        Ok(_) => (StatusCode::CREATED, "".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Project {
    title: String,
    text: String,
    url: Option<String>,
    datetime: NaiveDateTime,
}
pub async fn list_projects(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let query = sqlx::query_as!(
        Project,
        r#"
        SELECT title, text, url, datetime
          FROM "project"
        "#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or(vec![]);
    Json(query)
}
