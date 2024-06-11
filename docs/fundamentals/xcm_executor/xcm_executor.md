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

The main idea behind these types is that we can program the XCM Executor to behave in "configurable" ways behind the use of traits.

- Should funds be manipulated using `pallet_balances` or `orml_balances`?
- Should we limit XCM calls coming from certain places?
- etc...

We will see how these different types are used and get a better idea how they can be used to re-program the XCM Executor.

## Implementation of the XCM Executor

We have included for you a few simple functions for the `XcmExecutor`:

- `new`
- `process`
- `origin_ref`

The `process` function is the main entrypoint for the `XcmExecutor` to process an XCM.

The logic in there is quite simple: take each instruction from the XCM and process that instruction.

### Process Instruction

All of the "real" logic for the `XcmExecutor` lives in `process_instruction`, and it shows a simple example of how you might design a VM system using Rust.

We have a `match` statement over the possible instructions. Each instruction contains the parameters needed for that instruction in the form of fields. On top of all that, remember that we can keep track of additional state in the `XcmExecutor` state.

Next we will go through and implement a few XCM Instructions to learn more about how it all works.
