import { PublicKey } from '@solana/web3.js';

export enum AgentState {
    Uninitialized = 'uninitialized',
    Initialized = 'initialized',
    Running = 'running',
    Paused = 'paused',
    Error = 'error',
    Terminated = 'terminated'
}

export interface AgentConfig {
    autonomousMode: boolean;
    executionLimit: number;
    memoryLimit: number;
    capabilities: string[];
    metadata?: Record<string, unknown>;
}

export interface AgentCapabilities {
    compute: boolean;
    storage: boolean;
    network: boolean;
    customCapabilities: string[];
}

export interface AgentMetadata {
    createdAt: number;
    updatedAt: number;
    version: string;
    performanceMetrics: PerformanceMetrics;
}

export interface PerformanceMetrics {
    totalExecutions: number;
    successfulExecutions: number;
    failedExecutions: number;
    averageExecutionTime: number;
    totalComputeUnits: number;
}

export interface AgentAccount {
    publicKey: PublicKey;
    authority: PublicKey;
    name: string;
    config: AgentConfig;
    state: AgentState;
    lastExecution: number;
    executionCount: number;
    metadata: AgentMetadata;
}

export interface AgentInstruction {
    initialize(name: string, config: AgentConfig): Promise<void>;
    update(config: AgentConfig): Promise<void>;
    execute(data: Buffer): Promise<void>;
    pause(): Promise<void>;
    resume(): Promise<void>;
}

export interface AgentEvents {
    stateChange: (oldState: AgentState, newState: AgentState) => void;
    execution: (success: boolean, data?: any) => void;
    error: (error: Error) => void;
}

export interface ConnectionConfig {
    commitment?: string;
    confirmTransactionInitialTimeout?: number;
    wsEndpoint?: string;
    httpHeaders?: Record<string, string>;
}

export interface ProgramConfig {
    computeBudget?: number;
    prefetchAccounts?: boolean;
    retryStrategy?: RetryStrategy;
}

export interface RetryStrategy {
    maxAttempts: number;
    baseDelay: number;
    maxDelay: number;
}

// Utility types
export type AgentFilter = {
    authority?: PublicKey;
    state?: AgentState;
    name?: string;
}

export type AgentEventType = keyof AgentEvents;

export type AgentListener = {
    event: AgentEventType;
    callback: (...args: any[]) => void;
}

// Type guards
export function isAgentState(value: any): value is AgentState {
    return Object.values(AgentState).includes(value);
}

export function isAgentConfig(value: any): value is AgentConfig {
    return (
        typeof value === 'object' &&
        typeof value.autonomousMode === 'boolean' &&
        typeof value.executionLimit === 'number' &&
        typeof value.memoryLimit === 'number' &&
        Array.isArray(value.capabilities)
    );
}