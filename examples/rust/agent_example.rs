use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use sonoma_labs_toolkit::{
    agent::{Agent, AgentConfig, AgentState, Capabilities},
    error::SonomaError,
    program::{instruction::*, state::*},
};
use std::{str::FromStr, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize connection to devnet
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new_with_commitment(
        rpc_url,
        CommitmentConfig::confirmed(),
    );

    // Create or load keypair for testing
    let payer = Keypair::new();
    println!("Using keypair: {}", payer.pubkey());

    // Request airdrop for testing
    let airdrop_signature = client.request_airdrop(
        &payer.pubkey(),
        1_000_000_000, // 1 SOL
    )?;
    client.confirm_transaction(&airdrop_signature)?;
    println!("Airdrop successful");

    // Initialize program ID
    let program_id = Pubkey::from_str("YOUR_PROGRAM_ID")?;

    // Create agent configuration
    let config = AgentConfig {
        autonomous_mode: true,
        execution_limit: 1000,
        memory_limit: 10 * 1024 * 1024, // 10MB
        capabilities: Capabilities {
            compute: true,
            storage: true,
            network: false,
            custom_capabilities: vec!["example".to_string()],
        },
        metadata: Some(serde_json::json!({
            "description": "Example Rust agent",
            "version": "1.0.0"
        })),
    };

    // Create new agent
    println!("Creating new agent...");
    let agent = Agent::new(
        &client,
        &program_id,
        &payer,
        "example-rust-agent",
        config.clone(),
    )?;
    println!("Agent created: {}", agent.pubkey());

    // Subscribe to agent state changes
    let state_subscription = agent.subscribe_state_changes()?;
    tokio::spawn(async move {
        while let Ok(state) = state_subscription.recv().await {
            println!("Agent state changed: {:?}", state);
        }
    });

    // Execute agent action
    println!("Executing agent action...");
    let action_data = b"example action";
    agent.execute(action_data)?;

    // Wait for execution
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Get agent metrics
    let metrics = agent.get_metrics()?;
    println!("Agent metrics: {:#?}", metrics);

    // Update configuration
    println!("Updating agent configuration...");
    let mut new_config = config;
    new_config.execution_limit = 2000;
    new_config.capabilities.network = true;
    agent.update_config(&new_config)?;

    // Pause agent
    println!("Pausing agent...");
    agent.pause()?;
    assert_eq!(agent.get_state()?, AgentState::Paused);

    // Resume agent
    println!("Resuming agent...");
    agent.resume()?;
    assert_eq!(agent.get_state()?, AgentState::Running);

    // Example of custom instruction
    println!("Sending custom instruction...");
    let custom_instruction = create_custom_instruction(
        &program_id,
        &agent.pubkey(),
        &payer.pubkey(),
        action_data,
    )?;

    let mut transaction = Transaction::new_with_payer(
        &[custom_instruction],
        Some(&payer.pubkey()),
    );
    let blockhash = client.get_latest_blockhash()?;
    transaction.sign(&[&payer], blockhash);
    client.send_and_confirm_transaction(&transaction)?;

    // Error handling example
    match agent.execute(b"invalid action") {
        Ok(_) => println!("Action executed successfully"),
        Err(SonomaError::InvalidAction) => println!("Invalid action detected"),
        Err(e) => println!("Error executing action: {:?}", e),
    }

    // Clean up
    println!("Cleaning up...");
    agent.close(&payer.pubkey())?;

    println!("Example completed successfully!");
    Ok(())
}

// Helper function for custom instruction
fn create_custom_instruction(
    program_id: &Pubkey,
    agent: &Pubkey,
    authority: &Pubkey,
    data: &[u8],
) -> Result<solana_sdk::instruction::Instruction, Box<dyn std::error::Error>> {
    let instruction = solana_sdk::instruction::Instruction {
        program_id: *program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(*agent, false),
            solana_sdk::instruction::AccountMeta::new(*authority, true),
        ],
        data: data.to_vec(),
    };
    Ok(instruction)
}

// Example of implementing custom trait
trait AgentExtension {
    fn get_metrics(&self) -> Result<AgentMetrics, SonomaError>;
    fn subscribe_state_changes(&self) -> Result<tokio::sync::broadcast::Receiver<AgentState>, SonomaError>;
}

impl AgentExtension for Agent {
    fn get_metrics(&self) -> Result<AgentMetrics, SonomaError> {
        // Implementation details
        unimplemented!()
    }

    fn subscribe_state_changes(&self) -> Result<tokio::sync::broadcast::Receiver<AgentState>, SonomaError> {
        // Implementation details
        unimplemented!()
    }
}