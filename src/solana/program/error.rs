use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::ProgramError,
};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, FromPrimitive, PartialEq)]
pub enum AgentError {
    #[error("Invalid instruction data")]
    InvalidInstructionData = 0,

    #[error("Invalid agent state for operation")]
    InvalidAgentState = 1,

    #[error("Invalid authority for agent")]
    InvalidAuthority = 2,

    #[error("Agent account not initialized")]
    NotInitialized = 3,

    #[error("Agent execution limit exceeded")]
    ExecutionLimitExceeded = 4,

    #[error("Memory limit exceeded")]
    MemoryLimitExceeded = 5,

    #[error("Invalid configuration")]
    InvalidConfiguration = 6,

    #[error("Insufficient funds")]
    InsufficientFunds = 7,

    #[error("Invalid account data")]
    InvalidAccountData = 8,

    #[error("Operation timeout")]
    OperationTimeout = 9,

    #[error("Unauthorized operation")]
    Unauthorized = 10,

    #[error("Agent already initialized")]
    AlreadyInitialized = 11,

    #[error("Invalid account owner")]
    InvalidOwner = 12,

    #[error("Invalid program address")]
    InvalidProgramAddress = 13,

    #[error("Invalid system program")]
    InvalidSystemProgram = 14,
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

pub fn handle_error(error: AgentError) -> ProgramError {
    msg!("Agent error: {}", error);
    error.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let error = AgentError::InvalidInstructionData;
        let program_error: ProgramError = error.into();
        assert_eq!(program_error, ProgramError::Custom(0));
    }

    #[test]
    fn test_error_messages() {
        let error = AgentError::InvalidAuthority;
        assert_eq!(error.to_string(), "Invalid authority for agent");
    }

    #[test]
    fn test_error_handling() {
        let result = handle_error(AgentError::InvalidConfiguration);
        assert_eq!(result, ProgramError::Custom(AgentError::InvalidConfiguration as u32));
    }
}