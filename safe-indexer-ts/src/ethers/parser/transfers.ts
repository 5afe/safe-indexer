import { ethers } from "ethers";
import { EventDecoder } from ".";
import { Event, TransferTx, EtherDetails, TransferDetails } from "../../types";
import { erc20Interface, erc20OldInterface, erc721Interface, etherReceivedTopic, safeInterface, transferTopic, } from "../constants";

export class TransferDecoder implements EventDecoder {

    provider: ethers.providers.Provider;
    useFallbackDecoding: boolean;

    constructor(provider: ethers.providers.Provider, useFallbackDecoding?: boolean) {
        this.provider = provider;
        this.useFallbackDecoding = useFallbackDecoding || true;
    }

    async decodeTokenTransfer(safe: string, event: Event): Promise<TransferTx | undefined> {
        const id = "transfer_" + event.transactionHash + "_" + event.eventId
        const block = await this.provider.getBlock(event.blockHash)
        let type: string = ""
        let eventInterface
        if (event.topics.length === 4) {
            eventInterface = erc721Interface
            type = "ERC721"
        } else if (event.topics.length === 3) {
            eventInterface = erc20Interface
            type = "ERC20"
        } else if (event.topics.length === 2) {
            eventInterface = erc20OldInterface
            type = "ERC20"
        } else {
            return undefined
        }
        const eventParams = eventInterface.decodeEventLog("Transfer", event.data, event.topics)
        let details: TransferDetails
        if (type === "ERC20") {
            details = {
                type: "ERC20",
                tokenAddress: event.address,
                value: eventParams.amount.toString()
            }
        } else {
            details = {
                type: "ERC721",
                tokenAddress: event.address,
                tokenId: eventParams.tokenId.toString()
            }
        }
        return {
            type: "transfer",
            id,
            block: block.number,
            txHash: event.transactionHash,
            timestamp: block.timestamp,
            sender: eventParams.from,
            receipient: eventParams.to,
            direction: (eventParams.to.toLowerCase() === safe.toLowerCase() ? "INCOMING" : "OUTGOING"),
            details
        }
    }

    async decodeEtherTransfer(safe: string, event: Event): Promise<TransferTx | undefined> {
        const id = "transfer_" + event.transactionHash + "_" + event.eventId
        const block = await this.provider.getBlock(event.blockHash)
        const eventParams = safeInterface.decodeEventLog("SafeReceived", event.data, event.topics)
        let details: EtherDetails = {
            type: "ETHER",
            value: eventParams.value.toString()
        }
        const transferMeta: any = {
            type: "transfer",
            id,
            sender: eventParams.sender,
            receipient: safe,
            direction: "INCOMING",
            timestamp: block.timestamp,
            txHash: event.transactionHash,
            details
        }
        return transferMeta
    }

    async decode(event: Event, subEvents?: Event[], detailEvent?: Event): Promise<TransferTx | undefined> {
        switch (event.topics[0]) {
            case transferTopic: {
                if (subEvents && subEvents.length > 0) console.error("Sub logs for transfer entry!", event, subEvents)
                if (!event.account) return undefined
                return await this.decodeTokenTransfer(event.account, event)
            }
            case etherReceivedTopic: {
                if (subEvents && subEvents.length > 0) console.error("Sub logs for transfer entry!", event, subEvents)
                return await this.decodeEtherTransfer(event.address, event)
            }
            default: {
                return undefined
            }
        }
    }
}