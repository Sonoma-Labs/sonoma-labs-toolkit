use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum AgentInstruction {
    /// Initialize a new agent
    /// Accounts expected:
    /// 0. `[writable]` Agent account
    /// 1. `[signer]` Authority
    /// 2. `[]` System program
    Initialize {
        name: String,
        config: AgentConfig,
    },

    /// Update agent configuration
    /// Accounts expected:
    /// 0. `[writable]` Agent account
    /// 1. `[signer]` Authority
    Update {
        config: AgentConfig,
    },

    /// Execute agent action
    /// Accounts expected:
    /// 0. `[writable]` Agent account
    /// 1. `[signer]` Authority
    /// 2. `[writable]` Data account
    Execute {
        action_data: Vec<u8>,
    },

    /// Pause agent operations
    /// Accounts expected:
    /// 0. `[writable]` Agent account
    /// 1. `[signer]` Authority
    Pause,

    /// Resume agent operations
    /// Accounts expected:
    /// 0. `[writable]` Agent account
    /// 1. `[signer]` Authority
    Resume,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct AgentConfig {
    pub autonomous_mode: bool,
    pub execution_limit: u64,
    pub memory_limit: u64,
    pub capabilities: Vec<String>,
}

impl AgentInstruction {
    pub fn initialize(
        program_id: &Pubkey,
        agent_account: &Pubkey,
        authority: &Pubkey,
        name: String,
        config: AgentConfig,
    ) -> Instruction {
        let accounts = vec![
            AccountMeta::new(*agent_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ];

        Instruction::new_with_borsh(
            *program_id,
            &AgentInstruction::Initialize { name, config },
            accounts,
        )
    }

    pub fn update(
        program_id: &Pubkey,
        agent_account: &Pubkey,
        authority: &Pubkey,
        config: AgentConfig,
    ) -> Instruction {
        let accounts = vec![
            AccountMeta::new(*agent_account, false),
            AccountMeta::new_readonly(*authority, true),
        ];

        Instruction::new_with_borsh(
            *program_id,
            &AgentInstruction::Update { config },
            accounts,
        )
    }

    pub fn execute(
        program_id: &Pubkey,
        agent_account: &Pubkey,
        authority: &Pubkey,
        data_account: &Pubkey,
        action_data: Vec<u8>,
    ) -> Instruction {
        let accounts = vec![
            AccountMeta::new(*agent_account, false),
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new(*data_account, false),
        ];

        Instruction::new_with_borsh(
            *program_id,
            &AgentInstruction::Execute { action_data },
            accounts,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_serialization() {
        let config = AgentConfig {
            autonomous_mode: true,
            execution_limit: 1000,
            memory_limit: 5000,
            capabilities: vec!["compute".to_string()],
        };

        let instruction = AgentInstruction::Initialize {
            name: "test_agent".to_string(),
            config: config.clone(),
        };

        let serialized = borsh::to_vec(&instruction).unwrap();
        let deserialized = AgentInstruction::try_from_slice(&serialized).unwrap();
        assert_eq!(instruction, deserialized);
    }
}