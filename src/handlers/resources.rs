use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Query, State},
    Form,
};
use serde::Deserialize;

use crate::{errors::AppResult, models::Recurso, state::AppState};

#[derive(Template)]
#[template(path = "resources/page.html")]
pub struct ResourcesPageTemplate {
    pub recursos: Vec<Recurso>,
    pub categories: Vec<(String, bool)>,
    pub query: String,
    pub no_category_selected: bool,
}

#[derive(Template)]
#[template(path = "resources/list_partial.html")]
pub struct ResourcesListPartialTemplate {
    pub recursos: Vec<Recurso>,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub category: Option<String>,
}

async fn fetch_recursos(
    db: &sqlx::SqlitePool,
    q: Option<&str>,
    category: Option<&str>,
) -> AppResult<Vec<Recurso>> {
    let rows = match (q, category) {
        (Some(s), Some(c)) => {
            let pattern = format!("%{}%", s.to_lowercase());
            sqlx::query_as(
                "SELECT id, title, description, category, file_url, created_at FROM recursos WHERE LOWER(title) LIKE ? AND category = ? ORDER BY title ASC"
            )
            .bind(pattern).bind(c).fetch_all(db).await?
        }
        (Some(s), None) => {
            let pattern = format!("%{}%", s.to_lowercase());
            sqlx::query_as(
                "SELECT id, title, description, category, file_url, created_at FROM recursos WHERE LOWER(title) LIKE ? ORDER BY title ASC"
            )
            .bind(pattern).fetch_all(db).await?
        }
        (None, Some(c)) => {
            sqlx::query_as(
                "SELECT id, title, description, category, file_url, created_at FROM recursos WHERE category = ? ORDER BY title ASC"
            )
            .bind(c).fetch_all(db).await?
        }
        (None, None) => {
            sqlx::query_as(
                "SELECT id, title, description, category, file_url, created_at FROM recursos ORDER BY title ASC"
            )
            .fetch_all(db).await?
        }
    };
    Ok(rows)
}

async fn fetch_categories(db: &sqlx::SqlitePool) -> AppResult<Vec<String>> {
    let rows: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT category FROM recursos ORDER BY category ASC"
    )
    .fetch_all(db)
    .await?;
    Ok(rows)
}

pub async fn resources_page(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> AppResult<impl IntoResponse> {
    let recursos = fetch_recursos(&state.db, params.q.as_deref(), params.category.as_deref()).await?;
    let raw_categories = fetch_categories(&state.db).await?;
    let no_category_selected = params.category.is_none();
    let categories = raw_categories
        .into_iter()
        .map(|c| {
            let selected = params.category.as_deref() == Some(c.as_str());
            (c, selected)
        })
        .collect();
    Ok(ResourcesPageTemplate {
        recursos,
        categories,
        query: params.q.unwrap_or_default(),
        no_category_selected,
    })
}

pub async fn search_resources(
    State(state): State<AppState>,
    Form(form): Form<SearchQuery>,
) -> AppResult<impl IntoResponse> {
    let category = form.category.as_deref().filter(|c| !c.is_empty());
    let q = form.q.as_deref().filter(|s| !s.is_empty());
    let recursos = fetch_recursos(&state.db, q, category).await?;
    Ok(ResourcesListPartialTemplate { recursos })
}