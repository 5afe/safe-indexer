export interface Event {
    topics: string[],
    data: string,
    address: string,
    blockHash: string,
    transactionHash: string,
    eventId: string,
    account?: string
}
