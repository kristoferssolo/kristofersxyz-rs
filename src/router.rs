use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    project::{add_project, delete_project, edit_project, get_project, list_projects},
    state::AppState,
};

pub fn init_router(state: Arc<AppState>) -> Router {
    let router = Router::new();
    router
        .route("/", get(list_projects))
        .route("/add", post(add_project))
        .route(
            "/projects/:title",
            get(get_project).delete(delete_project).patch(edit_project),
        )
        .with_state(state)
}
