import { EventDecoder } from "."
import { Event, SafeInteractionEvent } from "../.."


export const mapEvent = async (decoder: EventDecoder | undefined, event: Event): Promise<SafeInteractionEvent> => {
    return {
        event,
        interaction: decoder ? await decoder.decode(event) : undefined
    }
}

export const mapEvents = async (decoder: EventDecoder | undefined, logs: Event[] | undefined): Promise<SafeInteractionEvent[]> => {
    if (!logs) return []
    return await Promise.all(logs.map(async (e) => {
        return mapEvent(decoder, e)
    }))
}