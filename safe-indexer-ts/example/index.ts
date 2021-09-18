import { ethers } from "ethers";
import { EthersLoader, EthersParser, IncomingEthEventSource, IncomingTransferEventSource, ModuleTransactionEventSource, MultisigTransactionEventSource, OutgoingTransferEventSource, SafeIndexer, SafeInteraction } from "../src"

const provider = new ethers.providers.JsonRpcProvider("https://rinkeby.infura.io/v3/dfa033b2501f41459eb513a7b16e26b7");
const state = { lastIndexedBlock: 9006048 } // 8485873 is the Block of initial 1.3.0 deployment
const loader = new EthersLoader(provider, [
    new MultisigTransactionEventSource(provider),
    new ModuleTransactionEventSource(provider),
    new IncomingEthEventSource(provider),
    new OutgoingTransferEventSource(provider),
    new IncomingTransferEventSource(provider)
])
const parser = new EthersParser(provider)
const callback = {
    onNewInteractions: (interactions: SafeInteraction[]) => console.log(interactions)
}
const indexer = new SafeIndexer(state, loader, parser, callback, { safe: "0x969c350577B6CD3A8E963cBB8D9c728B840c459e", maxBlocks: 10000 })
indexer.start()