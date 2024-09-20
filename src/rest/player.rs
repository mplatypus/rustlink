use axum::{
    extract::{DefaultBodyLimit, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post},
    Json, Router,
};

use crate::models::{app::App, error::AppError};

pub fn generate_router() -> Router<App> {
    Router::new()
        .route("/sessions/:session_id/players", get(get_players))
        .route("/sessions/:session_id/players/:guild_id", get(get_player))
        .route("/sessions/:session_id/players/:guild_id", post(create_player))
        .route("/sessions/:session_id/players/:guild_id", patch(edit_player))
        .route("/sessions/:session_id/players/:guild_id", delete(delete_player))
        .layer(DefaultBodyLimit::disable())
}

async fn get_players(
    Path(session_id): Path<String>,
    State(app): State<App>) -> Response {
    let players = app.lock().await.get_players(session_id);

    if players.len() == 0 {
        return StatusCode::NOT_FOUND.into_response()
    }

    (StatusCode::ACCEPTED, Json(players)).into_response()
}

async fn get_player(
    Path((session_id, guild_id)): Path<(String, String)>,
    State(app): State<App>) -> Result<Response, AppError> {
    let mut mapp = app.lock().await;

    let player = mapp.get_player(session_id, guild_id);

    if let Some(p) = player {
        return Ok((StatusCode::ACCEPTED, Json(p)).into_response());
    }

    Err(AppError::NotFound(String::from("No player was found.")))
}

async fn create_player(
    Path((session_id, guild_id)): Path<(String, String)>,
    State(app): State<App>) -> Result<Response, AppError> {
    let player = app.lock().await.new_player(session_id, guild_id)?.clone();

    Ok((StatusCode::ACCEPTED, Json(player)).into_response())
}

async fn edit_player(
    Path((session_id, guild_id)): Path<(String, String)>,
    State(app): State<App>) -> Result<Response, AppError> {
    let player = app.lock().await.new_player(session_id, guild_id)?.clone();

    Ok((StatusCode::ACCEPTED, Json(player)).into_response())
}

async fn delete_player(
    Path((session_id, guild_id)): Path<(String, String)>,
    State(app): State<App>) -> Result<Response, AppError> {
    let player = app.lock().await.delete_player(session_id, guild_id).await?.clone();

    Ok((StatusCode::ACCEPTED, Json(player)).into_response())
}