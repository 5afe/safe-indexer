import { Callback, Loader, Parser, State } from "./types";

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
        console.log(config)
        this.config = { ...configDefaults, ...config };
        console.log(this.config)
    }

    async start() {
        this.indexing = true;
        while(this.indexing) {
            if (this.paused) {
                await sleep(this.config.syncTimeout || 100)
                continue
            }
            const currentBlock = await this.loader.loadCurrentBlock()
            if (currentBlock <= this.state.lastIndexedBlock) {
                this.config.logger?.log("Up to date with current block!")
                await conditionalSleep(this.config.upToDateTimeout)
                continue
            }
            const targetBlock = Math.min(currentBlock, this.state.lastIndexedBlock + (this.config.maxBlocks || 100))
            this.config.logger?.log("Process from block", this.state.lastIndexedBlock, "to block", targetBlock)
            try {
                await this.processBlocks(this.state.lastIndexedBlock, targetBlock)
                this.state.lastIndexedBlock = targetBlock
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