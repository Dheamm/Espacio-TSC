use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, Json};
use serde::Serialize;

use crate::{errors::AppResult, models::{MoodOption, mood_options}, state::AppState};

#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
    pub mood_options: Vec<MoodOption>,
}

pub async fn home_page() -> impl IntoResponse {
    HomeTemplate {
        mood_options: mood_options(),
    }
}

#[derive(Serialize, sqlx::FromRow)]
pub struct PostPreview {
    pub alias: String,
    pub content: String,
}

pub async fn forum_preview(State(state): State<AppState>) -> AppResult<Json<Vec<PostPreview>>> {
    let posts: Vec<PostPreview> = sqlx::query_as(
        "SELECT alias, content FROM posts ORDER BY RANDOM() LIMIT 10"
    )
    .fetch_all(&state.db)
    .await?;

    if posts.is_empty() {
        return Ok(Json(vec![]));
    }

    let mut result = Vec::with_capacity(10);
    let mut i = 0;
    while result.len() < 10 {
        let p = &posts[i % posts.len()];
        result.push(PostPreview {
            alias: p.alias.clone(),
            content: p.content.clone(),
        });
        i += 1;
    }

    Ok(Json(result))
}
