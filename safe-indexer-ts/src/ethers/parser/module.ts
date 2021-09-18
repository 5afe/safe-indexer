import { MetaTransaction } from "@gnosis.pm/safe-contracts";
import { ethers } from "ethers";
import { EventDecoder } from ".";
import { Event, ModuleTx, SafeInteractionEvent } from "../../types";
import { moduleFailureTopic, moduleSuccessTopic, safeInterface } from "../constants";
import { mapsLogs } from "./utils";

export class ModuleDecoder implements EventDecoder {

    provider: ethers.providers.Provider;
    useFallbackDecoding: boolean;

    constructor(provider: ethers.providers.Provider, useFallbackDecoding?: boolean) {
        this.provider = provider;
        this.useFallbackDecoding = useFallbackDecoding || true;
    }

    decodeEthereumTx(safe: string, tx: ethers.providers.TransactionResponse): MetaTransaction | undefined {
        // TODO: implement
        return undefined
    }

    async decodeFromEthereumHash(safe: string, txHash: string): Promise<MetaTransaction | undefined> {
        console.log("Fallback to transaction decoding")
        const ethTx = await this.provider.getTransaction(txHash)
        return this.decodeEthereumTx(safe, ethTx)
    }

    decodeModuleDetailsEvent(event: Event | undefined): MetaTransaction | undefined {
        if (!event) return undefined
        const parsed = safeInterface.decodeEventLog("SafeModuleTransaction", event.data, event.topics)
        return {
            to: parsed.to,
            value: parsed.value.toString(),
            data: parsed.data,
            operation: parsed.operation,
        }
    }

    async decodeInternal(safe: string, module: string, event: Event, success: boolean, subLogs?: Event[], details?: Event, parentDecoder?: EventDecoder): Promise<ModuleTx> {
        let decodedTx: MetaTransaction | undefined
        const id = "module_" + event.transactionHash + "_" + event.eventId
        decodedTx = this.decodeModuleDetailsEvent(details)
        if (!decodedTx && this.useFallbackDecoding) {
            decodedTx = await this.decodeFromEthereumHash(safe, event.transactionHash);
        }
        const block = await this.provider.getBlock(event.blockHash)
        const txMeta: ModuleTx = {
            type: "module_transaction",
            id,
            timestamp: block.timestamp,
            logs: await mapsLogs(parentDecoder, subLogs),
            txHash: event.transactionHash,
            module,
            success,
            details: decodedTx
        }
        return txMeta
    }

    async decode(event: Event, subEvents?: Event[], detailEvent?: Event, parentDecoder?: EventDecoder): Promise<ModuleTx | undefined> {
        switch (event.topics[0]) {
            case moduleSuccessTopic: {
                const eventParams = safeInterface.decodeEventLog("ExecutionFromModuleSuccess", event.data, event.topics)
                return await this.decodeInternal(eventParams.address, eventParams.module, event, true, subEvents, detailEvent, parentDecoder)
            }
            case moduleFailureTopic: {
                const eventParams = safeInterface.decodeEventLog("ExecutionFromModuleFailure", event.data, event.topics)
                return await this.decodeInternal(eventParams.address, eventParams.module, event, false, subEvents, detailEvent, parentDecoder)
            }
            default: {
                return undefined
            }
        }
    }
}