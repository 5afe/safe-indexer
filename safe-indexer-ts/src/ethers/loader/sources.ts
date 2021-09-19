import { ethers } from "ethers";
import { EventSource } from ".";
import { etherReceivedTopic, failureTopic, moduleDetailsTopic, moduleFailureTopic, moduleSuccessTopic, multisigDetailsTopic, safeInterface, successTopic, transferTopic } from "../constants";

abstract class BaseEventSource implements EventSource {
    provider: ethers.providers.Provider;
    name: string;

    constructor(name: string, provider: ethers.providers.Provider) {
        this.provider = provider;
        this.name = name
    }

    abstract buildFilter(fromBlock: number | string | undefined, toBlock: number | string | undefined, address?: string): ethers.providers.Filter

    async loadEvents(fromBlock: number | string | undefined, toBlock: number | string | undefined, address?: string): Promise<ethers.providers.Log[]> {
        const filter = this.buildFilter(fromBlock, toBlock, address)
    
        return this.provider.getLogs(filter).then((e) => {
            console.debug(this.name, "source loaded", e.length, "events")
            return e
        })
    }
}

export class StaticTopicEventSource extends BaseEventSource {
    topics: string[];

    constructor(name: string, topics: string[], provider: ethers.providers.Provider) {
        super (name, provider);
        this.topics = topics;
    }

    buildFilter(fromBlock: number | string | undefined, toBlock: number | string | undefined, address?: string) {
        return {
            topics: [this.topics],
            address: address,
            fromBlock,
            toBlock,
        }
    }
}

export class MultisigTransactionEventSource extends StaticTopicEventSource {
    constructor(provider: ethers.providers.Provider) {
        super("MultiSig", [successTopic, failureTopic, multisigDetailsTopic], provider)
    }
}

export class ModuleTransactionEventSource extends StaticTopicEventSource {
    constructor(provider: ethers.providers.Provider) {
        super("Module", [moduleSuccessTopic, moduleFailureTopic, moduleDetailsTopic], provider)
    }
}

export class SettingsChangeEventSource extends StaticTopicEventSource {
    constructor(provider: ethers.providers.Provider) {
        super("Settings", [
            "SafeSetup",
            "AddedOwner",
            "RemovedOwner",
            "ChangedThreshold",
            "EnabledModule",
            "DisabledModule",
            "ChangedFallbackHandler",
            "ChangedGuard",
        ].map((name) => safeInterface.getEventTopic(name)), provider)
    }
}

export class IncomingEthEventSource extends StaticTopicEventSource {
    constructor(provider: ethers.providers.Provider) {
        super("Incoming Ether", [etherReceivedTopic], provider)
    }
}

export class IncomingTransferEventSource extends BaseEventSource {
    constructor(provider: ethers.providers.Provider) {
        super("Incoming transfers", provider)
    }

    buildFilter(fromBlock: number | string | undefined, toBlock: number | string | undefined, address?: string) {
        // Transfer events are only supported if an account address is provided
        if (!address) throw Error("Transfer event source requires an address")
        return {
            topics: [[transferTopic], [ethers.utils.defaultAbiCoder.encode(["address"], [address])]],
            fromBlock,
            toBlock,
        }
    }
}

export class OutgoingTransferEventSource extends BaseEventSource {
    constructor(provider: ethers.providers.Provider) {
        super("Incoming transfers", provider)
    }

    buildFilter(fromBlock: number | string | undefined, toBlock: number | string | undefined, address?: string) {
        // Transfer events are only supported if an account address is provided
        if (!address) throw Error("Transfer event source requires an address")
        return {
            topics: [[transferTopic], null as any, [ethers.utils.defaultAbiCoder.encode(["address"], [address])]],
            fromBlock,
            toBlock,
        }
    }
}