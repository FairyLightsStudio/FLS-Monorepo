//! JWT token handling

use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, DecodingKey, Algorithm, Header, Validation};
use serde::{Deserialize, Serialize};
use tera_common::error::{Error, Result};

/// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user ID
    pub username: String,
    pub roles: Vec<String>,
    pub exp: i64, // expiration timestamp
}

/// JWT token handler
pub struct JwtHandler {
    secret: Vec<u8>,
}

impl JwtHandler {
    /// Create a new JWT handler
    pub fn new(secret: &str) -> Self {
        JwtHandler {
            secret: secret.as_bytes().to_vec(),
        }
    }

    /// Generate a JWT token
    pub fn generate_token(&self, user_id: &str, username: &str, roles: &[String]) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // 24 hour expiration

        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            roles: roles.to_vec(),
            exp: exp.timestamp(),
        };

        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
        .map_err(|e| Error::Auth(format!("Failed to encode JWT: {}", e)))?;

        Ok(token)
    }

    /// Validate a JWT token
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let mut validation = Validation::default();
        validation.set_algorithm(&Algorithm::HS256);
        validation.set_required_spec_claims(&["sub", "exp", "username", "roles"]);

        let claims = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &validation,
        )
        .map_err(|e| Error::Auth(format!("Invalid JWT token: {}", e)))?;

        Ok(claims.claims)
    }
}
