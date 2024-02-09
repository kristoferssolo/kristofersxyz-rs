use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Template)]
#[template(path = "project.html")]
pub struct Project {
    title: String,
    text: String,
    url: Option<String>,
    datetime: NaiveDateTime,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Projects {
    projects: Vec<Project>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateProject {
    title: String,
    text: String,
    url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProject {
    title: Option<String>,
    text: Option<String>,
    url: Option<String>,
}

pub async fn add_project(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProject>,
) -> impl IntoResponse {
    let query = sqlx::query!(
        r#"
        INSERT INTO project (title, text, url)
          VALUES ($1, $2, $3) RETURNING title
        "#,
        payload.title,
        payload.text,
        payload.url,
    )
    .fetch_one(&state.pool)
    .await;

    match query {
        Ok(project) => Ok((StatusCode::CREATED, project.title.to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn list_projects(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let query = sqlx::query_as!(
        Project,
        r#"
        SELECT title, text, url, datetime
          FROM project
        "#
    )
    .fetch_all(&state.pool)
    .await;

    match query {
        Ok(projects) => Ok((StatusCode::OK, Projects { projects })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_project(
    State(state): State<Arc<AppState>>,
    Path(title): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let query = sqlx::query_as!(
        Project,
        r#"
        SELECT title, text, url, datetime
          FROM project
          WHERE title = $1
        "#,
        title
    )
    .fetch_one(&state.pool)
    .await;
    match query {
        Ok(project) => Ok((StatusCode::OK, project)),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

pub async fn delete_project(
    State(state): State<Arc<AppState>>,
    Path(title): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let query = sqlx::query!(
        r#"
        DELETE
          FROM project
          WHERE title = $1
        "#,
        title
    )
    .execute(&state.pool)
    .await;
    match query {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn edit_project(
    State(state): State<Arc<AppState>>,
    Path(title): Path<String>,
    Json(project): Json<UpdateProject>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let query = sqlx::query!(
        r#"
        UPDATE project
          SET title = $1, text = $2, url = $3
          WHERE title = $4
        "#,
        project.title,
        project.text,
        project.url,
        title
    )
    .execute(&state.pool)
    .await;

    match query {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
