import { ethers } from "ethers";
import { Loader, Event } from "../../types";
export * from "./subloaders";

const isOlder = (compare: ethers.providers.Log | undefined, base: ethers.providers.Log | undefined) => {
    if (compare === undefined) return false
    if (base === undefined) return true
    if (compare.blockNumber != base.blockNumber) return compare.blockNumber < base.blockNumber
    if (compare.transactionIndex != base.transactionIndex) return compare.transactionIndex < base.transactionIndex
    if (compare.logIndex != base.logIndex) return compare.logIndex < base.logIndex
    return false // Equal defaults to false
}

export interface SubLoader {
    loadEvents(from: number, to: number, address?: string): Promise<ethers.providers.Log[]>
}

export class EthersLoader implements Loader {

    provider: ethers.providers.Provider;
    subLoaders: SubLoader[];

    constructor(provider: ethers.providers.Provider, subLoaders: SubLoader[]) {
        this.provider = provider
        this.subLoaders = subLoaders
    }

    async loadCurrentBlock(): Promise<number> {
        return await this.provider.getBlockNumber()
    }

    async merge(...loaders: Promise<ethers.providers.Log[]>[]): Promise<ethers.providers.Log[]> {
        const loaderCount = loaders.length
        if (loaderCount == 0) return []
    
        const logResults = await Promise.all(loaders)
        if (loaderCount == 1) return logResults[0]
        const currentLogIndex: number[] = new Array(loaderCount).fill(0)
        for (var i = 0; i < loaderCount; i++) currentLogIndex[i] = 0;
        const out: ethers.providers.Log[] = []
        var runs = 0
        // Panic check against endless loop (10k is max amount of events, per loader)
        while (runs < 10000 * loaderCount) {
            let resultIndex = 0
            let nextLog = logResults[0][currentLogIndex[0]]
            for (var i = 1; i < loaderCount; i++) {
                let candidate = logResults[i][currentLogIndex[i]]
                if (isOlder(candidate, nextLog)) {
                    resultIndex = i
                    nextLog = candidate
                }
            }
            currentLogIndex[resultIndex]++
            if (nextLog) out.push(nextLog)
            else break
            runs++
        }
        return out
    }

    toEvent(logs: ethers.providers.Log): Event {
        return {
            ...logs
        }
    }

    async loadEvents(from: number, to: number): Promise<Event[]> {
        const logs = await this.merge(...this.subLoaders.map(l => l.loadEvents(from, to)))
        return logs.reverse().map(this.toEvent)
    }
}