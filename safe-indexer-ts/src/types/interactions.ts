import { MetaTransaction, SafeTransaction } from "@gnosis.pm/safe-contracts"
import { Event } from "./ethereum"

interface Base {
    id: string,
    timestamp: number,
}

export interface ModuleTx extends Base {
    type: 'module_transaction',
    txHash: string,
    success: boolean,
    module: string,
    logs: SafeInteractionEvent[],
    details?: MetaTransaction
}

export interface MultisigTx extends Base {
    type: 'multisig_transaction',
    safeTxHash: string,
    txHash: string,
    success: boolean,
    logs: SafeInteractionEvent[],
    details?: SignedSafeTransaction
}

export interface SignedSafeTransaction extends SafeTransaction {
    signatures: string
}

export interface TransferTx extends Base {
    type: 'transfer'
    sender: string
    receipient: string
    txHash: string,
    direction: 'INCOMING' | 'OUTGOING'
    details: TransferDetails
}

export interface Erc20Details {
    type: "ERC20",
    tokenAddress: string,
    value: string
}

export interface Erc721Details {
    type: "ERC721",
    tokenAddress: string,
    tokenId: string
}

export interface EtherDetails {
    type: "ETHER",
    value: string
}

export type TransferDetails = Erc20Details | Erc721Details | EtherDetails

export type SafeInteraction = MultisigTx | ModuleTx | TransferTx

export type SafeInteractionEvent = SafeInteraction | Event