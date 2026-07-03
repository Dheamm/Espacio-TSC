use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Extension, State},
    Form,
};
use serde::Deserialize;

use crate::{
    errors::{AppError, AppResult},
    models::{mood_options, MoodCheckin},
    state::AppState,
    user::CurrentUser,
};

#[derive(Deserialize)]
pub struct MoodForm {
    pub mood: String,
}

#[derive(Template)]
#[template(path = "mood/confirmation.html")]
pub struct MoodConfirmationTemplate {
    pub icon: String,
    pub label: String,
}

pub struct MoodCheckinView {
    pub label: String,
    pub icon: String,
    pub created_at: String,
}

pub struct MoodSummary {
    pub label: String,
    pub icon: String,
    pub count: i64,
    pub percentage: i64,
}

#[derive(Template)]
#[template(path = "mood/historial.html")]
pub struct MoodHistoryTemplate {
    pub checkins: Vec<MoodCheckinView>,
    pub summary: Vec<MoodSummary>,
}

pub async fn submit_mood(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Form(form): Form<MoodForm>,
) -> AppResult<impl IntoResponse> {
    let mood = form.mood.trim().to_string();

    if mood.is_empty() {
        return Err(AppError::BadRequest("Estado de ánimo inválido".to_string()));
    }

    let option = mood_options()
        .into_iter()
        .find(|m| m.value == mood)
        .ok_or_else(|| AppError::BadRequest("Estado inválido".to_string()))?;

    sqlx::query("INSERT INTO mood_checkins (user_id, mood, emoji) VALUES (?, ?, ?)")
        .bind(&current_user.id)
        .bind(&mood)
        .bind(option.icon)
        .execute(&state.db)
        .await?;

    Ok(MoodConfirmationTemplate {
        icon: option.icon.to_string(),
        label: option.label.to_string(),
    })
}

pub async fn history_page(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResult<impl IntoResponse> {
    let checkins: Vec<MoodCheckin> = sqlx::query_as(
        "SELECT id, user_id, mood, emoji, created_at FROM mood_checkins WHERE user_id = ? ORDER BY created_at DESC",
    )
    .bind(&current_user.id)
    .fetch_all(&state.db)
    .await?;

    let options = mood_options();
    let total = checkins.len() as i64;

    let checkin_views = checkins
        .iter()
        .map(|checkin| {
            let label = options
                .iter()
                .find(|option| option.value == checkin.mood)
                .map(|option| option.label.to_string())
                .unwrap_or_else(|| checkin.mood.clone());

            MoodCheckinView {
                label,
                icon: checkin.emoji.clone(),
                created_at: checkin.created_at.clone(),
            }
        })
        .collect();

    let summary = options
        .into_iter()
        .filter_map(|option| {
            let count = checkins.iter().filter(|c| c.mood == option.value).count() as i64;

            if count == 0 {
                return None;
            }

            let percentage = (count * 100) / total.max(1);

            Some(MoodSummary {
                label: option.label.to_string(),
                icon: option.icon.to_string(),
                count,
                percentage,
            })
        })
        .collect();

    Ok(MoodHistoryTemplate {
        checkins: checkin_views,
        summary,
    })
}
