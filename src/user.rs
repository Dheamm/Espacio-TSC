use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{alias::generate_alias, models::User, state::AppState};

const COOKIE_NAME: &str = "uid";
const COOKIE_MAX_AGE_SECONDS: i64 = 60 * 60 * 24 * 365;

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: String,
    pub alias: String,
}

fn extract_cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    let raw = headers.get(header::COOKIE)?.to_str().ok()?;
    raw.split(';').find_map(|pair| {
        let (key, value) = pair.trim().split_once('=')?;
        if key == name {
            Some(value.to_string())
        } else {
            None
        }
    })
}

fn build_cookie_header(value: &str) -> HeaderValue {
    let cookie = format!(
        "{}={}; Path=/; Max-Age={}; HttpOnly; SameSite=Lax",
        COOKIE_NAME, value, COOKIE_MAX_AGE_SECONDS
    );
    HeaderValue::from_str(&cookie).expect("la cookie generada debe ser ASCII valido")
}

async fn load_existing_user(state: &AppState, id: &str) -> Option<CurrentUser> {
    let user: Option<User> = sqlx::query_as("SELECT id, alias, created_at FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .ok()?;

    user.map(|u| CurrentUser { id: u.id, alias: u.alias })
}

async fn create_user(state: &AppState) -> Result<CurrentUser, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let alias = generate_alias();

    sqlx::query("INSERT INTO users (id, alias) VALUES (?, ?)")
        .bind(&id)
        .bind(&alias)
        .execute(&state.db)
        .await?;

    Ok(CurrentUser { id, alias })
}

fn internal_error_response() -> Response {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("Error interno del servidor"))
        .expect("la respuesta de error interno debe construirse correctamente")
}

pub async fn resolve_user(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let existing_id = extract_cookie_value(request.headers(), COOKIE_NAME);

    let existing_user = match &existing_id {
        Some(id) => load_existing_user(&state, id).await,
        None => None,
    };

    let (current_user, needs_cookie) = match existing_user {
        Some(user) => (user, false),
        None => match create_user(&state).await {
            Ok(user) => (user, true),
            Err(err) => {
                tracing::error!("No se pudo crear un usuario anonimo: {}", err);
                return internal_error_response();
            }
        },
    };

    let cookie_value = current_user.id.clone();
    request.extensions_mut().insert(current_user);

    let mut response = next.run(request).await;

    if needs_cookie {
        response
            .headers_mut()
            .insert(header::SET_COOKIE, build_cookie_header(&cookie_value));
    }

    response
}
