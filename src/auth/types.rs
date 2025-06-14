use serde::{Deserialize, Serialize};
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;
use async_trait::async_trait;
use jsonwebtoken::jwk::JwkSet;
use tracing::{error, warn};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,           // Auth0 user ID
    pub exp: i64,           // Expiration
    pub aud: String,          // Audience
    pub iss: String,          // Issuer
    pub iat: i64,
    pub azp: String,
    pub scope: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AuthState {
    pub jwks_client: JwkSet,
    pub audience: String,
    pub issuer: String,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
} 