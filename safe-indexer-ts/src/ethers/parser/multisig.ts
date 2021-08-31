import { BigNumber, ethers } from "ethers";
import { calculateSafeTransactionHash, EIP712_SAFE_TX_TYPE } from "@gnosis.pm/safe-contracts";
import { Event, MultisigTx, MultisigTxUnknown } from "../../types";
import { safeAbi, safeInterface } from "../constants";

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
    chainId: number|undefined;

    constructor(provider: ethers.providers.Provider) {
        this.provider = provider
    }

    async init() {
        this.chainId = (await this.provider.getNetwork()).chainId
    }

    calculateHash111(tx: DecodedMultisigTx, nonce: number): string {
        return ethers.utils._TypedDataEncoder.hash({ verifyingContract: tx.safe }, EIP712_SAFE_TX_TYPE, {...tx, nonce})
    }

    async map(expectedHash: string, tx: DecodedMultisigTx): Promise<number> {
        if (!this.chainId) {
            await this.init()
        }

        const safe = new ethers.Contract(tx.safe, safeAbi, this.provider)
        const currentNonce = (await safe.nonce()).toNumber()
        for (let nonce = currentNonce; nonce >= 0; nonce--) {
            if (this.calculateHash111(tx, nonce) === expectedHash) return nonce
            if (calculateSafeTransactionHash(safe, {...tx, nonce}, this.chainId!!) === expectedHash) return nonce
        }
        return -1
    }
}

export class MultisigDecoder {

    provider: ethers.providers.Provider;
    nonceMapper: NonceMapper;

    constructor(provider: ethers.providers.Provider) {
        this.provider = provider;
        this.nonceMapper = new NonceMapper(provider);
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

    decodeMultisigDetailsEvent(event: Event | undefined): DecodedMultisigTx | undefined {
        if (!event) return undefined
        const parsed = safeInterface.decodeEventLog("SafeMultiSigTransaction", event.data, event.topics)
        return {
            safe: event.address,
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

    async decode(safe: string, event: Event, safeTxHash: string, success: boolean, subLogs: Event[], details: Event | undefined): Promise<MultisigTx | MultisigTxUnknown> {
        let decodedTx: DecodedMultisigTx | undefined
        decodedTx = this.decodeMultisigDetailsEvent(details)
        if (!decodedTx) {
            console.log("Fallback to transaction decoding")
            const ethTx = await this.provider.getTransaction(event.transactionHash)
            decodedTx = this.decodeEthereumTx(safe, ethTx)
        }
        const block = await this.provider.getBlock(event.blockHash)
        if (!decodedTx) return {
            type: "multisig_transaction_unknown",
            id: "multisig_" + safeTxHash,
            timestamp: block.timestamp,
            logs: subLogs,
            txHash: event.transactionHash,
            safeTxHash,
            success
        }
        return {
            type: "multisig_transaction",
            id: "multisig_" + safeTxHash,
            timestamp: block.timestamp,
            logs: subLogs,
            txHash: event.transactionHash,
            safeTxHash,
            success,
            ...decodedTx,
            nonce: decodedTx.nonce || await this.nonceMapper.map(safeTxHash, decodedTx)
        } 
    }
}