//! NATS messaging module
//!
//! This module handles communication with terad instances via NATS.

pub mod commands;
pub mod listeners;

use async_nats::Client;
use tera_common::error::Result;

/// NATS client wrapper
pub struct NatsClient {
    client: Client,
}

impl NatsClient {
    /// Create a new NATS client
    pub async fn new(server_url: &str) -> Result<Self> {
        let client = async_nats::connect(server_url)
            .await
            .map_err(tera_common::error::Error::Nats)?;

        Ok(NatsClient { client })
    }

    /// Get the underlying NATS client
    pub fn get_client(&self) -> &Client {
        &self.client
    }
}
