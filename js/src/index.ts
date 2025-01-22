import { Agent } from './core/Agent';
import { Program } from './core/Program';
import { Connection } from './core/Connection';
import { AgentConfig, AgentState, AgentCapabilities } from './types';
import { SonomaError } from './errors';

export class SonomaSDK {
    private connection: Connection;
    private program: Program;

    constructor(endpoint: string, programId: string) {
        this.connection = new Connection(endpoint);
        this.program = new Program(programId, this.connection);
    }

    /**
     * Create a new autonomous agent
     * @param name Agent name
     * @param config Agent configuration
     */
    async createAgent(name: string, config: AgentConfig): Promise<Agent> {
        try {
            return await Agent.create(this.program, name, config);
        } catch (error) {
            throw new SonomaError('Failed to create agent', { cause: error });
        }
    }

    /**
     * Get an existing agent by address
     * @param address Agent's public key
     */
    async getAgent(address: string): Promise<Agent> {
        try {
            return await Agent.load(this.program, address);
        } catch (error) {
            throw new SonomaError('Failed to load agent', { cause: error });
        }
    }

    /**
     * Get the program instance
     */
    getProgram(): Program {
        return this.program;
    }

    /**
     * Get the connection instance
     */
    getConnection(): Connection {
        return this.connection;
    }
}

// Export core classes
export { Agent, Program, Connection };

// Export types
export { 
    AgentConfig, 
    AgentState, 
    AgentCapabilities,
    SonomaError
};

// Export utility functions
export * from './utils/helpers';
export * from './utils/constants';

// Default export
export default SonomaSDK;