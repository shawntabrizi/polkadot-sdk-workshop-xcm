Students will first go through, learn, and use all the fundamental building blocks for XCM:

1. [Location / Topography](location.md)
	- Learn how to construct relative and absolute locations for common objects and types used in XCM.
2. Assets and Filters
	- Learn how to represent various types of assets like fungible tokens and non-fungible tokens.
	- Constructing asset filters to target pools of assets.
3. Instructions
	- Construct common XCM messages through individual XCM instructions.
4. Asset Holding
	- Learn how the XCM Executor manages assets in its own state using the `AssetsInHolding` abstraction.
5. The XCM Executor
	- Learn how the XCM Executor actually functions, and loosely implement a few common instructions needed to complete end to end scenarios.
6. Pallet XCM
	- Learn how Pallet XCM provides a simple to access wrapper to the underlying XCM Executor to perform common tasks like send, execute, and teleport transfers.
