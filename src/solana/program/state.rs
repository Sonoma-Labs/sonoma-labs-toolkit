use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey,
};
use crate::solana::program::instruction::AgentConfig;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum AgentState {
    Uninitialized,
    Initialized,
    Running,
    Paused,
    Error,
    Terminated,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AgentAccount {
    pub authority: Pubkey,
    pub name: String,
    pub config: AgentConfig,
    pub state: AgentState,
    pub last_execution: i64,
    pub execution_count: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AgentMetadata {
    pub created_at: i64,
    pub updated_at: i64,
    pub version: u32,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PerformanceMetrics {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub average_execution_time: u64,
    pub total_compute_units: u64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_executions: 0,
            successful_executions: 0,
            failed_executions: 0,
            average_execution_time: 0,
            total_compute_units: 0,
        }
    }
}

impl AgentAccount {
    pub fn new(authority: Pubkey, name: String, config: AgentConfig) -> Self {
        Self {
            authority,
            name,
            config,
            state: AgentState::Initialized,
            last_execution: 0,
            execution_count: 0,
        }
    }

    pub fn update_state(&mut self, new_state: AgentState) -> Result<(), ProgramError> {
        match (self.state.clone(), new_state) {
            (AgentState::Uninitialized, AgentState::Initialized) => Ok(()),
            (AgentState::Initialized, AgentState::Running) => Ok(()),
            (AgentState::Running, AgentState::Paused) => Ok(()),
            (AgentState::Paused, AgentState::Running) => Ok(()),
            (_, AgentState::Error) => Ok(()),
            (_, AgentState::Terminated) => Ok(()),
            _ => Err(ProgramError::InvalidAccountData),
        }?;

        self.state = new_state;
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        matches!(self.state, AgentState::Running)
    }

    pub fn can_execute(&self) -> bool {
        self.is_active() && self.config.execution_limit > self.execution_count
    }

    pub fn record_execution(&mut self, timestamp: i64) {
        self.last_execution = timestamp;
        self.execution_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_state_transitions() {
        let mut agent = AgentAccount::new(
            Pubkey::new_unique(),
            "test_agent".to_string(),
            AgentConfig {
                autonomous_mode: true,
                execution_limit: 1000,
                memory_limit: 5000,
                capabilities: vec!["compute".to_string()],
            },
        );

        assert_eq!(agent.state, AgentState::Initialized);
        assert!(agent.update_state(AgentState::Running).is_ok());
        assert_eq!(agent.state, AgentState::Running);
    }

    #[test]
    fn test_agent_execution_tracking() {
        let mut agent = AgentAccount::new(
            Pubkey::new_unique(),
            "test_agent".to_string(),
            AgentConfig {
                autonomous_mode: true,
                execution_limit: 2,
                memory_limit: 5000,
                capabilities: vec!["compute".to_string()],
            },
        );

        agent.update_state(AgentState::Running).unwrap();
        assert!(agent.can_execute());
        
        agent.record_execution(1000);
        assert!(agent.can_execute());
        
        agent.record_execution(2000);
        assert!(!agent.can_execute());
    }

    #[test]
    fn test_performance_metrics() {
        let metrics = PerformanceMetrics::default();
        assert_eq!(metrics.total_executions, 0);
        assert_eq!(metrics.successful_executions, 0);
        assert_eq!(metrics.failed_executions, 0);
    }
}