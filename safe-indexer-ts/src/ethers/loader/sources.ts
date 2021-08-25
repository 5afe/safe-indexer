import { ethers } from "ethers";
import { EventSource } from ".";
import { failureTopic, moduleDetailsTopic, moduleFailureTopic, moduleSuccessTopic, multisigDetailsTopic, successTopic } from "../constants";

class BaseEventSource implements EventSource {
    provider: ethers.providers.Provider;
    topics: string[];
    name: string;

    constructor(name: string, topics: string[], provider: ethers.providers.Provider) {
        this.provider = provider;
        this.topics = topics;
        this.name = name
    }

    async loadEvents(fromBlock: number | string | undefined, toBlock: number | string | undefined, address?: string): Promise<ethers.providers.Log[]> {
        const filter = {
            topics: [this.topics],
            address: address,
            fromBlock,
            toBlock,
        }
    
        return this.provider.getLogs(filter).then((e) => {
            console.debug(this.name, "source loaded", e.length, "events")
            return e
        })
    }
}

export class MultisigTransactionEventSource extends BaseEventSource {
    constructor(provider: ethers.providers.Provider) {
        super("MultiSig", [successTopic, failureTopic, multisigDetailsTopic], provider)
    }
}

export class ModuleTransactionEventSource extends BaseEventSource {
    constructor(provider: ethers.providers.Provider) {
        super("Module", [moduleSuccessTopic, moduleFailureTopic, moduleDetailsTopic], provider)
    }
}