use solana_program::{program_error::ProgramError, decode_error::DecodeError};
use thiserror::Error;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Error, Debug, Clone, PartialEq, FromPrimitive)]
pub enum AgentError {
    #[error("Invalid agent configuration")]
    InvalidConfiguration = 0,

    #[error("Agent not initialized")]
    NotInitialized = 1,

    #[error("Invalid state transition")]
    InvalidStateTransition = 2,

    #[error("Capability not found")]
    CapabilityNotFound = 3,

    #[error("Insufficient permissions")]
    InsufficientPermissions = 4,

    #[error("Processing error")]
    ProcessingError = 5,

    #[error("Memory allocation error")]
    MemoryError = 6,

    #[error("Network communication error")]
    NetworkError = 7,

    #[error("Data validation error")]
    ValidationError = 8,

    #[error("Resource limit exceeded")]
    ResourceLimitExceeded = 9,

    #[error("Operation timeout")]
    Timeout = 10,

    #[error("Invalid input data")]
    InvalidInput = 11,

    #[error("System overload")]
    SystemOverload = 12,

    #[error("Unauthorized action")]
    Unauthorized = 13,

    #[error("Custom error: {0}")]
    Custom(String) = 14,
}

impl From<AgentError> for ProgramError {
    fn from(e: AgentError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for AgentError {
    fn type_of() -> &'static str {
        "AgentError"
    }
}

#[derive(Debug)]
pub struct ErrorMetadata {
    pub timestamp: u64,
    pub severity: ErrorSeverity,
    pub context: String,
    pub recoverable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl ErrorMetadata {
    pub fn new(severity: ErrorSeverity, context: &str, recoverable: bool) -> Self {
        Self {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            severity,
            context: context.to_string(),
            recoverable,
        }
    }
}

pub type AgentResult<T> = Result<T, AgentError>;

pub trait ErrorHandler {
    fn handle_error(&self, error: AgentError, metadata: ErrorMetadata) -> AgentResult<()>;
    fn log_error(&self, error: &AgentError, metadata: &ErrorMetadata);
    fn can_recover(&self, error: &AgentError) -> bool;
}

#[derive(Debug)]
pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn handle_error(&self, error: AgentError, metadata: ErrorMetadata) -> AgentResult<()> {
        self.log_error(&error, &metadata);
        
        if self.can_recover(&error) && metadata.recoverable {
            println!("Attempting to recover from error: {:?}", error);
            Ok(())
        } else {
            Err(error)
        }
    }

    fn log_error(&self, error: &AgentError, metadata: &ErrorMetadata) {
        println!(
            "Error occurred at {}: {:?} - Severity: {:?}, Context: {}",
            metadata.timestamp,
            error,
            metadata.severity,
            metadata.context
        );
    }

    fn can_recover(&self, error: &AgentError) -> bool {
        matches!(
            error,
            AgentError::NetworkError | AgentError::Timeout | AgentError::SystemOverload
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let error = AgentError::InvalidConfiguration;
        let program_error: ProgramError = error.into();
        assert_eq!(program_error, ProgramError::Custom(0));
    }

    #[test]
    fn test_error_handling() {
        let handler = DefaultErrorHandler;
        let error = AgentError::NetworkError;
        let metadata = ErrorMetadata::new(
            ErrorSeverity::Medium,
            "Network connection failed",
            true
        );
        
        assert!(handler.handle_error(error, metadata).is_ok());
    }

    #[test]
    fn test_non_recoverable_error() {
        let handler = DefaultErrorHandler;
        let error = AgentError::InvalidConfiguration;
        let metadata = ErrorMetadata::new(
            ErrorSeverity::Critical,
            "Invalid config",
            false
        );
        
        assert!(handler.handle_error(error, metadata).is_err());
    }

    #[test]
    fn test_custom_error() {
        let error = AgentError::Custom("Test error".to_string());
        assert_eq!(
            error.to_string(),
            "Custom error: Test error"
        );
    }
}