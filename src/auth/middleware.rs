use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation, decode_header, jwk::JwkSet};
use reqwest;
use tracing::{error, warn};
use crate::AppState;
use crate::auth::types::Claims;
use crate::error::AppError;

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::MissingToken)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::InvalidToken);
    }

    let token = auth_header.trim_start_matches("Bearer ").to_string();

    // Get the key ID from the token header
    let header = decode_header(&token)
        .map_err(|e| {
            error!("Failed to decode token header: {}", e);
            AppError::Internal(format!("Token header decode error: {}", e))
        })?;
    let kid = header.kid.ok_or(AppError::InvalidToken)?;

    // Fetch JWKS from Auth0
    let jwks_url = format!("https://{}/.well-known/jwks.json", state.env.auth0_domain);
    let jwks: JwkSet = reqwest::get(&jwks_url)
        .await
        .map_err(|e| {
            error!("Failed to fetch JWKS: {}", e);
            AppError::Internal(format!("JWKS fetch error: {}", e))
        })?
        .json()
        .await
        .map_err(|e| {
            error!("Failed to parse JWKS: {}", e);
            AppError::Internal(format!("JWKS parse error: {}", e))
        })?;

    // Find the key that matches the kid
    let jwk = jwks.find(&kid).ok_or_else(|| {
        warn!("No matching key found for kid: {}", kid);
        AppError::InvalidToken
    })?;
    let decoding_key = DecodingKey::from_jwk(jwk)
        .map_err(|e| {
            error!("Failed to create decoding key: {}", e);
            AppError::Internal(format!("Decoding key error: {}", e))
        })?;

    // Validate the token
    let mut validation = Validation::default();
    validation.set_issuer(&[format!("https://{}/", state.env.auth0_domain)]);
    validation.set_audience(&[state.env.auth0_audience.clone()]);

    let claims = decode::<Claims>(
        &token,
        &decoding_key,
        &validation,
    )
    .map_err(|e| {
        error!("Token validation failed: {}", e);
        match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::ExpiredToken,
            _ => AppError::Internal(format!("Token validation error: {}", e))
        }
    })?;

    request.extensions_mut().insert(claims.claims);
    Ok(next.run(request).await)
} 