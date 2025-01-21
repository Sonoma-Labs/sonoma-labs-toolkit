use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg,
};

pub mod state;
pub mod instruction;
pub mod processor;
pub mod error;

// Declare the program's entrypoint
entrypoint!(process_instruction);

/// Program entrypoint implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Sonoma Labs Program - Processing instruction");
    
    // Log the program ID for debugging
    msg!("Program ID: {}", program_id);
    
    // Log the number of accounts for debugging
    msg!("Number of accounts: {}", accounts.len());
    
    // Log instruction data length for debugging
    msg!("Instruction data length: {}", instruction_data.len());

    // Process the instruction
    match processor::Processor::process(program_id, accounts, instruction_data) {
        Ok(_) => {
            msg!("Instruction processed successfully");
            Ok(())
        }
        Err(error) => {
            msg!("Error processing instruction: {:?}", error);
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_entrypoint() {
        // Create test accounts
        let program_id = Pubkey::new_unique();
        let key = Pubkey::new_unique();
        let mut lamports = 0;
        
        let mut data = vec![0; 32];
        let owner = Pubkey::new_unique();
        
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];
        let instruction_data = vec![0, 1, 2, 3];

        // Test instruction processing
        let result = process_instruction(&program_id, &accounts, &instruction_data);
        assert!(result.is_err()); // Should error with invalid instruction
    }
}
