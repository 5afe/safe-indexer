import { ethers } from "ethers";
import { Event, Parser, SafeInteraction } from "../../types";
import { detailsTopics, parentTopics } from "../constants";
import { ModuleDecoder } from "./module";
import { MultisigDecoder } from "./multisig";
import { SettingsDecoder } from "./settings";
import { TransferDecoder } from "./transfers";

interface GroupedLogs {
    parent: Event,
    details?: Event, // This is for L2 Safes, we expect the event order details -> children -> parent
    children: Event[]
}

export interface EventDecoder {
    decode(event: Event, subEvents?: Event[], detailEvent?: Event, parentDecoder?: EventDecoder): Promise<SafeInteraction | undefined>
}

export interface ParserConfig {
    provider: ethers.providers.Provider,
    multisigDecoder: MultisigDecoder
}

export class EthersParser implements Parser, EventDecoder {

    provider: ethers.providers.Provider;
    decoders: EventDecoder[];

    constructor(provider: ethers.providers.Provider) {
        this.provider = provider;
        this.decoders = [
            new MultisigDecoder(provider),
            new ModuleDecoder(provider),
            new TransferDecoder(provider),
            new SettingsDecoder(provider)
        ]
    }

    private updateGroupedLogs(groups: GroupedLogs[], detailsCandidate: Event | undefined, parentCandidate: Event | undefined, currentChildren: Event[]) {
        if (parentCandidate) {
            groups.push({
                parent: parentCandidate,
                details: detailsCandidate,
                children: currentChildren
            })
        } else if (currentChildren.length > 0) {
            groups.push(...currentChildren.map((log) => { return { parent: log, children: [] } }))
        }
    }

    private groupIdFromEvent = (event: Event): string => `${event.transactionHash}`

    async group(events: Event[]): Promise<GroupedLogs[]> {
        const out: GroupedLogs[] = []
        let currentChildren: Event[] = []
        let detailsCandidates: Event[] = []
        let parentCandidate: Event | undefined
        let currentGroupId: string | undefined = undefined
        for (const event of events) {
            const groupId = this.groupIdFromEvent(event)
            const isParentCandidate = parentTopics.indexOf(event.topics[0]) >= 0
            const isDetailsCandidate = detailsTopics.indexOf(event.topics[0]) >= 0
            if (currentGroupId !== groupId || (isParentCandidate && parentCandidate)) {
                this.updateGroupedLogs(out, detailsCandidates.pop(), parentCandidate, currentChildren)
                parentCandidate = undefined
                detailsCandidates = []
                currentChildren = []
                currentGroupId = undefined
            }
            if (!currentGroupId) currentGroupId = groupId
            if (isParentCandidate) {
                parentCandidate = event
            } else if (isDetailsCandidate) {
                detailsCandidates.push(event)
            } else {
                currentChildren.push(event)
            }
        }
        this.updateGroupedLogs(out, detailsCandidates.pop(), parentCandidate, currentChildren)
        return out
    }

    async decode(event: Event, subEvents?: Event[], detailEvent?: Event): Promise<SafeInteraction | undefined> {
        for (const decoder of this.decoders) {
            const interaction = await decoder.decode(event, subEvents, detailEvent, this)
            if (interaction) return interaction
        }
    }

    async parse(events: Event[]): Promise<SafeInteraction[]> {
        const groups = await this.group(events)
        const inter = groups.map(({ parent, children, details }) => this.decode(parent, children, details))
        return (await Promise.all(inter)).filter((e) => e !== undefined) as SafeInteraction[]
    }
}