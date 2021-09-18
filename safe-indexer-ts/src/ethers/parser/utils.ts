import { EventDecoder } from "."
import { Event, SafeInteractionEvent } from "../.."


export const mapsLogs = async (decoder: EventDecoder | undefined, logs: Event[] | undefined): Promise<SafeInteractionEvent[]> => {
    if (!logs) return []
    if (!decoder) return logs
    return await Promise.all(logs.map(async (e) => {
        try {
            const interaction = await decoder.decode(e)
            if (interaction) return interaction
        } catch (e) {
            console.error(e)
        }
        return e
    }))
}