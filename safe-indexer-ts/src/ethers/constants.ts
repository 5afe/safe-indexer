import { getSafeL2SingletonDeployment } from '@gnosis.pm/safe-deployments'
import { ethers } from 'ethers'

export const erc20InterfaceDefinition = [
    "event Transfer(address indexed from, address indexed to, uint256 amount)"
]
export const erc20OldInterfaceDefinition = [
    "event Transfer(address indexed from, address to, uint256 amount)"
]
export const erc721InterfaceDefinition = [
    "event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)"
]
export const erc20Interface = new ethers.utils.Interface(erc20InterfaceDefinition)
export const erc20OldInterface = new ethers.utils.Interface(erc20OldInterfaceDefinition)
export const erc721Interface = new ethers.utils.Interface(erc721InterfaceDefinition)
// The same for all interfaces as `indexed` has no impact on the topic
export const transferTopic = erc20Interface.getEventTopic("Transfer")

export const safeAbi = getSafeL2SingletonDeployment({ released: undefined })!!.abi
export const safeInterface = new ethers.utils.Interface(safeAbi)
export const successTopic = safeInterface.getEventTopic("ExecutionSuccess")
export const failureTopic = safeInterface.getEventTopic("ExecutionFailure")
export const multisigDetailsTopic = safeInterface.getEventTopic("SafeMultiSigTransaction")
export const moduleSuccessTopic = safeInterface.getEventTopic("ExecutionFromModuleSuccess")
export const moduleFailureTopic = safeInterface.getEventTopic("ExecutionFromModuleFailure")
export const moduleDetailsTopic = safeInterface.getEventTopic("SafeModuleTransaction")
export const etherReceivedTopic = safeInterface.getEventTopic("SafeReceived")
// Failure topics cannot generate sub events, we should remove them in the future
export const parentTopics = [successTopic, moduleSuccessTopic, failureTopic, moduleFailureTopic]
export const detailsTopics = [multisigDetailsTopic, moduleDetailsTopic]