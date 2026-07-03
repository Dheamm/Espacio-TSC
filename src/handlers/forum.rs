use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, Form};
use serde::Deserialize;
use sqlx::Row;

use crate::{
    alias::generate_alias,
    errors::{AppError, AppResult},
    models::{Post, PostWithReplies},
    state::AppState,
};

#[derive(Template)]
#[template(path = "forum/page.html")]
pub struct ForumPageTemplate {
    pub threads: Vec<PostWithReplies>,
}

#[derive(Template)]
#[template(path = "forum/thread_item.html")]
pub struct ThreadItemTemplate {
    pub thread: PostWithReplies,
}

#[derive(Template)]
#[template(path = "forum/reply_item.html")]
pub struct ReplyItemTemplate {
    pub reply: Post,
}

#[derive(Deserialize)]
pub struct NewPostForm {
    pub content: String,
}

#[derive(Deserialize)]
pub struct NewReplyForm {
    pub content: String,
    pub parent_id: i64,
}

async fn fetch_threads(db: &sqlx::SqlitePool) -> AppResult<Vec<PostWithReplies>> {
    let roots: Vec<Post> = sqlx::query_as(
        "SELECT id, alias, content, parent_id, created_at FROM posts WHERE parent_id IS NULL ORDER BY created_at DESC"
    )
    .fetch_all(db)
    .await?;

    let mut threads = Vec::with_capacity(roots.len());

    for post in roots {
        let replies: Vec<Post> = sqlx::query_as(
            "SELECT id, alias, content, parent_id, created_at FROM posts WHERE parent_id = ? ORDER BY created_at ASC"
        )
        .bind(post.id)
        .fetch_all(db)
        .await?;

        threads.push(PostWithReplies { post, replies });
    }

    Ok(threads)
}

pub async fn forum_page(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    let threads = fetch_threads(&state.db).await?;
    Ok(ForumPageTemplate { threads })
}

pub async fn create_post(
    State(state): State<AppState>,
    Form(form): Form<NewPostForm>,
) -> AppResult<impl IntoResponse> {
    let content = form.content.trim().to_string();

    if content.is_empty() || content.len() > 1000 {
        return Err(AppError::BadRequest(
            "El contenido debe tener entre 1 y 1000 caracteres".to_string(),
        ));
    }

    let alias = generate_alias();

    let row = sqlx::query(
        "INSERT INTO posts (alias, content, parent_id) VALUES (?, ?, NULL) RETURNING id"
    )
    .bind(&alias)
    .bind(&content)
    .fetch_one(&state.db)
    .await?;

    let id: i64 = row.try_get("id")?;

    let post: Post = sqlx::query_as(
        "SELECT id, alias, content, parent_id, created_at FROM posts WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    Ok(ThreadItemTemplate {
        thread: PostWithReplies { post, replies: vec![] },
    })
}

pub async fn create_reply(
    State(state): State<AppState>,
    Form(form): Form<NewReplyForm>,
) -> AppResult<impl IntoResponse> {
    let content = form.content.trim().to_string();

    if content.is_empty() || content.len() > 1000 {
        return Err(AppError::BadRequest(
            "El contenido debe tener entre 1 y 1000 caracteres".to_string(),
        ));
    }

    let parent_exists: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM posts WHERE id = ? AND parent_id IS NULL"
    )
    .bind(form.parent_id)
    .fetch_optional(&state.db)
    .await?;

    if parent_exists.is_none() {
        return Err(AppError::NotFound);
    }

    let alias = generate_alias();

    let row = sqlx::query(
        "INSERT INTO posts (alias, content, parent_id) VALUES (?, ?, ?) RETURNING id"
    )
    .bind(&alias)
    .bind(&content)
    .bind(form.parent_id)
    .fetch_one(&state.db)
    .await?;

    let id: i64 = row.try_get("id")?;

    let reply: Post = sqlx::query_as(
        "SELECT id, alias, content, parent_id, created_at FROM posts WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    Ok(ReplyItemTemplate { reply })
}