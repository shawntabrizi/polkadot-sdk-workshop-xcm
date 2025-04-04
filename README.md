# polkadot-sdk-workshop-xcm

This project is a workshop for learning about Polkadot SDK's XCM.

## Get Started

To start the workshop:

```sh
git clone -b fundamentals-0 https://github.com/shawntabrizi/polkadot-sdk-workshop-xcm.git
cd polkadot-sdk-workshop-xcm
```

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

After learning the fundamentals, students should feel confident they have strong understanding of how these underlying XCM primitives function and are constructed.
With this knowledge, they will be able to investigate the real implementations of XCM to learn more deeply if needed.

After fundamentals, students will learn how to craft different XCM programs and execute them.
These include:

1. Cross-chain transfer.
2. Transfer N times.
3. Transfer and transact.
4. Transfer and swap.
5. Transfer, swap and send back.

The final step is investigating the different ways we can configure XCM for various common scenarios.
This workshop will not be comprehensive to all possible interactions, but will focus on a few key scenarios that we commonly expect to see in the Polkadot Ecosystem.

As a parachain:

1. Accepting and using the native asset of your own parachain.
2. Accepting and using the native asset of your relay chain.

Each branch will tell you how to pass the test in its README and show you
the branch to go to for the next exercise.
