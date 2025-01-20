use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};

pub mod agent;
pub mod models;
pub mod state;
pub mod error;
pub mod instructions;

#[cfg(feature = "ai-integration")]
pub mod ai;

pub struct SonomaConfig {
    pub network: String,
    pub api_key: Option<String>,
    pub model_config: Option<ModelConfig>,
}

pub struct ModelConfig {
    pub model_type: String,
    pub parameters: serde_json::Value,
}

impl Default for SonomaConfig {
    fn default() -> Self {
        Self {
            network: "devnet".to_string(),
            api_key: None,
            model_config: None,
        }
    }
}

pub struct Sonoma {
    config: SonomaConfig,
}

impl Sonoma {
    pub fn new(config: SonomaConfig) -> Self {
        Self { config }
    }

    pub fn create_agent(&self, name: &str) -> agent::Agent {
        agent::Agent::new(name, &self.config)
    }
}

// Solana program entrypoint
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    instructions::process_instruction(program_id, accounts, instruction_data)
        .map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SonomaConfig::default();
        assert_eq!(config.network, "devnet");
        assert!(config.api_key.is_none());
    }

    #[test]
    fn test_create_agent() {
        let config = SonomaConfig::default();
        let sonoma = Sonoma::new(config);
        let agent = sonoma.create_agent("test_agent");
        // Add more specific tests as agent functionality is implemented
    }
}
