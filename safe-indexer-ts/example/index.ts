import { ethers } from "ethers";
import { EthersLoader, EthersParser, MultisigTransactionSubLoader, SafeIndexer, SafeInteraction } from "../src"

const provider = new ethers.providers.JsonRpcProvider("https://bsc-dataseed1.ninicoin.io");
const state = { lastIndexedBlock: 10343000 } // 8485873 is the Block of initial 1.3.0 deployment
const loader = new EthersLoader(provider, [new MultisigTransactionSubLoader(provider)])
const parser = new EthersParser(provider)
const callback = { 
    onNewInteractions: (interactions: SafeInteraction[]) => console.log(interactions) 
}
const indexer = new SafeIndexer(state, loader, parser, callback)
indexer.index()