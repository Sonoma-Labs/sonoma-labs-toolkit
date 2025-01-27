//! Storage module for managing persistent data and caching
//! 
//! This module provides:
//! - Database abstraction
//! - Caching mechanisms
//! - Data persistence
//! - Storage optimization
//! - Backup/restore functionality

use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use tokio::sync::RwLock;
use std::sync::Arc;

mod database;
mod cache;

pub use database::{Database, DatabaseConfig};
pub use cache::{Cache, CacheConfig};

/// Default storage directory name
pub const DEFAULT_STORAGE_DIR: &str = ".sonoma/storage";

/// Storage configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Base directory for storage
    pub base_dir: PathBuf,
    /// Database configuration
    pub database: DatabaseConfig,
    /// Cache configuration
    pub cache: CacheConfig,
    /// Maximum storage size (in bytes)
    pub max_size: u64,
    /// Auto-cleanup threshold (0.0 - 1.0)
    pub cleanup_threshold: f32,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            base_dir: dirs::home_dir()
                .unwrap_or_default()
                .join(DEFAULT_STORAGE_DIR),
            database: DatabaseConfig::default(),
            cache: CacheConfig::default(),
            max_size: 1024 * 1024 * 1024, // 1GB
            cleanup_threshold: 0.9, // 90%
        }
    }
}

/// Storage errors that can occur during operations
#[derive(Error, Debug)]
pub enum StorageError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),

    /// Database error
    #[error("Database error: {0}")]
    Database(String),

    /// Cache error
    #[error("Cache error: {0}")]
    Cache(String),

    /// Storage full
    #[error("Storage full: required {required} bytes, available {available} bytes")]
    StorageFull {
        required: u64,
        available: u64,
    },

    /// Invalid path
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Data not found
    #[error("Data not found: {0}")]
    NotFound(String),
}

/// Result type for storage operations
pub type StorageResult<T> = Result<T, StorageError>;

/// Storage metrics for monitoring
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    /// Total storage size used
    pub used_size: u64,
    /// Total number of items
    pub total_items: u64,
    /// Cache hit rate
    pub cache_hit_rate: f32,
    /// Database operations per second
    pub db_ops_per_second: f32,
}

/// Storage manager for handling data persistence
pub struct StorageManager {
    /// Storage configuration
    config: StorageConfig,
    /// Database instance
    database: Arc<RwLock<Database>>,
    /// Cache instance
    cache: Arc<RwLock<Cache>>,
    /// Storage metrics
    metrics: Arc<RwLock<StorageMetrics>>,
}

impl StorageManager {
    /// Create a new storage manager
    pub async fn new(config: StorageConfig) -> StorageResult<Self> {
        // Ensure storage directory exists
        tokio::fs::create_dir_all(&config.base_dir).await?;

        // Initialize database and cache
        let database = Database::new(config.database.clone()).await?;
        let cache = Cache::new(config.cache.clone()).await?;

        Ok(Self {
            config,
            database: Arc::new(RwLock::new(database)),
            cache: Arc::new(RwLock::new(cache)),
            metrics: Arc::new(RwLock::new(StorageMetrics::default())),
        })
    }

    /// Store data with given key
    pub async fn store<T: Serialize>(&self, key: &str, value: &T) -> StorageResult<()> {
        // Check storage capacity
        let size = bincode::serialized_size(value)? as u64;
        self.ensure_capacity(size).await?;

        // Try cache first
        let mut cache = self.cache.write().await;
        cache.set(key, value).await?;

        // Then persist to database
        let mut database = self.database.write().await;
        database.store(key, value).await?;

        // Update metrics
        let mut metrics = self.metrics.write().await;
        metrics.used_size += size;
        metrics.total_items += 1;

        Ok(())
    }

    /// Retrieve data for given key
    pub async fn retrieve<T: for<'de> Deserialize<'de>>(&self, key: &str) -> StorageResult<T> {
        // Try cache first
        let mut cache = self.cache.write().await;
        if let Some(value) = cache.get::<T>(key).await? {
            let mut metrics = self.metrics.write().await;
            metrics.cache_hit_rate = (metrics.cache_hit_rate * 0.9) + 0.1;
            return Ok(value);
        }

        // Fall back to database
        let database = self.database.read().await;
        let value = database.retrieve::<T>(key).await?;

        // Update cache
        cache.set(key, &value).await?;

        // Update metrics
        let mut metrics = self.metrics.write().await;
        metrics.cache_hit_rate = metrics.cache_hit_rate * 0.9;

        Ok(value)
    }

    /// Delete data for given key
    pub async fn delete(&self, key: &str) -> StorageResult<()> {
        // Remove from cache
        let mut cache = self.cache.write().await;
        cache.delete(key).await?;

        // Remove from database
        let mut database = self.database.write().await;
        database.delete(key).await?;

        Ok(())
    }

    /// Clear all storage
    pub async fn clear(&self) -> StorageResult<()> {
        // Clear cache
        let mut cache = self.cache.write().await;
        cache.clear().await?;

        // Clear database
        let mut database = self.database.write().await;
        database.clear().await?;

        // Reset metrics
        let mut metrics = self.metrics.write().await;
        *metrics = StorageMetrics::default();

        Ok(())
    }

    /// Get current storage metrics
    pub async fn get_metrics(&self) -> StorageMetrics {
        self.metrics.read().await.clone()
    }

    /// Ensure storage has enough capacity
    async fn ensure_capacity(&self, required: u64) -> StorageResult<()> {
        let metrics = self.metrics.read().await;
        let available = self.config.max_size - metrics.used_size;

        if required > available {
            return Err(StorageError::StorageFull {
                required,
                available,
            });
        }

        if (metrics.used_size + required) as f32 / self.config.max_size as f32 
            > self.config.cleanup_threshold 
        {
            // Trigger cleanup in background
            let cache = self.cache.clone();
            tokio::spawn(async move {
                if let Err(e) = cache.write().await.cleanup().await {
                    eprintln!("Cache cleanup error: {}", e);
                }
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_storage_manager() {
        let temp_dir = tempdir().unwrap();
        let config = StorageConfig {
            base_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let manager = StorageManager::new(config).await.unwrap();
        
        // Test store and retrieve
        manager.store("test-key", &"test-value").await.unwrap();
        let value: String = manager.retrieve("test-key").await.unwrap();
        assert_eq!(value, "test-value");

        // Test delete
        manager.delete("test-key").await.unwrap();
        assert!(manager.retrieve::<String>("test-key").await.is_err());
    }
}