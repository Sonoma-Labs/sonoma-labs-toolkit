import { Commitment } from '@solana/web3.js';

/**
 * SDK Configuration
 */
export interface SonomaConfig {
    // Network configuration
    network: NetworkConfig;
    
    // Agent configuration
    agent: AgentDefaultConfig;
    
    // Program configuration
    program: ProgramConfig;
    
    // Global settings
    settings: GlobalSettings;
}

/**
 * Network Configuration
 */
export interface NetworkConfig {
    // Solana cluster endpoint
    endpoint: string;
    
    // WebSocket endpoint (optional)
    wsEndpoint?: string;
    
    // Transaction commitment level
    commitment: Commitment;
    
    // Transaction timeout (ms)
    timeout: number;
    
    // Maximum transaction retries
    maxRetries: number;
}

/**
 * Default Agent Configuration
 */
export interface AgentDefaultConfig {
    // Default execution limits
    executionLimit: number;
    
    // Default memory limits (bytes)
    memoryLimit: number;
    
    // Default capabilities
    defaultCapabilities: string[];
    
    // Auto-retry settings
    retry: RetryConfig;
    
    // Performance thresholds
    performance: PerformanceConfig;
}

/**
 * Program Configuration
 */
export interface ProgramConfig {
    // Program ID
    programId: string;
    
    // Compute budget
    computeBudget: number;
    
    // Account prefetching
    prefetchAccounts: boolean;
    
    // RPC batch size
    batchSize: number;
}

/**
 * Global Settings
 */
export interface GlobalSettings {
    // Logging level
    logLevel: LogLevel;
    
    // Enable debug mode
    debug: boolean;
    
    // Enable telemetry
    telemetry: boolean;
    
    // Cache settings
    cache: CacheConfig;
}

/**
 * Retry Configuration
 */
export interface RetryConfig {
    // Maximum retry attempts
    maxAttempts: number;
    
    // Base delay between retries (ms)
    baseDelay: number;
    
    // Maximum delay between retries (ms)
    maxDelay: number;
    
    // Exponential backoff factor
    backoffFactor: number;
}

/**
 * Performance Configuration
 */
export interface PerformanceConfig {
    // Maximum execution time (ms)
    maxExecutionTime: number;
    
    // Maximum compute units
    maxComputeUnits: number;
    
    // Success rate threshold (%)
    successRateThreshold: number;
}

/**
 * Cache Configuration
 */
export interface CacheConfig {
    // Enable caching
    enabled: boolean;
    
    // Cache TTL (ms)
    ttl: number;
    
    // Maximum cache size
    maxSize: number;
}

/**
 * Log Levels
 */
export enum LogLevel {
    DEBUG = 'debug',
    INFO = 'info',
    WARN = 'warn',
    ERROR = 'error'
}

/**
 * Default configuration
 */
export const DEFAULT_CONFIG: SonomaConfig = {
    network: {
        endpoint: 'https://api.mainnet-beta.solana.com',
        commitment: 'confirmed',
        timeout: 30000,
        maxRetries: 3
    },
    agent: {
        executionLimit: 1000,
        memoryLimit: 1024 * 1024 * 10, // 10MB
        defaultCapabilities: ['compute', 'storage'],
        retry: {
            maxAttempts: 3,
            baseDelay: 1000,
            maxDelay: 10000,
            backoffFactor: 2
        },
        performance: {
            maxExecutionTime: 5000,
            maxComputeUnits: 200000,
            successRateThreshold: 95
        }
    },
    program: {
        programId: '',
        computeBudget: 200000,
        prefetchAccounts: true,
        batchSize: 100
    },
    settings: {
        logLevel: LogLevel.INFO,
        debug: false,
        telemetry: true,
        cache: {
            enabled: true,
            ttl: 300000, // 5 minutes
            maxSize: 1000
        }
    }
};

/**
 * Configuration validation
 */
export function validateConfig(config: Partial<SonomaConfig>): SonomaConfig {
    return {
        ...DEFAULT_CONFIG,
        ...config,
        network: {
            ...DEFAULT_CONFIG.network,
            ...config.network
        },
        agent: {
            ...DEFAULT_CONFIG.agent,
            ...config.agent
        },
        program: {
            ...DEFAULT_CONFIG.program,
            ...config.program
        },
        settings: {
            ...DEFAULT_CONFIG.settings,
            ...config.settings
        }
    };
}