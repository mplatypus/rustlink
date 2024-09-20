use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

use super::{app::App, error::{AppError, AuthError}};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Authentication {}


#[async_trait::async_trait]
impl FromRequestParts<App> for Authentication {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &App) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingCredentials)?;

        if bearer.token().to_string() == state.lock().await.config.password.expose_secret() {
            return Ok(Authentication {});
        } else {
            return Err(AppError::Authentication(AuthError::InvalidToken));
        }
    }
}