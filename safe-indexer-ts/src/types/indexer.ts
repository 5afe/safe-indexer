import { SafeInteraction } from "./interactions";
import { Event } from "./ethereum";

export interface State {
    lastIndexedBlock: number;  
}

export interface Loader { 
    loadChainId(): Promise<number>
    loadCurrentBlock(): Promise<number>
    loadEvents(from: number, to: number, safe?: string): Promise<Event[]>
}

export interface Parser {
    parse(events: Event[]): Promise<SafeInteraction[]>
}

export interface Aborted {
    type: "aborted"
    code: number
    reason: string
}

export interface Processing {
    type: "processing",
    fromBlock: number,
    toBlock: number,
    latestBlock: number
}

export interface UpToDate {
    type: "up_to_date"
    latestBlock: number
}

export type IndexerStatus = Processing | UpToDate | Aborted

export interface Callback {
    onNewInteractions(interactions: SafeInteraction[]): void;
    onStatusUpdate?: (status: IndexerStatus) => void;
}

