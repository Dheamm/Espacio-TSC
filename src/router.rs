use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::{
    handlers::{forum, home, mood, resources},
    state::AppState,
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(home::home_page))
        .route("/foro/preview", get(home::forum_preview))
        .route("/foro", get(forum::forum_page))
        .route("/foro/post", post(forum::create_post))
        .route("/foro/reply", post(forum::create_reply))
        .route("/recursos", get(resources::resources_page))
        .route("/recursos/buscar", post(resources::search_resources))
        .route("/mood/submit", post(mood::submit_mood))
        .nest_service("/icons", ServeDir::new("static/icons"))
        .with_state(state)
}
