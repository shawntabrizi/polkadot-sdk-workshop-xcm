# polkadot-sdk-workshop-xcm

This project is a workshop for learning about Polkadot SDK's XCM.

## Overview

This workshop aims to teach students about XCM following the philosophy of "discovery through experience".

Students will first go through, learn, and use all the fundamental building blocks for XCM:

- Location / Topography
	- Learn how to construct relative and absolute locations for common objects and types used in XCM.
- Assets and Filters
	- Learn how to represent various types of assets like fungible tokens and non-fungible tokens.
	- Constructing asset filters to target pools of assets.
- Instructions
	- Construct common XCM messages through individual XCM instructions.
- Asset Holding
	- Learn how the XCM Executor manages assets in its own state using the `AssetsInHolding` abstraction.
- The XCM Executor
	- Learn how the XCM Executor actually functions, and loosely implement a few common instructions needed to complete end to end scenarios.
- Pallet XCM
	- Learn how Pallet XCM provides a simple to access wrapper to the underlying XCM Executor to perform common tasks like send, execute, and teleport transfers.

After learning the fundamentals, students should feel confident they have strong understanding of how these underlying XCM primitives function and are constructed. With this knowledge, they will be able to investigate the real implementations of XCM to learn more deeply if needed.

The next step after fundamentals is using the XCM Simulator an investigating the different ways we can configure XCM for various common scenarios. This workshop will not be comprehensive to all possible interactions, but will focus on a few key scenarios that we commonly expect to see in the Polkadot Ecosystem.

As a parachain:

1. Accepting and using the native asset of your relay chain.
2. Accepting and using the native asset of other parachains.
3. Accessing pallets of the relay chain or other parachains.
