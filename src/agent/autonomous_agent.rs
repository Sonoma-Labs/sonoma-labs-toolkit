use solana_program::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    program_error::ProgramError,
};
use crate::SonomaConfig;
use super::{AgentBehavior, AgentState, capabilities::AgentCapabilities, base::Agent};

#[derive(Debug)]
pub struct AutonomousAgent {
    base: Agent,
    autonomous_config: AutonomousConfig,
    execution_state: ExecutionState,
    last_action: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AutonomousConfig {
    pub decision_threshold: f32,
    pub max_actions_per_cycle: u32,
    pub learning_rate: f32,
    pub memory_capacity: usize,
}

#[derive(Debug)]
pub enum ExecutionState {
    Planning,
    Executing,
    Learning,
    Idle,
}

impl Default for AutonomousConfig {
    fn default() -> Self {
        Self {
            decision_threshold: 0.7,
            max_actions_per_cycle: 100,
            learning_rate: 0.01,
            memory_capacity: 1000,
        }
    }
}

impl AutonomousAgent {
    pub fn new(name: &str, config: &SonomaConfig) -> Self {
        Self {
            base: Agent::new(name, config),
            autonomous_config: AutonomousConfig::default(),
            execution_state: ExecutionState::Idle,
            last_action: None,
        }
    }

    pub async fn execute_cycle(&mut self) -> Result<(), ProgramError> {
        println!("Executing autonomous cycle for agent: {}", self.base.name);
        self.execution_state = ExecutionState::Planning;
        // Implement autonomous decision-making cycle
        self.last_action = Some("Completed planning phase".to_string());
        Ok(())
    }

    pub async fn update_config(&mut self, config: AutonomousConfig) -> Result<(), ProgramError> {
        self.autonomous_config = config;
        println!("Updated autonomous configuration for: {}", self.base.name);
        Ok(())
    }

    pub fn get_execution_state(&self) -> &ExecutionState {
        &self.execution_state
    }
}

impl AgentBehavior for AutonomousAgent {
    fn process_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Processing data in autonomous agent: {}", self.base.name);
        // Implement autonomous data processing
        Ok(())
    }

    fn update_state(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Updating autonomous agent state: {}", self.base.name);
        // Implement autonomous state updates
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autonomous_agent_creation() {
        let config = SonomaConfig::default();
        let agent = AutonomousAgent::new("test_autonomous_agent", &config);
        assert_eq!(agent.base.name, "test_autonomous_agent");
        matches!(agent.execution_state, ExecutionState::Idle);
    }

    #[test]
    fn test_autonomous_config_update() {
        let config = SonomaConfig::default();
        let mut agent = AutonomousAgent::new("test_autonomous_agent", &config);
        let new_config = AutonomousConfig {
            decision_threshold: 0.8,
            max_actions_per_cycle: 200,
            learning_rate: 0.02,
            memory_capacity: 2000,
        };
        agent.update_config(new_config.clone()).unwrap();
        assert_eq!(agent.autonomous_config.decision_threshold, 0.8);
        assert_eq!(agent.autonomous_config.max_actions_per_cycle, 200);
    }
}