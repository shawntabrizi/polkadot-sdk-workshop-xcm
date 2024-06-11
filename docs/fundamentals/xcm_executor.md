# The XCM Executor

In this section we will dive deep into how the XCM Executor works.

To do this, we will attempt to build our own simple XCM Executor, which has the same basic behavior, but is missing many of the "production ready" features needed for the real code.

Hopefully, by the end of this process, you will have a strong understanding of how the XCM Executor works.

## XCM Executor

Let's start by looking at our version of the XcmExecutor:

```rust
pub struct XcmExecutor<Config: XcmConfig> {
	pub holding: AssetsInHolding,
	pub context: XcmContext,
	_config: PhantomData<Config>,
}
```

You can see here we have a simple `struct` with two fields used to manage the XCM Executor state.

1. The `holding` which you have learned about, and is used to manage assets being manipulated by the XCM.
2. The `context`, which primarily holds the `origin` of the message, but also some metadata like `message_id` and `topic` which we won't cover here.

## XCM Config

We see there is a `Config: XcmConfig` being brought into the `XcmExecutor`.

Similar to the `Config` for pallets, this `XcmConfig` is a vehicle to bring in various associated types and configurations into the `XcmExecutor` environment.

The real `XcmConfig` has MANY associated types and configurations. Ours just focuses on a few key ones:

```rust
pub trait XcmConfig {
	/// How to withdraw and deposit an asset.
	type AssetTransactor: TransactAsset;

	/// Whether we should execute the given XCM at all.
	type Barrier: ShouldExecute;

	/// Transactional processor for XCM instructions.
	type TransactionalProcessor: ProcessTransaction;
}
```
