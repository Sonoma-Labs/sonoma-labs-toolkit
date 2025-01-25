//! Network client implementation for handling network communication
//! 
//! This module provides:
//! - HTTP/WebSocket client functionality
//! - Connection pooling
//! - Request/response handling
//! - Retry logic
//! - Rate limiting

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, Semaphore};
use reqwest::{Client as HttpClient, Response};
use async_tungstenite::WebSocketStream;
use futures::{StreamExt, SinkExt};
use super::{NetworkConfig, NetworkError, NetworkResult, NetworkStatus, NetworkMetrics, Message};

/// Network client for handling communication
#[derive(Clone)]
pub struct NetworkClient {
    /// HTTP client
    http_client: HttpClient,
    /// WebSocket client
    ws_client: Option<WebSocketStream<async_tungstenite::stream::Stream<tokio::net::TcpStream>>>,
    /// Network configuration
    config: NetworkConfig,
    /// Connection semaphore for limiting concurrent connections
    connection_semaphore: Arc<Semaphore>,
    /// Network metrics
    metrics: Arc<RwLock<NetworkMetrics>>,
    /// Network status
    status: Arc<RwLock<NetworkStatus>>,
}

impl NetworkClient {
    /// Create a new network client with given configuration
    pub async fn new(config: NetworkConfig) -> NetworkResult<Self> {
        let http_client = HttpClient::builder()
            .timeout(config.timeout)
            .pool_max_idle_per_host(config.max_connections as usize)
            .build()
            .map_err(|e| NetworkError::ConnectionFailed(e.to_string()))?;

        Ok(Self {
            http_client,
            ws_client: None,
            config,
            connection_semaphore: Arc::new(Semaphore::new(100)), // Default max connections
            metrics: Arc::new(RwLock::new(NetworkMetrics::default())),
            status: Arc::new(RwLock::new(NetworkStatus {
                connected: false,
                latency: Duration::from_secs(0),
                active_connections: 0,
                pending_requests: 0,
            })),
        })
    }

    /// Send HTTP request
    pub async fn send_request(&self, endpoint: &str, body: &[u8]) -> NetworkResult<Vec<u8>> {
        let _permit = self.connection_semaphore.acquire().await
            .map_err(|e| NetworkError::ConnectionFailed(e.to_string()))?;

        let start_time = std::time::Instant::now();
        let mut retries = 0;

        loop {
            match self.http_client.post(&format!("{}{}", self.config.url, endpoint))
                .body(body.to_vec())
                .send()
                .await {
                    Ok(response) => {
                        self.update_metrics(start_time.elapsed()).await;
                        return self.handle_response(response).await;
                    }
                    Err(e) => {
                        if retries >= self.config.max_retries {
                            return Err(NetworkError::ConnectionFailed(e.to_string()));
                        }
                        retries += 1;
                        tokio::time::sleep(Duration::from_secs(1 << retries)).await;
                    }
                }
        }
    }

    /// Connect to WebSocket endpoint
    pub async fn connect_ws(&mut self, endpoint: &str) -> NetworkResult<()> {
        let url = format!("ws://{}{}", self.config.url.trim_start_matches("http://"), endpoint);
        let (ws_stream, _) = async_tungstenite::connect_async(&url)
            .await
            .map_err(|e| NetworkError::ConnectionFailed(e.to_string()))?;

        self.ws_client = Some(ws_stream);
        self.update_status(true).await;
        Ok(())
    }

    /// Send WebSocket message
    pub async fn send_ws_message(&mut self, message: Message) -> NetworkResult<()> {
        if let Some(ws) = &mut self.ws_client {
            ws.send(message.into())
                .await
                .map_err(|e| NetworkError::ProtocolError(e.to_string()))?;
            Ok(())
        } else {
            Err(NetworkError::ConnectionFailed("WebSocket not connected".to_string()))
        }
    }

    /// Receive WebSocket message
    pub async fn receive_ws_message(&mut self) -> NetworkResult<Option<Message>> {
        if let Some(ws) = &mut self.ws_client {
            match ws.next().await {
                Some(Ok(msg)) => Ok(Some(msg.into())),
                Some(Err(e)) => Err(NetworkError::ProtocolError(e.to_string())),
                None => Ok(None),
            }
        } else {
            Err(NetworkError::ConnectionFailed("WebSocket not connected".to_string()))
        }
    }

    /// Handle HTTP response
    async fn handle_response(&self, response: Response) -> NetworkResult<Vec<u8>> {
        match response.status() {
            status if status.is_success() => {
                response.bytes()
                    .await
                    .map(|b| b.to_vec())
                    .map_err(|e| NetworkError::InvalidResponse(e.to_string()))
            }
            status if status.is_client_error() => {
                Err(NetworkError::AuthenticationFailed("Invalid credentials".to_string()))
            }
            status if status.is_server_error() => {
                Err(NetworkError::ConnectionFailed("Server error".to_string()))
            }
            _ => Err(NetworkError::InvalidResponse("Unknown response status".to_string())),
        }
    }

    /// Update network metrics
    async fn update_metrics(&self, latency: Duration) {
        let mut metrics = self.metrics.write().await;
        metrics.total_requests += 1;
        metrics.total_responses += 1;
        metrics.average_latency = (metrics.average_latency + latency) / 2;
        if latency > metrics.max_latency {
            metrics.max_latency = latency;
        }
    }

    /// Update network status
    async fn update_status(&self, connected: bool) {
        let mut status = self.status.write().await;
        status.connected = connected;
    }

    /// Get current network metrics
    pub async fn get_metrics(&self) -> NetworkMetrics {
        self.metrics.read().await.clone()
    }

    /// Get current network status
    pub async fn get_status(&self) -> NetworkStatus {
        self.status.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let config = NetworkConfig::default();
        let client = NetworkClient::new(config).await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_metrics_update() {
        let config = NetworkConfig::default();
        let client = NetworkClient::new(config).await.unwrap();
        
        client.update_metrics(Duration::from_millis(100)).await;
        let metrics = client.get_metrics().await;
        
        assert_eq!(metrics.total_requests, 1);
        assert_eq!(metrics.total_responses, 1);
        assert!(metrics.average_latency <= Duration::from_millis(100));
    }
}