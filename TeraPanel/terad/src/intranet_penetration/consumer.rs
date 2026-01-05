//! Tunnel consumer - connects to tunnels on other nodes

use tera_common::error::{Error, Result};
use std::collections::HashMap;
use tokio::net::{TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Connection information
#[derive(Debug, Clone)]
pub struct Connection {
    pub id: String,
    pub remote_node: String,
    pub remote_addr: String,
    pub local_addr: String,
}

/// Tunnel consumer
pub struct Consumer {
    server_url: String,
    node_id: String,
    connections: HashMap<String, Connection>,
}

impl Consumer {
    /// Create a new tunnel consumer
    pub fn new(server_url: &str, node_id: &str) -> Self {
        Consumer {
            server_url: server_url.to_string(),
            node_id: node_id.to_string(),
            connections: HashMap::new(),
        }
    }

    /// Start the tunnel consumer
    pub async fn start(&self) -> Result<()> {
        // TODO: Connect to the server
        // - Register this node with the server
        // - Listen for tunnel creation requests
        // - Create connections when tunnels are created
        Ok(())
    }

    /// Stop the tunnel consumer
    pub async fn stop(&self) -> Result<()> {
        // TODO: Close all connections
        for connection in self.connections.values() {
            self.close_connection(&connection.id).await.ok();
        }
        Ok(())
    }

    /// Connect to a remote tunnel
    pub async fn connect_to_tunnel(
        &self,
        remote_node: &str,
        remote_addr: &str,
        local_addr: &str,
    ) -> Result<String> {
        let connection_id = uuid::Uuid::new_v4().to_string();

        // TODO: Establish the connection
        // - Connect to the remote node's tunnel
        // - Set up local forwarding
        // - Store connection information

        let connection = Connection {
            id: connection_id.clone(),
            remote_node: remote_node.to_string(),
            remote_addr: remote_addr.to_string(),
            local_addr: local_addr.to_string(),
        };

        // In a real implementation, insert into self.connections
        // self.connections.insert(connection_id.clone(), connection);

        Ok(connection_id)
    }

    /// Close a connection
    pub async fn close_connection(&self, connection_id: &str) -> Result<()> {
        // TODO: Close the connection
        // - Close the TCP connection
        // - Remove from connections map

        // self.connections.remove(connection_id);

        Ok(())
    }

    /// List all active connections
    pub fn list_connections(&self) -> Result<Vec<String>> {
        Ok(self.connections.keys().cloned().collect())
    }

    /// Handle a forwarded connection
    async fn handle_forward(
        &self,
        remote_stream: TcpStream,
        local_addr: &str,
    ) -> Result<()> {
        // TODO: Handle the forwarded connection
        // - Connect to the local service
        // - Bidirectional forwarding between remote and local

        let mut buffer = [0; 4096];
        loop {
            let n = remote_stream.read(&mut buffer).await
                .map_err(Error::Io)?;

            if n == 0 {
                break;
            }

            // TODO: Forward to local service
            // let local_stream = TcpStream::connect(local_addr).await?;
            // local_stream.write_all(&buffer[..n]).await?;

            // TODO: Read response and send back to remote
        }

        Ok(())
    }
}
