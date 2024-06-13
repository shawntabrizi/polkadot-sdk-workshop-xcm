# Holding

Now that we have learned how to construct various kinds of assets and their locations, we can look more deeply how we would actually interact with multiple assets within the XCM system.

## Holding Registrar

To manage multiple assets within XCM, we use an abstraction called the "holding registrar" and a structure representing this abstraction called `AssetsInHolding`.

```rust
/// Map of non-wildcard fungible and non-fungible assets held in the holding register.
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct AssetsInHolding {
	/// The fungible assets.
	pub fungible: BTreeMap<AssetId, u128>,

	/// The non-fungible assets.
	pub non_fungible: BTreeSet<(AssetId, AssetInstance)>,
}
```

This structure keeps track of all assets which are currently being processed by XCM.

As you can see, the `AssetsInHolding` uses a `BTreeMap` and `BTreeSet` to manage fungible and non-fungible assets respectively. The holding registrar should be treated as a single pool of assets, and there should only be a single instance of any asset in the holding. If we want to include some assets into the holding, we should check if the asset already exists, and increase that value if so. Otherwise, we place that asset into the holding for the first time.

## Cross-Chain Uses

The holding registrar is a key part of enabling end-to-end scenarios between consensus systems. Let's learn how it works and how it is used.

### In Memory

The holding registrar is a completely in-memory abstractions. Changes happening here do not necessarily reflect changes to the underlying blockchain state. If you include a new asset into the holding, you must also perform actions on the local chain to "remove" assets from the local state, to keep all balances in sync.

Similarly, when moving assets from the holding registrar back to the local chain, you will need to mint that asset locally in addition to removing it from the registrar.

You can really think of the holding registrar as an in memory abstraction that you can interact with through the XCM executor.

### Passing the Holding Registrar

The holding registrar will keep track of all assets in the current XCM state.

After executing all XCM messages on a single consensus system, the holding registrar may have some assets inside of it with the intention to move those assets to a new consensus system.

Let's talk through a simple example:

- You want to **teleport** some DOT tokens from the Relay Chain to the Asset Hub parachain.
- First, on the Relay Chain, you need destroy some DOT on the local chain, and move it to the holding registrar.
- Then, a copy of the assets in the holding registrar will be sent via XCMP to the Asset Hub.
- The Asset Hub sees in the holding registrar there is some DOT token, and in the XCM message, that the Asset Hub should use this to mint new DOT token in its consensus system.
- The Asset Hub mints new DOT token locally, and reduces the values the assets in the holding registrar.
- Now that the holding registrar is empty of assets, the XCM message can gracefully end.

### Trust Assumptions

Take note that in the scenario above, there is a need for **trust**.

For the Asset Hub to mint new DOT token in its local chain, it needs to trust that the Polkadot Relay Chain actually destroyed the same amount of DOT, and that the holding registrar and the XCM instructions were all accurate. Of course, in the context of the Polkadot Network, all chains in the whole ecosystem need to have some inherent trust in the Relay Chain.

Polkadot would trust similar messages coming from the Asset Hub because the Asset Hub is also just an extension of the Relay Chain logic. So these are already high trust scenarios.

In the current state of XCM, there is nothing better we can do here. Two different consensus systems need some amount of trust to interact with each other. In the future, we will be able to provide stronger guarantees for both systems using "accords", which will use

## Other XCM State

The holding registrar is not the only state managed by the XCM executor.

It also has information like:

- `context`: Contextual information about where the message is coming from
-
