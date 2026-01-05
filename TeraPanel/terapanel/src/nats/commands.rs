//! NATS command senders
//!
//! Functions to send commands from terapanel to terad instances.

use async_nats::Client;
use tera_common::{
    protocol::*,
    types::*,
    error::{Error, Result},
};
use serde_json::json;

/// Send a command to a specific terad instance
pub async fn send_command(client: &Client, node_id: &str, command: Command) -> Result<()> {
    let subject = format!("{}.{}", SUBJECT_TERAD_COMMAND, node_id);

    let payload = serde_json::to_vec(&command)
        .map_err(Error::Serialization)?;

    client.publish(&subject, payload.into())
        .await
        .map_err(Error::Nats)?;

    Ok(())
}

/// Send a file management command
pub async fn send_file_command(
    client: &Client,
    node_id: &str,
    path: &str,
    action: &str,
) -> Result<()> {
    let subject = format!("{}.{}", SUBJECT_FILE_MANAGER, node_id);

    let payload = json!({
        "action": action,
        "path": path,
    });

    let payload_str = serde_json::to_string(&payload)
        .map_err(Error::Serialization)?;

    client.publish(&subject, payload_str.into())
        .await
        .map_err(Error::Nats)?;

    Ok(())
}

/// Send a terminal command
pub async fn send_terminal_command(
    client: &Client,
    node_id: &str,
    session_id: &str,
    command: &str,
) -> Result<()> {
    let subject = format!("{}.{}.{}", SUBJECT_TERMINAL, node_id, session_id);

    let payload = json!({
        "command": command,
    });

    let payload_str = serde_json::to_string(&payload)
        .map_err(Error::Serialization)?;

    client.publish(&subject, payload_str.into())
        .await
        .map_err(Error::Nats)?;

    Ok(())
}

/// Send a service management command
pub async fn send_service_command(
    client: &Client,
    node_id: &str,
    service_id: &str,
    action: &str,
) -> Result<()> {
    let subject = format!("{}.{}.{}", SUBJECT_SERVICE, node_id, service_id);

    let payload = json!({
        "action": action,
    });

    let payload_str = serde_json::to_string(&payload)
        .map_err(Error::Serialization)?;

    client.publish(&subject, payload_str.into())
        .await
        .map_err(Error::Nats)?;

    Ok(())
}

/// Send a node administration command
pub async fn send_node_command(
    client: &Client,
    node_id: &str,
    action: &str,
    parameters: HashMap<String, String>,
) -> Result<()> {
    let subject = format!("{}.{}", SUBJECT_NODE_ADMIN, node_id);

    let payload = json!({
        "action": action,
        "parameters": parameters,
    });

    let payload_str = serde_json::to_string(&payload)
        .map_err(Error::Serialization)?;

    client.publish(&subject, payload_str.into())
        .await
        .map_err(Error::Nats)?;

    Ok(())
}
