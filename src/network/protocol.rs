//! Protocol implementation for network communication
//! 
//! This module provides:
//! - Message types and serialization
//! - Protocol versioning
//! - Message validation
//! - Protocol handshaking
//! - Message routing

use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use sha2::{Sha256, Digest};
use super::NetworkError;

/// Protocol version
pub const PROTOCOL_VERSION: u32 = 1;

/// Message types for network communication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    /// Handshake message for protocol negotiation
    Handshake {
        version: u32,
        timestamp: u64,
        capabilities: Vec<String>,
    },
    /// Request message
    Request {
        id: String,
        method: String,
        params: Vec<u8>,
    },
    /// Response message
    Response {
        id: String,
        status: ResponseStatus,
        data: Vec<u8>,
    },
    /// Error message
    Error {
        id: String,
        code: u32,
        message: String,
    },
    /// Ping message for keepalive
    Ping(u64),
    /// Pong message in response to ping
    Pong(u64),
    /// Notification message
    Notification {
        topic: String,
        data: Vec<u8>,
    },
}

/// Response status codes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseStatus {
    Success,
    Error,
    Pending,
}

/// Protocol message wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message version
    pub version: u32,
    /// Message type
    pub message_type: MessageType,
    /// Message timestamp
    pub timestamp: u64,
    /// Message signature (if required)
    pub signature: Option<Vec<u8>>,
}

impl Message {
    /// Create a new message
    pub fn new(message_type: MessageType) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            message_type,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            signature: None,
        }
    }

    /// Create a new request message
    pub fn request(id: impl Into<String>, method: impl Into<String>, params: Vec<u8>) -> Self {
        Self::new(MessageType::Request {
            id: id.into(),
            method: method.into(),
            params,
        })
    }

    /// Create a new response message
    pub fn response(id: impl Into<String>, status: ResponseStatus, data: Vec<u8>) -> Self {
        Self::new(MessageType::Response {
            id: id.into(),
            status,
            data,
        })
    }

    /// Create a new error message
    pub fn error(id: impl Into<String>, code: u32, message: impl Into<String>) -> Self {
        Self::new(MessageType::Error {
            id: id.into(),
            code,
            message: message.into(),
        })
    }

    /// Create a new notification message
    pub fn notification(topic: impl Into<String>, data: Vec<u8>) -> Self {
        Self::new(MessageType::Notification {
            topic: topic.into(),
            data,
        })
    }

    /// Calculate message hash
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&bincode::serialize(self).unwrap_or_default());
        hasher.finalize().into()
    }

    /// Validate message format and contents
    pub fn validate(&self) -> Result<(), NetworkError> {
        // Check protocol version
        if self.version != PROTOCOL_VERSION {
            return Err(NetworkError::ProtocolError(
                format!("Invalid protocol version: {}", self.version)
            ));
        }

        // Validate timestamp
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        if self.timestamp > current_time + 300 { // Allow 5 minutes clock skew
            return Err(NetworkError::ProtocolError(
                "Message timestamp is in the future".to_string()
            ));
        }

        // Validate message-specific fields
        match &self.message_type {
            MessageType::Request { id, method, .. } => {
                if id.is_empty() || method.is_empty() {
                    return Err(NetworkError::ProtocolError(
                        "Invalid request message format".to_string()
                    ));
                }
            }
            MessageType::Response { id, .. } |
            MessageType::Error { id, .. } => {
                if id.is_empty() {
                    return Err(NetworkError::ProtocolError(
                        "Invalid response/error message format".to_string()
                    ));
                }
            }
            MessageType::Notification { topic, .. } => {
                if topic.is_empty() {
                    return Err(NetworkError::ProtocolError(
                        "Invalid notification message format".to_string()
                    ));
                }
            }
            _ => {}
        }

        Ok(())
    }
}

/// Protocol handler trait
#[async_trait::async_trait]
pub trait Protocol: Send + Sync {
    /// Handle incoming message
    async fn handle_message(&self, message: Message) -> Result<Option<Message>, NetworkError>;
    
    /// Handle protocol error
    async fn handle_error(&self, error: NetworkError);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let request = Message::request("test-id", "test-method", vec![1, 2, 3]);
        assert_eq!(request.version, PROTOCOL_VERSION);
        
        if let MessageType::Request { id, method, params } = request.message_type {
            assert_eq!(id, "test-id");
            assert_eq!(method, "test-method");
            assert_eq!(params, vec![1, 2, 3]);
        } else {
            panic!("Unexpected message type");
        }
    }

    #[test]
    fn test_message_validation() {
        let valid_msg = Message::request("test-id", "test-method", vec![]);
        assert!(valid_msg.validate().is_ok());

        let mut invalid_msg = Message::new(MessageType::Request {
            id: "".to_string(),
            method: "test".to_string(),
            params: vec![],
        });
        assert!(invalid_msg.validate().is_err());

        invalid_msg.version = 999;
        assert!(invalid_msg.validate().is_err());
    }
}