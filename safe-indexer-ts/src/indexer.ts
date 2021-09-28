import { Callback, Loader, Parser, State, IndexerStatus } from "./types";

const sleep = (timeout: number) => new Promise(cb => setTimeout(cb, timeout))

const conditionalSleep = async (timeout?: number) => {
    if (timeout && timeout > 0) await sleep(timeout)
}

export interface Logger {
    log(message?: any, ...optionalParams: any[]): void;
    error(message?: any, ...optionalParams: any[]): void;
}

export interface SafeIndexerUserConfig {
    safe?: string,
    chainId?: number,
    maxBlocks?: number,
    logger?: Logger,
    upToDateTimeout?: number,
    syncTimeout?: number,
    earliestBlock?: number,
    reverse?: boolean
}

export interface SafeIndexerUserConfigUpdate {
    maxBlocks?: number,
    logger?: Logger,
    upToDateTimeout?: number,
    syncTimeout?: number,
    earliestBlock?: number,
    reverse?: boolean
}

export interface SafeIndexerConfig {
    safe?: string,
    chainId?: number,
    maxBlocks: number,
    logger?: Logger,
    earliestBlock: number,
    upToDateTimeout: number,
    syncTimeout: number,
    reverse: boolean
}

const configDefaults = {
    earliestBlock: 0,
    maxBlocks: 100,
    upToDateTimeout: 10000,
    syncTimeout: 100,
    reverse: false
}

export class SafeIndexer {

    state: State;
    loader: Loader;
    parser: Parser;
    callback: Callback;
    indexing: boolean = false;
    paused: boolean = false;
    config: SafeIndexerConfig;

    constructor(state: State, loader: Loader, parser: Parser, callback: Callback, config?: SafeIndexerUserConfig) {
        this.state = state;
        this.loader = loader;
        this.parser = parser;
        this.callback = callback;
        this.config = { ...configDefaults, ...config };
    }

    updateConfig(update: SafeIndexerUserConfigUpdate) {
        this.config = { ...this.config, ...update };
    }

    private postStatusUpdate(status: IndexerStatus) {
        try {
            this.callback?.onStatusUpdate?.(status)
        } catch (e) {
            console.error(e)
        }
    }

    private getCurrentBlockInterval(earliestBlock: number, latestBlock: number, reverse?: boolean): { fromBlock: number, toBlock: number } | undefined {
        console.log(this.state)
        const earliestIndexedBlock = this.state.earliestIndexedBlock
        const lastIndexedBlock = this.state.lastIndexedBlock
        const maxBlocks = this.config.maxBlocks
        if (reverse !== undefined ? reverse : this.config.reverse) {
            if (earliestBlock >= earliestIndexedBlock) {
                return
            }
            return {
                fromBlock: Math.max(earliestBlock, earliestIndexedBlock - maxBlocks),
                toBlock: earliestIndexedBlock - 1
            }
        } else {
            if (latestBlock <= lastIndexedBlock) {
                return
            }
            return {
                fromBlock: lastIndexedBlock + 1,
                toBlock: Math.min(latestBlock, lastIndexedBlock + maxBlocks)
            }
        }
    }

    private ensureBlockDefaults(latestBlock: number) {
        if (this.config.earliestBlock < 0) {
            this.config.earliestBlock = latestBlock
        }
        if (this.state.earliestIndexedBlock < 0) {
            this.state.earliestIndexedBlock = this.config.earliestBlock + 1
        }
        if (this.state.lastIndexedBlock < 0) {
            this.state.lastIndexedBlock = latestBlock - 1
        }
    }

    async start(reverse?: boolean) {
        const activeChainId = await this.loader.loadChainId()
        if (this.config.chainId && activeChainId != this.config.chainId) {
            const errorMsg = `Wrong chain! Expected ${this.config.chainId} got ${activeChainId}`
            this.config.logger?.error(errorMsg)
            this.postStatusUpdate({ type: "aborted", reason: errorMsg, code: 1 });
            return
        }
        this.indexing = true;
        while(this.indexing) {
            if (this.paused) {
                await sleep(this.config.syncTimeout)
                continue
            }
            const latestBlock = await this.loader.loadCurrentBlock()
            this.ensureBlockDefaults(latestBlock);
            const earliestBlock = this.config.earliestBlock;
            const blockInterval = this.getCurrentBlockInterval(earliestBlock, latestBlock, reverse)
            if (!blockInterval) {
                this.config.logger?.log("Up to date with current block!")
                this.postStatusUpdate({ type: "up_to_date", latestBlock, earliestBlock });
                await conditionalSleep(this.config.upToDateTimeout)
                continue
            }
            const { fromBlock, toBlock } = blockInterval;
            
            this.postStatusUpdate({ type: "processing", fromBlock, toBlock, latestBlock, earliestBlock });
            this.config.logger?.log("Process from block", fromBlock, "to block", toBlock)
            try {
                await this.processBlocks(fromBlock, toBlock)
                this.state.lastIndexedBlock = Math.max(toBlock, this.state.lastIndexedBlock)
                this.state.earliestIndexedBlock = Math.min(fromBlock, this.state.earliestIndexedBlock)
                await conditionalSleep(this.config.syncTimeout)
            } catch (e) {
                this.config.logger?.error(e)
                await conditionalSleep(this.config.upToDateTimeout)
            }
        }
    }

    stop() {
        this.indexing = false;
    }

    pause() {
        this.paused = true;
    }

    resume() { 
        this.paused = false;
    }

    async processBlocks(from: number, to: number) {
        const events = await this.loader.loadEvents(from, to, this.config.safe)
        const interactions = await this.parser.parse(events)
        if (interactions.length > 0) try { this.callback.onNewInteractions(interactions) } catch {}
    }
}