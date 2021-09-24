import { Callback, Loader, Parser, State, IndexerStatus } from "./types";

const sleep = (timeout: number) => new Promise(cb => setTimeout(cb, timeout))

const conditionalSleep = async (timeout?: number) => {
    if (timeout && timeout > 0) await sleep(timeout)
}

export interface Logger {
    log(message?: any, ...optionalParams: any[]): void;
    error(message?: any, ...optionalParams: any[]): void;
}

export interface SafeIndexerConfig {
    safe?: string,
    chainId?: number,
    maxBlocks?: number,
    logger?: Logger,
    upToDateTimeout?: number,
    syncTimeout?: number,
}

const configDefaults = {
    maxBlocks: 100,
    upToDateTimeout: 10000
}

export class SafeIndexer {

    state: State;
    loader: Loader;
    parser: Parser;
    callback: Callback;
    indexing: boolean = false;
    paused: boolean = false;
    config: SafeIndexerConfig;

    constructor(state: State, loader: Loader, parser: Parser, callback: Callback, config?: SafeIndexerConfig) {
        this.state = state;
        this.loader = loader;
        this.parser = parser;
        this.callback = callback;
        this.config = { ...configDefaults, ...config };
        console.log(this.config)
    }

    postStatusUpdate(status: IndexerStatus) {
        try {
            this.callback?.onStatusUpdate?.(status)
        } catch (e) {
            console.error(e)
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
                await sleep(this.config.syncTimeout || 100)
                continue
            }
            const latestBlock = await this.loader.loadCurrentBlock()
            let fromBlock;
            let toBlock;
            if (reverse) {
                fromBlock = this.state.lastIndexedBlock + 1
                toBlock = Math.min(latestBlock, this.state.lastIndexedBlock + (this.config.maxBlocks || 100))
            } else {
                if (latestBlock <= this.state.lastIndexedBlock) {
                    this.config.logger?.log("Up to date with current block!")
                    this.postStatusUpdate({ type: "up_to_date", latestBlock });
                    await conditionalSleep(this.config.upToDateTimeout)
                    continue
                }
                fromBlock = this.state.lastIndexedBlock + 1
                toBlock = Math.min(latestBlock, this.state.lastIndexedBlock + (this.config.maxBlocks || 100))
            }
            this.postStatusUpdate({ type: "processing", fromBlock, toBlock, latestBlock });
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