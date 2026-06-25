use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::{AppState, AppError};

// ============================================================================
// MODELS
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    user_id: String,   // user id
    role: String,   // user role
    iat: usize, // issued timestamp
    exp: usize,  // expiry timestamp
}

#[derive(Serialize, Clone)]
pub struct CurrentUser {
    pub id: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_in: i64,
}


// ============================================================================
// PASSWORD HASHING
// ============================================================================

pub fn hash_password(password: &str) -> String {
    use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
    let salt = SaltString::generate(rand::rngs::OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

// ============================================================================
// JWT
// ============================================================================

pub fn create_token(config: &AppState, user_id: &str, role: &str) -> Result<String, AppError> {
    let header = Header::default();
    let claims = TokenClaims {
        user_id: user_id.to_string(),
        role: role.to_string(),
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    };
    let key = EncodingKey::from_secret(config.jwt_secret.as_bytes());
    let token = encode(&header, &claims, &key)?;
    Ok(token)
}

pub fn verify_token(config: &AppState, token: &str) -> Result<TokenClaims, AppError> {
    let key = DecodingKey::from_secret(config.jwt_secret.as_bytes());
    let validation = Validation::default();
    let token_data = decode::<TokenClaims>(token, &key, &validation)?;
    let claims = token_data.claims;
    Ok(claims)
}


// ============================================================================
// AUTH MIDDLEWARE
// ============================================================================

pub async fn auth_middleware(
    State(config): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let token = auth_header.ok_or(AppError::unauthorized("Invalid token"))?;
    let claims = verify_token(&config, token)?;

    let user = CurrentUser {
        id: claims.user_id,
        role: claims.role,
    };
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}