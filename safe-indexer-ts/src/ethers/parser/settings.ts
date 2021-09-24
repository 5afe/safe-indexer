import { ethers } from "ethers";
import { EventDecoder } from ".";
import { Event, SetupTx, SettingsChangeTx } from "../../types";
import { safeInterface, } from "../constants";

const setupTopic = safeInterface.getEventTopic("SafeSetup")
const addedOwnerTopic = safeInterface.getEventTopic("AddedOwner")
const removedOwnerTopic = safeInterface.getEventTopic("RemovedOwner")
const changeThresholdTopic = safeInterface.getEventTopic("ChangedThreshold")
const enabledModuleTopic = safeInterface.getEventTopic("EnabledModule")
const disabledModuledTopic = safeInterface.getEventTopic("DisabledModule")
const changeFallbackHandlerTopic = safeInterface.getEventTopic("ChangedFallbackHandler")
const changeGuardTopic = safeInterface.getEventTopic("ChangedGuard")

export class SettingsDecoder implements EventDecoder {

    provider: ethers.providers.Provider;
    useFallbackDecoding: boolean;

    constructor(provider: ethers.providers.Provider, useFallbackDecoding?: boolean) {
        this.provider = provider;
        this.useFallbackDecoding = useFallbackDecoding || true;
    }

    async getSettingsDetails(event: Event): Promise<{component: string, change: 'added' | 'removed' | 'set', value: string } | undefined> {
        switch (event.topics[0]) {
            case addedOwnerTopic: {
                const eventParams = safeInterface.decodeEventLog("AddedOwner", event.data, event.topics)
                return {
                    component: 'owners',
                    change: 'added',
                    value: eventParams.owner
                }
            }
            case removedOwnerTopic: {
                const eventParams = safeInterface.decodeEventLog("RemovedOwner", event.data, event.topics)
                return {
                    component: 'owners',
                    change: 'removed',
                    value: eventParams.owner
                }
            }
            case changeThresholdTopic: {
                const eventParams = safeInterface.decodeEventLog("ChangedThreshold", event.data, event.topics)
                return {
                    component: 'threshold',
                    change: 'set',
                    value: eventParams.threshold.toString()
                }
            }
            case enabledModuleTopic: {
                const eventParams = safeInterface.decodeEventLog("EnabledModule", event.data, event.topics)
                return {
                    component: 'modules',
                    change: 'added',
                    value: eventParams.module
                }
            }
            case disabledModuledTopic: {
                const eventParams = safeInterface.decodeEventLog("DisabledModule", event.data, event.topics)
                return {
                    component: 'modules',
                    change: 'removed',
                    value: eventParams.module
                }
            }
            case changeFallbackHandlerTopic: {
                const eventParams = safeInterface.decodeEventLog("ChangedFallbackHandler", event.data, event.topics)
                return {
                    component: 'fallbackHandler',
                    change: 'set',
                    value: eventParams.handler
                }
            }
            case changeGuardTopic: {
                const eventParams = safeInterface.decodeEventLog("ChangedGuard", event.data, event.topics)
                return {
                    component: 'guard',
                    change: 'set',
                    value: eventParams.guard
                }
            }
            default: {
                return undefined
            }
        }
    }

    async decodeSettingsChange(event: Event): Promise<SettingsChangeTx | undefined> {
        const details = await this.getSettingsDetails(event)
        if (!details) return undefined
        const block = await this.provider.getBlock(event.blockHash)
        return {
            type: "settings",
            id: "settings_" + event.transactionHash + "_" + event.eventId,
            timestamp: block.timestamp,
            txHash: event.transactionHash,
            ...details
        }
    }

    async decode(event: Event): Promise<SettingsChangeTx | SetupTx | undefined> {
        switch (event.topics[0]) {
            case setupTopic: {
                const block = await this.provider.getBlock(event.blockHash)
                return {
                    type: "setup",
                    id: "setup_" + event.transactionHash + "_" + event.eventId,
                    timestamp: block.timestamp,
                    txHash: event.transactionHash,
                }
            }
            default: {
                return this.decodeSettingsChange(event)
            }
        }
    }
}