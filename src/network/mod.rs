//! Network module for handling communication and RPC interactions
//! 
//! This module provides functionality for:
//! - Network client management
//! - Protocol handling
//! - RPC communication
//! - Connection pooling
//! - Request/response handling

use std::time::Duration;
use thiserror::Error;
use tokio::time::timeout;
use serde::{Serialize, Deserialize};

mod client;
mod protocol;

pub use client::NetworkClient;
pub use protocol::{Protocol, Message, MessageType};

/// Default timeout for network requests
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Default maximum retries for network operations
pub const MAX_RETRIES: u32 = 3;

/// Network configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Base URL for the network
    pub url: String,
    /// Request timeout
    pub timeout: Duration,
    /// Maximum number of retries
    pub max_retries: u32,
    /// Keep alive duration
    pub keep_alive: Duration,
    /// Maximum connections in pool
    pub max_connections: u32,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:8899".to_string(),
            timeout: DEFAULT_TIMEOUT,
            max_retries: MAX_RETRIES,
            keep_alive: Duration::from_secs(60),
            max_connections: 100,
        }
    }
}

/// Network errors that can occur during operations
#[derive(Error, Debug)]
pub enum NetworkError {
    /// Connection failed
    #[error("Failed to connect to network: {0}")]
    ConnectionFailed(String),

    /// Request timeout
    #[error("Request timed out after {0:?}")]
    Timeout(Duration),

    /// Invalid response
    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    /// Protocol error
    #[error("Protocol error: {0}")]
    ProtocolError(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Try again in {0:?}")]
    RateLimitExceeded(Duration),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
}

/// Result type for network operations
pub type NetworkResult<T> = Result<T, NetworkError>;

/// Network status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    /// Whether the network is connected
    pub connected: bool,
    /// Current latency
    pub latency: Duration,
    /// Number of active connections
    pub active_connections: u32,
    /// Number of pending requests
    pub pending_requests: u32,
}

/// Network metrics for monitoring
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Total requests sent
    pub total_requests: u64,
    /// Total responses received
    pub total_responses: u64,
    /// Total errors encountered
    pub total_errors: u64,
    /// Average latency
    pub average_latency: Duration,
    /// Maximum latency observed
    pub max_latency: Duration,
}

/// Trait for network handlers
#[async_trait::async_trait]
pub trait NetworkHandler: Send + Sync {
    /// Handle incoming message
    async fn handle_message(&self, message: Message) -> NetworkResult<Message>;
    
    /// Handle network error
    async fn handle_error(&self, error: NetworkError);
    
    /// Handle network status update
    async fn handle_status(&self, status: NetworkStatus);
}

/// Initialize the network module with given configuration
pub async fn init(config: NetworkConfig) -> NetworkResult<NetworkClient> {
    NetworkClient::new(config).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_network_config_default() {
        let config = NetworkConfig::default();
        assert_eq!(config.url, "http://localhost:8899");
        assert_eq!(config.timeout, DEFAULT_TIMEOUT);
        assert_eq!(config.max_retries, MAX_RETRIES);
    }

    #[tokio::test]
    async fn test_network_init() {
        let config = NetworkConfig::default();
        let result = init(config).await;
        assert!(result.is_ok());
    }
}