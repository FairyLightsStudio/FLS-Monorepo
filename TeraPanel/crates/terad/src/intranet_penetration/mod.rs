//! Intranet penetration module
//!
//! This module provides secure remote access to internal services through
//! a public endpoint, similar to tools like ngrok or frp.

pub mod provider;
pub mod consumer;

use tera_common::error::Result;

/// Tunnel configuration
#[derive(Debug, Clone)]
pub struct TunnelConfig {
    pub local_addr: String,
    pub remote_addr: String,
    pub protocol: String,
    pub auth_token: Option<String>,
}

/// Intranet penetration manager
pub struct IntranetPenetration {
    provider: provider::Provider,
    consumer: consumer::Consumer,
}

impl IntranetPenetration {
    /// Create a new intranet penetration manager
    pub fn new(server_url: &str, node_id: &str) -> Self {
        IntranetPenetration {
            provider: provider::Provider::new(),
            consumer: consumer::Consumer::new(server_url, node_id),
        }
    }

    /// Start the penetration service
    pub async fn start(&self) -> Result<()> {
        // TODO: Start the provider (for creating tunnels to this node)
        self.provider.start().await?;

        // TODO: Start the consumer (for connecting to other nodes)
        self.consumer.start().await?;

        Ok(())
    }

    /// Stop the penetration service
    pub async fn stop(&self) -> Result<()> {
        self.provider.stop().await?;
        self.consumer.stop().await?;
        Ok(())
    }

    /// Create a new tunnel
    pub async fn create_tunnel(&self, config: TunnelConfig) -> Result<String> {
        self.provider.create_tunnel(config).await
    }

    /// Close a tunnel
    pub async fn close_tunnel(&self, tunnel_id: &str) -> Result<()> {
        self.provider.close_tunnel(tunnel_id).await
    }

    /// List all active tunnels
    pub fn list_tunnels(&self) -> Result<Vec<String>> {
        self.provider.list_tunnels()
    }
}
