import {
    Connection as SolanaConnection,
    ConnectionConfig as SolanaConnectionConfig,
    Commitment,
    PublicKey,
    Transaction,
    SendOptions,
    Keypair,
    ConfirmOptions
} from '@solana/web3.js';
import { ConnectionConfig } from '../types';
import { SonomaError } from '../errors';
import { retry } from '../utils/helpers';

export class Connection extends SolanaConnection {
    private config: ConnectionConfig;
    private _wallet: { publicKey: PublicKey; payer: Keypair } | null;

    constructor(
        endpoint: string,
        commitmentOrConfig: Commitment | SolanaConnectionConfig = 'confirmed',
        config: Partial<ConnectionConfig> = {}
    ) {
        super(endpoint, commitmentOrConfig);
        
        this.config = {
            commitment: config.commitment || 'confirmed',
            confirmTransactionInitialTimeout: config.confirmTransactionInitialTimeout || 60000,
            wsEndpoint: config.wsEndpoint || endpoint.replace('http', 'ws'),
            httpHeaders: config.httpHeaders || {}
        };
        
        this._wallet = null;
    }

    /**
     * Set wallet for signing transactions
     */
    public setWallet(wallet: { publicKey: PublicKey; payer: Keypair }): void {
        this._wallet = wallet;
    }

    /**
     * Get current wallet
     */
    public get wallet(): { publicKey: PublicKey; payer: Keypair } {
        if (!this._wallet) {
            throw new SonomaError('Wallet not set');
        }
        return this._wallet;
    }

    /**
     * Send and confirm transaction with retry
     */
    public async sendAndConfirmTransaction(
        transaction: Transaction,
        signers: Array<Keypair>,
        options?: SendOptions
    ): Promise<string> {
        return await retry(
            async () => {
                try {
                    const latestBlockhash = await this.getLatestBlockhash(this.config.commitment);
                    transaction.recentBlockhash = latestBlockhash.blockhash;
                    transaction.lastValidBlockHeight = latestBlockhash.lastValidBlockHeight;

                    transaction.sign(...signers);

                    const signature = await this.sendRawTransaction(
                        transaction.serialize(),
                        {
                            ...options,
                            preflightCommitment: this.config.commitment
                        }
                    );

                    await this.confirmTransaction(
                        {
                            signature,
                            blockhash: latestBlockhash.blockhash,
                            lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
                        },
                        this.config.commitment
                    );

                    return signature;
                } catch (error) {
                    throw new SonomaError('Transaction failed', { cause: error });
                }
            },
            {
                maxAttempts: 3,
                delay: 1000
            }
        );
    }

    /**
     * Get multiple accounts with retry
     */
    public async getMultipleAccounts(
        publicKeys: PublicKey[],
        commitment?: Commitment
    ) {
        return await retry(
            async () => {
                try {
                    return await super.getMultipleAccountsInfo(
                        publicKeys,
                        commitment || this.config.commitment
                    );
                } catch (error) {
                    throw new SonomaError('Failed to fetch accounts', { cause: error });
                }
            },
            {
                maxAttempts: 3,
                delay: 1000
            }
        );
    }

    /**
     * Subscribe to account changes
     */
    public async subscribeAccount(
        publicKey: PublicKey,
        callback: (accountInfo: any) => void,
        commitment?: Commitment
    ): Promise<number> {
        try {
            return this.onAccountChange(
                publicKey,
                callback,
                commitment || this.config.commitment
            );
        } catch (error) {
            throw new SonomaError('Failed to subscribe to account', { cause: error });
        }
    }

    /**
     * Subscribe to program accounts
     */
    public async subscribeProgramAccounts(
        programId: PublicKey,
        callback: (accountInfo: any) => void,
        filters?: any[],
        commitment?: Commitment
    ): Promise<number> {
        try {
            return this.onProgramAccountChange(
                programId,
                callback,
                commitment || this.config.commitment,
                filters
            );
        } catch (error) {
            throw new SonomaError('Failed to subscribe to program', { cause: error });
        }
    }

    /**
     * Get program accounts with filters
     */
    public async getProgramAccounts(
        programId: PublicKey,
        filters?: any[],
        commitment?: Commitment
    ) {
        return await retry(
            async () => {
                try {
                    return await super.getProgramAccounts(
                        programId,
                        {
                            commitment: commitment || this.config.commitment,
                            filters
                        }
                    );
                } catch (error) {
                    throw new SonomaError('Failed to fetch program accounts', { cause: error });
                }
            },
            {
                maxAttempts: 3,
                delay: 1000
            }
        );
    }

    /**
     * Simulate transaction
     */
    public async simulateTransaction(
        transaction: Transaction,
        signers?: Array<Keypair>,
        commitment?: Commitment
    ) {
        try {
            if (signers?.length) {
                transaction.sign(...signers);
            }

            return await super.simulateTransaction(
                transaction,
                undefined,
                commitment || this.config.commitment
            );
        } catch (error) {
            throw new SonomaError('Transaction simulation failed', { cause: error });
        }
    }

    /**
     * Close all WebSocket connections
     */
    public async disconnect(): Promise<void> {
        try {
            // @ts-ignore - Private method access
            if (this._rpcWebSocket) {
                // @ts-ignore - Private method access
                this._rpcWebSocket.close();
            }
            // @ts-ignore - Private method access
            if (this._subscriptionClient) {
                // @ts-ignore - Private method access
                await this._subscriptionClient.close();
            }
        } catch (error) {
            throw new SonomaError('Failed to disconnect', { cause: error });
        }
    }
}