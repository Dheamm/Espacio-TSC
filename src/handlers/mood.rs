use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, Form};
use serde::Deserialize;

use crate::{errors::{AppError, AppResult}, models::mood_options, state::AppState};

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

pub async fn submit_mood(
    State(state): State<AppState>,
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

    sqlx::query("INSERT INTO mood_checkins (mood, emoji) VALUES (?, ?)")
        .bind(&mood)
        .bind(option.icon)
        .execute(&state.db)
        .await?;

    Ok(MoodConfirmationTemplate {
        icon: option.icon.to_string(),
        label: option.label.to_string(),
    })
}
