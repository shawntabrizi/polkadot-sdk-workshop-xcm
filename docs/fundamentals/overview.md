Students will first go through, learn, and use all the fundamental building blocks for XCM:

1. [Location / Topography](location.md)
	- Learn how to construct relative and absolute locations for common objects and types used in XCM.
2. Assets and Filters
	- Learn how to represent various types of assets like fungible tokens and non-fungible tokens.
	- Constructing asset filters to target pools of assets.
3. Asset Holding
	- Learn how we can manage multiple assets in memory using the `AssetsInHolding` abstraction.
4. Instructions
	- Construct common XCM messages through individual XCM instructions.
5. The XCM Executor
	- Learn how the XCM Executor actually functions, and loosely implement a few common instructions needed to complete end to end scenarios.
6. Pallet XCM
	- Learn how Pallet XCM provides a simple to access wrapper to the underlying XCM Executor to perform common tasks like send, execute, and teleport transfers.


## Prerequisite Knowledge

Before we can even start teaching the low level concepts of XCM, we need to provide some high level knowledge about Polkadot, cross consensus messages, and tokens, and more...

## Parachains

TODO

### System Parachains

TODO

## Transfers

We can argue that one of the native operations of any blockchain is to transfer assets and keep track of the ownership of those assets. Within a single consensus system, this is a solved problem.

However, in a multi-chain, multi-token, multi-consensus ecosystem, this is a problem still being solved.

Within the XCM world, we have established two different kinds of transfers which can be used across consensus systems, and based on their trust assumptions.

### Teleport Transfers

In high trust scenarios, we

Teleport:

	Destroying an asset (or amount of funds/token/currency) in one place and minting a corresponding amount in a second place. Imagine the teleporter from Star Trek. The two places need not be equivalent in nature (e.g. could be a UTXO chain that destroys assets and an account-based chain that mints them). Neither place acts as a reserve or derivative for the other. Though the nature of the tokens may be different, neither place is more canonical than the other. This is only possible if there is a bilateral trust relationship both of the STF and the validity/finality/availability between them.

### Reserved Backed Transfers

## XCM

### Principles

XCM is designed around four 'A's:

- Asynchronous: XCM messages in no way assume that the sender will be blocking on its completion.
- Absolute: XCM messages are guaranteed to be delivered and interpreted accurately, in order and in a timely fashion.
- Asymmetric: XCM messages do not have results. Any results must be separately communicated to the sender with an additional message.
- Agnostic: XCM makes no assumptions about the nature of the Consensus System between which messages are being passed.

### Messages

### XCM Virtual Machine
