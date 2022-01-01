import { ethers } from "ethers";
import { EthersLoader, EthersParser, IncomingEthEventSource, IncomingTransferEventSource, IndexerStatus, ModuleTransactionEventSource, MultisigTransactionEventSource, OutgoingTransferEventSource, SafeIndexer, SafeInteraction, SettingsChangeEventSource } from "../src"
import dotenv from "dotenv";
dotenv.config()

const provider = new ethers.providers.JsonRpcProvider(process.env.NODE_URL);
const startBlock = -1
const state = { lastIndexedBlock: startBlock, earliestIndexedBlock: startBlock } // 8485873 is the Block of initial 1.3.0 deployment
const loader = new EthersLoader(provider, [
    new MultisigTransactionEventSource(provider),
    new ModuleTransactionEventSource(provider),
    new IncomingEthEventSource(provider),
    new OutgoingTransferEventSource(provider),
    new IncomingTransferEventSource(provider),
    new SettingsChangeEventSource(provider)
])
const parser = new EthersParser(provider)
const callback = {
    onNewInteractions: (interactions: SafeInteraction[]) => console.log(interactions),
    onStatusUpdate: (status: IndexerStatus) => console.log(status)
}
const indexer = new SafeIndexer(state, loader, parser, callback, { chainId: 4, safe: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b", maxBlocks: 10000, earliestBlock: startBlock })

console.log("Indexer started")
indexer.start().then(() => {
    console.log("Indexer stopped")
    process.exit(0)
})

process.stdin.setRawMode(true)
process.stdin.resume()
process.stdin.on('data', (data) => {
    if (data.length === 0) return
    switch (data[0]) {
        case 3:
            console.log("Stopping indexer")
            indexer.stop()
            return
        case 42: {
            const currentConfig = indexer.config
            const newValue = Math.max(1, currentConfig.maxBlocks * 10)
            console.log("Increase maxBlocks by 10x to", newValue)
            indexer.updateConfig({
                maxBlocks: newValue
            })
            return
        }
        case 43: {
            const currentConfig = indexer.config
            const newValue = Math.max(1, currentConfig.maxBlocks + 1000)
            console.log("Increase maxBlocks by 1000 to", newValue)
            indexer.updateConfig({
                maxBlocks: newValue
            })
            return
        }
        case 45: {
            const currentConfig = indexer.config
            const newValue = Math.max(1, currentConfig.maxBlocks - 1000)
            console.log("Decrease maxBlocks by 1000 to", newValue)
            indexer.updateConfig({
                maxBlocks: newValue
            })
            return
        }
        case 47: {
            const currentConfig = indexer.config
            const newValue = Math.max(1, currentConfig.maxBlocks / 10)
            console.log("Decrease maxBlocks by 10x to", newValue)
            indexer.updateConfig({
                maxBlocks: newValue
            })
            return
        }
        case 99: {
            console.log("Load earlier data")
            const currentConfig = indexer.config
            indexer.updateConfig({
                earliestBlock: Math.max(0, currentConfig.earliestBlock - currentConfig.maxBlocks)
            })
            return
        }
        case 114: {
            console.log("Switch indexing direction")
            const currentConfig = indexer.config
            indexer.updateConfig({
                reverse: !currentConfig.reverse
            })
            return
        }

    }
});