import { Callback, Loader, Parser, State } from "./types";

const sleep = (timeout: number) => new Promise(cb => setTimeout(cb, timeout))

export class SafeIndexer {

    state: State;
    loader: Loader;
    parser: Parser;
    callback: Callback;

    constructor(state: State, loader: Loader, parser: Parser, callback: Callback) {
        this.state = state;
        this.loader = loader;
        this.parser = parser;
        this.callback = callback;
    }

    async index() {
        while(true) {
            const currentBlock = await this.loader.loadCurrentBlock()
            if (currentBlock <= this.state.lastIndexedBlock) return
            const targetBlock = Math.min(currentBlock, this.state.lastIndexedBlock + 100)
            console.debug("Process from block", this.state.lastIndexedBlock, "to block", targetBlock)
            await this.processBlocks(this.state.lastIndexedBlock, targetBlock)
            this.state.lastIndexedBlock = targetBlock
            await sleep(targetBlock != currentBlock ? 1000 : 15000);
        }
    }

    async processBlocks(from: number, to: number) {
        const events = await this.loader.loadEvents(from, to)
        const interactions = await this.parser.parse(events)
        if (interactions.length > 0) this.callback.onNewInteractions(interactions)
    }
}