//! Authentication and authorization module
//!
//! This module provides user authentication and role-based access control (RBAC).

pub mod rbac;
pub mod jwt;

use std::collections::HashMap;
use tera_common::error::{Error, Result};

/// User information
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
}

/// Authentication service
pub struct AuthService {
    users: HashMap<String, User>,
    jwt_handler: jwt::JwtHandler,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(secret_key: &str) -> Self {
        AuthService {
            users: HashMap::new(),
            jwt_handler: jwt::JwtHandler::new(secret_key),
        }
    }

    /// Authenticate a user with username and password
    pub fn authenticate(&self, username: &str, password: &str) -> Result<String> {
        // TODO: Verify credentials against database
        // For now, accept any username/password

        let user = self.users.get(username)
            .ok_or_else(|| Error::Auth("User not found".to_string()))?;

        // TODO: Verify password hash

        // Generate JWT token
        self.jwt_handler.generate_token(&user.id, &user.username, &user.roles)
    }

    /// Validate a JWT token
    pub fn validate_token(&self, token: &str) -> Result<User> {
        let claims = self.jwt_handler.validate_token(token)?;

        let user = self.users.get(&claims.username)
            .ok_or_else(|| Error::Auth("User not found".to_string()))?;

        Ok(user.clone())
    }

    /// Register a new user
    pub fn register(&mut self, username: &str, email: &str, roles: Vec<String>) -> Result<()> {
        if self.users.contains_key(username) {
            return Err(Error::Auth("User already exists".to_string()));
        }

        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            username: username.to_string(),
            email: email.to_string(),
            roles,
        };

        self.users.insert(username.to_string(), user);
        Ok(())
    }
}
