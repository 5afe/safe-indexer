import { SafeInteraction } from "./interactions";
import { Event } from "./ethereum";

export interface State {
    lastIndexedBlock: number;  
}

export interface Loader {
    loadCurrentBlock(): Promise<number>
    loadEvents(from: number, to: number, safe?: string): Promise<Event[]>
}

export interface Parser {
    parse(events: Event[]): Promise<SafeInteraction[]>
}

export interface Callback {
    onNewInteractions(interactions: SafeInteraction[]): void;
}

