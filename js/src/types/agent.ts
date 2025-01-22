import { PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';
import { Program } from './Program';
import { EventEmitter } from 'eventemitter3';
import {
    AgentState,
    AgentConfig,
    AgentAccount,
    AgentEvents,
    AgentEventType,
    PerformanceMetrics
} from '../types';
import { SonomaError } from '../errors';

export class Agent extends EventEmitter {
    private program: Program;
    private account: AgentAccount;
    private listeners: Map<AgentEventType, Function[]>;

    private constructor(program: Program, account: AgentAccount) {
        super();
        this.program = program;
        this.account = account;
        this.listeners = new Map();
    }

    /**
     * Create a new agent
     */
    public static async create(
        program: Program,
        name: string,
        config: AgentConfig
    ): Promise<Agent> {
        try {
            const account = await program.createAgentAccount();
            const instruction = await program.createInitializeInstruction(
                account.publicKey,
                name,
                config
            );

            await program.sendTransaction(new Transaction().add(instruction));
            
            const agentAccount = await program.getAgentAccount(account.publicKey);
            return new Agent(program, agentAccount);
        } catch (error) {
            throw new SonomaError('Failed to create agent', { cause: error });
        }
    }

    /**
     * Load an existing agent
     */
    public static async load(
        program: Program,
        address: string | PublicKey
    ): Promise<Agent> {
        try {
            const publicKey = typeof address === 'string' ? new PublicKey(address) : address;
            const account = await program.getAgentAccount(publicKey);
            return new Agent(program, account);
        } catch (error) {
            throw new SonomaError('Failed to load agent', { cause: error });
        }
    }

    /**
     * Update agent configuration
     */
    public async updateConfig(config: Partial<AgentConfig>): Promise<void> {
        try {
            const instruction = await this.program.createUpdateInstruction(
                this.account.publicKey,
                { ...this.account.config, ...config }
            );
            await this.sendTransaction(instruction);
            await this.refresh();
        } catch (error) {
            throw new SonomaError('Failed to update agent configuration', { cause: error });
        }
    }

    /**
     * Execute agent action
     */
    public async execute(data: Buffer): Promise<void> {
        try {
            const instruction = await this.program.createExecuteInstruction(
                this.account.publicKey,
                data
            );
            await this.sendTransaction(instruction);
            this.emit('execution', true, data);
        } catch (error) {
            this.emit('execution', false, error);
            throw new SonomaError('Failed to execute agent action', { cause: error });
        }
    }

    /**
     * Pause agent
     */
    public async pause(): Promise<void> {
        if (this.account.state !== AgentState.Running) {
            throw new SonomaError('Agent must be running to pause');
        }
        
        try {
            const instruction = await this.program.createPauseInstruction(
                this.account.publicKey
            );
            await this.sendTransaction(instruction);
            await this.refresh();
        } catch (error) {
            throw new SonomaError('Failed to pause agent', { cause: error });
        }
    }

    /**
     * Resume agent
     */
    public async resume(): Promise<void> {
        if (this.account.state !== AgentState.Paused) {
            throw new SonomaError('Agent must be paused to resume');
        }
        
        try {
            const instruction = await this.program.createResumeInstruction(
                this.account.publicKey
            );
            await this.sendTransaction(instruction);
            await this.refresh();
        } catch (error) {
            throw new SonomaError('Failed to resume agent', { cause: error });
        }
    }

    /**
     * Get agent state
     */
    public getState(): AgentState {
        return this.account.state;
    }

    /**
     * Get agent configuration
     */
    public getConfig(): AgentConfig {
        return this.account.config;
    }

    /**
     * Get performance metrics
     */
    public getMetrics(): PerformanceMetrics {
        return this.account.metadata.performanceMetrics;
    }

    /**
     * Refresh agent data
     */
    public async refresh(): Promise<void> {
        try {
            const oldState = this.account.state;
            this.account = await this.program.getAgentAccount(this.account.publicKey);
            
            if (oldState !== this.account.state) {
                this.emit('stateChange', oldState, this.account.state);
            }
        } catch (error) {
            throw new SonomaError('Failed to refresh agent data', { cause: error });
        }
    }

    private async sendTransaction(instruction: TransactionInstruction): Promise<void> {
        try {
            await this.program.sendTransaction(new Transaction().add(instruction));
        } catch (error) {
            this.emit('error', error);
            throw error;
        }
    }
}
