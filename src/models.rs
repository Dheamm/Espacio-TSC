use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub alias: String,
    pub content: String,
    pub parent_id: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostWithReplies {
    pub post: Post,
    pub replies: Vec<Post>,
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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MoodCheckin {
    pub id: i64,
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
