use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::{
    handlers::{forum, home, mood, resources},
    state::AppState,
    user::resolve_user,
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(home::home_page))
        .route("/foro/preview", get(home::forum_preview))
        .route("/foro", get(forum::forum_page))
        .route("/foro/mis-publicaciones", get(forum::my_posts_page))
        .route("/foro/post", post(forum::create_post))
        .route("/foro/reply", post(forum::create_reply))
        .route("/foro/post/:id", get(forum::view_post))
        .route("/foro/post/:id/editar", get(forum::edit_form).post(forum::edit_post))
        .route("/foro/post/:id/eliminar", post(forum::delete_post))
        .route("/recursos", get(resources::resources_page))
        .route("/recursos/buscar", post(resources::search_resources))
        .route("/mood/submit", post(mood::submit_mood))
        .route("/mood/historial", get(mood::history_page))
        .nest_service("/icons", ServeDir::new("static/icons"))
        .layer(middleware::from_fn_with_state(state.clone(), resolve_user))
        .with_state(state)
}
