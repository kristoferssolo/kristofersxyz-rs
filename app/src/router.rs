use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    project::{add_project, delete_project, get_project, list_projects},
    state::AppState,
};

pub fn init_router(state: Arc<AppState>) -> Router {
    let router = Router::new();
    router
        .route("/", get(|| async { "Hello there" }))
        .route("/projects", get(list_projects).post(add_project))
        .route("/projects/:title", get(get_project).delete(delete_project))
        .with_state(state)
}
