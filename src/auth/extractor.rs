use axum::{
    extract::{Extension, FromRequestParts},
    http::request::Parts,
};
use crate::auth::types::{AuthUser, Claims};
use crate::error::AppError;


impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Extension(claims) = Extension::<Claims>::from_request_parts(parts, _state)
            .await
            .map_err(|_| AppError::MissingToken)?;
        
        Ok(AuthUser {
            id: claims.sub.clone(),
            permissions: claims.permissions.clone(),
        })
    }
} 