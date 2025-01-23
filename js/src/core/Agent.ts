import { PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';
import { EventEmitter } from 'eventemitter3';
import { Program } from './Program';
import {
    AgentState,
    AgentConfig,
    AgentAccount,
    AgentEvents,
    AgentEventType,
    PerformanceMetrics,
    AgentMetadata
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
     * Get agent public key
     */
    public get publicKey(): PublicKey {
        return this.account.publicKey;
    }

    /**
     * Get agent authority
     */
    public get authority(): PublicKey {
        return this.account.authority;
    }

    /**
     * Get agent name
     */
    public get name(): string {
        return this.account.name;
    }

    /**
     * Get agent state
     */
    public get state(): AgentState {
        return this.account.state;
    }

    /**
     * Get agent configuration
     */
    public get config(): AgentConfig {
        return this.account.config;
    }

    /**
     * Get agent metadata
     */
    public get metadata(): AgentMetadata {
        return this.account.metadata;
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