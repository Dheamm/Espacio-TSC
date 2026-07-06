use std::collections::HashMap;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Form,
};
use serde::Deserialize;
use sqlx::Row;

use crate::{
    errors::{AppError, AppResult},
    models::{Post, PostView, ThreadView},
    state::AppState,
    user::CurrentUser,
};

#[derive(Template)]
#[template(path = "forum/page.html")]
pub struct ForumPageTemplate {
    pub threads: Vec<ThreadView>,
}

#[derive(Template)]
#[template(path = "forum/my_posts.html")]
pub struct MyPostsTemplate {
    pub threads: Vec<ThreadView>,
}

#[derive(Template)]
#[template(path = "forum/thread_item.html")]
pub struct ThreadItemTemplate {
    pub thread: ThreadView,
}

#[derive(Template)]
#[template(path = "forum/reply_item.html")]
pub struct ReplyItemTemplate {
    pub reply: PostView,
}

#[derive(Template)]
#[template(path = "forum/post_body.html")]
pub struct PostBodyTemplate {
    pub post: PostView,
    pub is_reply: bool,
}

#[derive(Template)]
#[template(path = "forum/edit_form.html")]
pub struct EditFormTemplate {
    pub id: i64,
    pub content: String,
    pub is_reply: bool,
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

#[derive(Deserialize)]
pub struct EditPostForm {
    pub content: String,
}

fn validate_content(content: &str) -> AppResult<String> {
    let trimmed = content.trim().to_string();

    if trimmed.is_empty() || trimmed.len() > 1000 {
        return Err(AppError::BadRequest(
            "El contenido debe tener entre 1 y 1000 caracteres".to_string(),
        ));
    }

    Ok(trimmed)
}

async fn fetch_post(db: &sqlx::SqlitePool, id: i64) -> AppResult<Post> {
    let post: Option<Post> = sqlx::query_as(
        "SELECT id, user_id, alias, content, parent_id, created_at, edited_at, previous_content FROM posts WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    post.ok_or(AppError::NotFound)
}

async fn fetch_all_posts(db: &sqlx::SqlitePool) -> AppResult<Vec<Post>> {
    let posts: Vec<Post> = sqlx::query_as(
        "SELECT id, user_id, alias, content, parent_id, created_at, edited_at, previous_content FROM posts ORDER BY created_at ASC",
    )
    .fetch_all(db)
    .await?;

    Ok(posts)
}

fn build_threads(posts: Vec<Post>, current_user_id: &str) -> Vec<ThreadView> {
    let mut roots = Vec::new();
    let mut replies_by_parent: HashMap<i64, Vec<Post>> = HashMap::new();

    for post in posts {
        match post.parent_id {
            Some(parent_id) => replies_by_parent.entry(parent_id).or_default().push(post),
            None => roots.push(post),
        }
    }

    roots.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    roots
        .into_iter()
        .map(|root| {
            let replies = replies_by_parent
                .remove(&root.id)
                .unwrap_or_default()
                .into_iter()
                .map(|reply| PostView::from_post(reply, current_user_id))
                .collect();

            ThreadView {
                post: PostView::from_post(root, current_user_id),
                replies,
            }
        })
        .collect()
}

pub async fn forum_page(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResult<impl IntoResponse> {
    let posts = fetch_all_posts(&state.db).await?;
    let threads = build_threads(posts, &current_user.id);
    Ok(ForumPageTemplate { threads })
}

pub async fn my_posts_page(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResult<impl IntoResponse> {
    let posts = fetch_all_posts(&state.db).await?;
    let threads: Vec<ThreadView> = build_threads(posts, &current_user.id)
        .into_iter()
        .filter(|thread| thread.post.is_owner || thread.replies.iter().any(|r| r.is_owner))
        .collect();

    Ok(MyPostsTemplate { threads })
}

pub async fn create_post(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Form(form): Form<NewPostForm>,
) -> AppResult<impl IntoResponse> {
    let content = validate_content(&form.content)?;

    let row = sqlx::query(
        "INSERT INTO posts (user_id, alias, content, parent_id) VALUES (?, ?, ?, NULL) RETURNING id",
    )
    .bind(&current_user.id)
    .bind(&current_user.alias)
    .bind(&content)
    .fetch_one(&state.db)
    .await?;

    let id: i64 = row.try_get("id")?;
    let post = fetch_post(&state.db, id).await?;

    Ok(ThreadItemTemplate {
        thread: ThreadView {
            post: PostView::from_post(post, &current_user.id),
            replies: vec![],
        },
    })
}

pub async fn create_reply(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Form(form): Form<NewReplyForm>,
) -> AppResult<impl IntoResponse> {
    let content = validate_content(&form.content)?;

    let parent_exists: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM posts WHERE id = ? AND parent_id IS NULL",
    )
    .bind(form.parent_id)
    .fetch_optional(&state.db)
    .await?;

    if parent_exists.is_none() {
        return Err(AppError::NotFound);
    }

    let row = sqlx::query(
        "INSERT INTO posts (user_id, alias, content, parent_id) VALUES (?, ?, ?, ?) RETURNING id",
    )
    .bind(&current_user.id)
    .bind(&current_user.alias)
    .bind(&content)
    .bind(form.parent_id)
    .fetch_one(&state.db)
    .await?;

    let id: i64 = row.try_get("id")?;
    let reply = fetch_post(&state.db, id).await?;

    Ok(ReplyItemTemplate {
        reply: PostView::from_post(reply, &current_user.id),
    })
}

pub async fn view_post(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    let post = fetch_post(&state.db, id).await?;
    let is_reply = post.parent_id.is_some();

    Ok(PostBodyTemplate {
        post: PostView::from_post(post, &current_user.id),
        is_reply,
    })
}

pub async fn edit_form(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    let post = fetch_post(&state.db, id).await?;

    if post.user_id != current_user.id {
        return Err(AppError::Forbidden);
    }

    Ok(EditFormTemplate {
        id: post.id,
        content: post.content,
        is_reply: post.parent_id.is_some(),
    })
}

pub async fn edit_post(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
    Form(form): Form<EditPostForm>,
) -> AppResult<impl IntoResponse> {
    let content = validate_content(&form.content)?;
    let post = fetch_post(&state.db, id).await?;

    if post.user_id != current_user.id {
        return Err(AppError::Forbidden);
    }

    if content != post.content {
        sqlx::query(
            "UPDATE posts SET content = ?, previous_content = ?, edited_at = datetime('now') WHERE id = ?",
        )
        .bind(&content)
        .bind(&post.content)
        .bind(id)
        .execute(&state.db)
        .await?;
    }

    let updated = fetch_post(&state.db, id).await?;
    let is_reply = updated.parent_id.is_some();

    Ok(PostBodyTemplate {
        post: PostView::from_post(updated, &current_user.id),
        is_reply,
    })
}

pub async fn delete_post(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
) -> AppResult<StatusCode> {
    let post = fetch_post(&state.db, id).await?;

    if post.user_id != current_user.id {
        return Err(AppError::Forbidden);
    }

    sqlx::query("DELETE FROM posts WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::OK)
}
