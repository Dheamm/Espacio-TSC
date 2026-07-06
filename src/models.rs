use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub alias: String,
    pub created_at: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub user_id: String,
    pub alias: String,
    pub content: String,
    pub parent_id: Option<i64>,
    pub created_at: String,
    pub edited_at: Option<String>,
    pub previous_content: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PostView {
    pub id: i64,
    pub alias: String,
    pub content: String,
    pub created_at: String,
    pub is_owner: bool,
    pub is_edited: bool,
    pub edited_at: String,
    pub previous_content: String,
}


impl PostView {
    pub fn from_post(post: Post, current_user_id: &str) -> Self {
        let is_owner = post.user_id == current_user_id;
        let is_edited = post.edited_at.is_some();

        Self {
            id: post.id,
            alias: post.alias,
            content: post.content,
            created_at: post.created_at,
            is_owner,
            is_edited,
            edited_at: post.edited_at.unwrap_or_default(),
            previous_content: post.previous_content.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThreadView {
    pub post: PostView,
    pub replies: Vec<PostView>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Recurso {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub category: String,
    pub file_url: String,
    pub created_at: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MoodCheckin {
    pub id: i64,
    pub user_id: String,
    pub mood: String,
    pub emoji: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodOption {
    pub value: &'static str,
    pub icon: &'static str,
    pub label: &'static str,
}

pub fn mood_options() -> Vec<MoodOption> {
    vec![
        MoodOption { value: "agotado",    icon: "/icons/agotado.svg",    label: "Agotado/a" },
        MoodOption { value: "ansioso",    icon: "/icons/ansioso.svg",    label: "Ansioso/a" },
        MoodOption { value: "enojado",    icon: "/icons/enojado.svg",    label: "Enojado/a" },
        MoodOption { value: "estable",    icon: "/icons/estable.svg",    label: "Estable" },
        MoodOption { value: "motivado",   icon: "/icons/motivado.svg",   label: "Motivado/a" },
        MoodOption { value: "satisfecho", icon: "/icons/satisfecho.svg", label: "Satisfecho/a" },
    ]
}
