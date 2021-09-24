import { BigNumber, ethers } from "ethers";
import { calculateSafeTransactionHash, EIP712_SAFE_TX_TYPE } from "@gnosis.pm/safe-contracts";
import { Event, MultisigTx, SignedSafeTransaction, SafeInteractionEvent } from "../../types";
import { failureTopic, safeAbi, safeInterface, successTopic } from "../constants";
import { EventDecoder } from ".";
import { mapEvents } from "./utils";

interface DecodedMultisigTx {
    safe: string,
    to: string
    value: string
    data: string
    operation: number
    safeTxGas: string
    baseGas: string
    gasPrice: string
    gasToken: string
    refundReceiver: string
    signatures: string
    nonce?: number
}

class NonceMapper {

    provider: ethers.providers.Provider;
    chainId: number | undefined;

    constructor(provider: ethers.providers.Provider) {
        this.provider = provider
    }

    async init() {
        this.chainId = (await this.provider.getNetwork()).chainId
    }

    calculateHash111(tx: DecodedMultisigTx, nonce: number): string {
        return ethers.utils._TypedDataEncoder.hash({ verifyingContract: tx.safe }, EIP712_SAFE_TX_TYPE, { ...tx, nonce })
    }

    async map(expectedHash: string, tx: DecodedMultisigTx): Promise<number> {
        if (!this.chainId) {
            await this.init()
        }

        const safe = new ethers.Contract(tx.safe, safeAbi, this.provider)
        const currentNonce = (await safe.nonce()).toNumber()
        for (let nonce = currentNonce; nonce >= 0; nonce--) {
            if (this.calculateHash111(tx, nonce) === expectedHash) return nonce
            if (calculateSafeTransactionHash(safe, { ...tx, nonce }, this.chainId!!) === expectedHash) return nonce
        }
        return -1
    }
}

export class MultisigDecoder implements EventDecoder {

    provider: ethers.providers.Provider;
    useFallbackDecoding: boolean;
    nonceMapper: NonceMapper;

    constructor(provider: ethers.providers.Provider, useFallbackDecoding?: boolean) {
        this.provider = provider;
        this.nonceMapper = new NonceMapper(provider);
        this.useFallbackDecoding = useFallbackDecoding || true;
    }

    decodeEthereumTx(safe: string, tx: ethers.providers.TransactionResponse): DecodedMultisigTx | undefined {
        try {
            if (tx.to !== safe) {
                console.log("Transaction is not to the Safe")
                return undefined
            }
            const result = safeInterface.decodeFunctionData("execTransaction", tx.data)
            return {
                safe,
                to: result.to,
                value: result.value.toString(),
                data: result.data,
                operation: result.operation,
                safeTxGas: result.safeTxGas.toString(),
                baseGas: result.baseGas.toString(),
                gasPrice: result.gasPrice.toString(),
                gasToken: result.gasToken,
                refundReceiver: result.refundReceiver,
                signatures: result.signatures,
            }
        } catch (e) {
            // TODO: try to decode other ways 
            console.log("Unknown function", tx.data.slice(0, 10))
            return undefined
        }
    }

    async decodeFromEthereumHash(safe: string, safeTxHash: string, txHash: string): Promise<SignedSafeTransaction | undefined> {
        console.log("Fallback to transaction decoding")
        const ethTx = await this.provider.getTransaction(txHash)
        const decodedTx = this.decodeEthereumTx(safe, ethTx)
        if (!decodedTx) return undefined;
        return {
            ...decodedTx,
            nonce: await this.nonceMapper.map(safeTxHash, decodedTx)
        }
    }

    decodeMultisigDetailsEvent(event: Event | undefined): SignedSafeTransaction | undefined {
        if (!event) return undefined
        const parsed = safeInterface.decodeEventLog("SafeMultiSigTransaction", event.data, event.topics)
        return {
            to: parsed.to,
            value: parsed.value.toString(),
            data: parsed.data,
            operation: parsed.operation,
            safeTxGas: parsed.safeTxGas.toString(),
            baseGas: parsed.baseGas.toString(),
            gasPrice: parsed.gasPrice.toString(),
            gasToken: parsed.gasToken,
            refundReceiver: parsed.refundReceiver,
            signatures: parsed.signatures,
            nonce: BigNumber.from(parsed.additionalInfo.slice(0, 66)).toNumber()
        }
    }

    async decodeInternal(safe: string, event: Event, safeTxHash: string, success: boolean, subLogs?: Event[], details?: Event, parentDecoder?: EventDecoder): Promise<MultisigTx> {
        const id = "multisig_" + safeTxHash
        let decodedTx: SignedSafeTransaction | undefined = this.decodeMultisigDetailsEvent(details)
        if (!decodedTx && this.useFallbackDecoding) {
            decodedTx = await this.decodeFromEthereumHash(safe, safeTxHash, event.transactionHash)
        }
        const block = await this.provider.getBlock(event.blockHash)
        const txMeta: MultisigTx = {
            type: "multisig_transaction",
            id,
            timestamp: block.timestamp,
            logs: await mapEvents(parentDecoder, subLogs),
            txHash: event.transactionHash,
            safeTxHash,
            success,
            details: decodedTx
        }
        return txMeta
    }

    async decode(event: Event, subEvents?: Event[], detailEvent?: Event, parentDecoder?: EventDecoder): Promise<MultisigTx | undefined> {
        switch (event.topics[0]) {
            case successTopic: {
                const eventParams = safeInterface.decodeEventLog("ExecutionSuccess", event.data, event.topics)
                return await this.decodeInternal(event.address, event, eventParams.txHash, true, subEvents, detailEvent, parentDecoder)
            }
            case failureTopic: {
                const eventParams = safeInterface.decodeEventLog("ExecutionFailure", event.data, event.topics)
                return await this.decodeInternal(event.address, event, eventParams.txHash, false, subEvents, detailEvent, parentDecoder)
            }
            default: {
                return undefined
            }
        }
    }
}