import { ethers } from "ethers";
import { Event, Parser, SafeInteraction } from "../../types";
import { detailsTopics, failureTopic, moduleFailureTopic, moduleSuccessTopic, parentTopics, safeInterface, successTopic } from "../constants";
import { MultisigDecoder } from "./multisig";

interface GroupedLogs {
    parent: Event,
    details?: Event, // This is for L2 Safes, we expect the event order details -> children -> parent
    children: Event[]
}

export interface ParserConfig {
    provider: ethers.providers.Provider,
    multisigDecoder: MultisigDecoder
}

export class EthersParser implements Parser {

    provider: ethers.providers.Provider;
    multisigDecoder: MultisigDecoder;
    safe?: string;

    constructor(provider: ethers.providers.Provider) {
        this.provider = provider;
        this.multisigDecoder = new MultisigDecoder(provider);
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

    async map(group: GroupedLogs): Promise<SafeInteraction | undefined> {
        const { parent, children, details } = group
        switch (parent.topics[0]) {
            case successTopic: {
                const event = safeInterface.decodeEventLog("ExecutionSuccess", parent.data, parent.topics)
                return await this.multisigDecoder.decode(parent.address, parent, event.txHash, true, children, details)
            }
            case failureTopic: {
                const event = safeInterface.decodeEventLog("ExecutionFailure", parent.data, parent.topics)
                return await this.multisigDecoder.decode(parent.address, parent, event.txHash, true, children, details)
            }
            case moduleSuccessTopic: {
                const event = safeInterface.decodeEventLog("ExecutionFromModuleSuccess", parent.data, parent.topics)
                return undefined // await moduleTxEntry(provider, parent, event.module, true, children, details)
            }
            case moduleFailureTopic: {
                const event = safeInterface.decodeEventLog("ExecutionFromModuleFailure", parent.data, parent.topics)
                return undefined // await moduleTxEntry(provider, parent, event.module, false, children, details)
            }
            default: {
                console.error("Received unknown event", parent)
                return undefined
            }
        }
    }

    async parse(events: Event[]): Promise<SafeInteraction[]> {
        const groups = await this.group(events)
        const inter = groups.map((group) => this.map(group))
        return (await Promise.all(inter)).filter((e) => e !== undefined) as SafeInteraction[]
    }
}