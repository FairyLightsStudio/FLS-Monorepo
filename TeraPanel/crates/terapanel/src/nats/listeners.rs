//! NATS message listeners
//!
//! Functions to listen for messages from terad instances.

use async_nats::{Client, Subscriber};
use futures::StreamExt;
use tera_common::{
    protocol::*,
    types::*,
    error::{Error, Result},
};

/// Subscribe to heartbeat messages from all terad instances
pub async fn subscribe_heartbeats(client: &Client) -> Result<Subscriber> {
    client.subscribe(SUBJECT_TERAD_HEARTBEAT.to_string())
        .await
        .map_err(Error::Nats)
}

/// Subscribe to log messages from all terad instances
pub async fn subscribe_logs(client: &Client) -> Result<Subscriber> {
    client.subscribe(SUBJECT_TERAD_LOG.to_string())
        .await
        .map_err(Error::Nats)
}

/// Subscribe to command responses from a specific terad instance
pub async fn subscribe_command_responses(client: &Client, node_id: &str) -> Result<Subscriber> {
    let subject = format!("{}.{}.response", SUBJECT_TERAD_COMMAND, node_id);
    client.subscribe(subject)
        .await
        .map_err(Error::Nats)
}

/// Process heartbeat messages
pub async fn process_heartbeats(mut subscriber: Subscriber) -> Result<()> {
    while let Some(message) = subscriber.next().await {
        let heartbeat: Heartbeat = match serde_json::from_slice(&message.payload) {
            Ok(h) => h,
            Err(e) => {
                // log::error!("Failed to parse heartbeat: {}", e);
                continue;
            }
        };

        // TODO: Process heartbeat
        // - Update node status in database
        // - Check for alerts
        // - Update web UI

        // log::debug!("Received heartbeat from node {}: {:?}", heartbeat.node_id, heartbeat);
    }

    Ok(())
}

/// Process log messages
pub async fn process_logs(mut subscriber: Subscriber) -> Result<()> {
    while let Some(message) = subscriber.next().await {
        let log_msg: LogMessage = match serde_json::from_slice(&message.payload) {
            Ok(l) => l,
            Err(e) => {
                // log::error!("Failed to parse log message: {}", e);
                continue;
            }
        };

        // TODO: Process log message
        // - Store in database
        // - Stream to web UI clients
        // - Check for error patterns

        // log::debug!("Received log from node {}: {:?}", log_msg.node_id, log_msg);
    }

    Ok(())
}

/// Process command responses
pub async fn process_command_responses(mut subscriber: Subscriber) -> Result<()> {
    while let Some(message) = subscriber.next().await {
        // TODO: Process command response
        // - Parse response
        // - Update command status
        // - Notify web UI

        // log::debug!("Received command response: {:?}", message.payload);
    }

    Ok(())
}
