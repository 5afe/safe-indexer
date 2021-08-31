import { Event } from "./ethereum"

interface Base {
    id: string,
    timestamp: number,
}

export interface ModuleTx extends Base {
    type: 'module_transaction'
}

export interface ModuleTxUnknown extends Base {
    type: 'module_transaction_unknown'
}

export interface MultisigTx extends Base {
    type: 'multisig_transaction',
    safeTxHash: string,
    txHash: string,
    success: boolean,
    to: string,
    value: string,
    data: string,
    operation: number,
    safeTxGas: string,
    baseGas: string,
    gasPrice: string,
    gasToken: string,
    refundReceiver: string,
    signatures: string,
    nonce: number,
    logs: Event[]
}

export interface MultisigTxUnknown extends Base {
    type: 'multisig_transaction_unknown',
    safeTxHash: string,
    txHash: string,
    success: boolean,
    logs: Event[]
}

export interface TransferTx extends Base {
    type: 'Transfer'
    sender: string
    receipient: string
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

export type SafeInteraction = MultisigTx | ModuleTx | MultisigTxUnknown | ModuleTxUnknown | TransferTx