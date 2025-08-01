use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::{Result, anyhow};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub email: String,
    pub is_admin: bool,
    pub exp: usize,
    pub iat: usize,
}

pub fn generate_jwt_token(user_id: Uuid, email: &str, is_admin: bool) -> Result<String> {
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| anyhow!("JWT_SECRET not found in environment"))?;

    let now = Utc::now();
    let exp = now + Duration::hours(24); // Token expires in 24 hours

    let claims = Claims {
        user_id,
        email: email.to_owned(),
        is_admin,
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| anyhow!("Failed to generate token: {}", e))
}

pub fn validate_jwt_token(token: &str) -> Result<Claims> {
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| anyhow!("JWT_SECRET not found in environment"))?;

    let validation = Validation::new(Algorithm::HS256);
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|e| anyhow!("Invalid token: {}", e))
}
