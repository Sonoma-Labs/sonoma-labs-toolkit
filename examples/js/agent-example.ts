import { Connection, Keypair } from '@solana/web3.js';
import { SonomaSDK, Agent, AgentConfig } from '@sonoma-labs/toolkit';

async function main() {
    try {
        // Initialize connection and SDK
        const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
        const payer = Keypair.generate(); // In production, use your own keypair
        
        const sdk = new SonomaSDK({
            connection,
            payer: payer.publicKey,
            programId: 'YOUR_PROGRAM_ID' // Replace with actual program ID
        });

        // Create agent configuration
        const agentConfig: AgentConfig = {
            autonomousMode: true,
            executionLimit: 1000,
            memoryLimit: 1024 * 1024 * 10, // 10MB
            capabilities: ['compute', 'storage'],
            metadata: {
                description: 'Example autonomous agent',
                version: '1.0.0'
            }
        };

        // Create a new agent
        console.log('Creating new agent...');
        const agent = await Agent.create(
            sdk.getProgram(),
            'example-agent',
            agentConfig
        );
        console.log('Agent created:', agent.publicKey.toBase58());

        // Set up event listeners
        agent.on('stateChange', (oldState, newState) => {
            console.log(`State changed from ${oldState} to ${newState}`);
        });

        agent.on('execution', (success, data) => {
            if (success) {
                console.log('Execution successful:', data);
            } else {
                console.error('Execution failed:', data);
            }
        });

        agent.on('error', (error) => {
            console.error('Agent error:', error);
        });

        // Execute some actions
        console.log('Executing agent action...');
        await agent.execute(Buffer.from('example action'));

        // Get agent metrics
        const metrics = agent.getMetrics();
        console.log('Agent metrics:', {
            totalExecutions: metrics.totalExecutions,
            successfulExecutions: metrics.successfulExecutions,
            averageExecutionTime: metrics.averageExecutionTime
        });

        // Update configuration
        console.log('Updating agent configuration...');
        await agent.updateConfig({
            executionLimit: 2000,
            capabilities: ['compute', 'storage', 'network']
        });

        // Pause the agent
        console.log('Pausing agent...');
        await agent.pause();

        // Resume the agent
        console.log('Resuming agent...');
        await agent.resume();

        // Refresh agent data
        console.log('Refreshing agent data...');
        await agent.refresh();

        // Display final state
        console.log('Final agent state:', {
            name: agent.name,
            state: agent.state,
            config: agent.config,
            metadata: agent.metadata
        });

    } catch (error) {
        console.error('Error in example:', error);
    }
}

// Add error handling for the main function
main().catch((error) => {
    console.error('Unhandled error:', error);
    process.exit(1);
});

// Add cleanup handler
process.on('SIGINT', () => {
    console.log('Cleaning up...');
    process.exit(0);
});