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

The holding registrar is a key part of enabling end-to-end scenarios between consensus systems. Let's learn why.

### Passing the Holding Registrar

The holding registrar keeps track of all assets in the current XCM state. So after executing all XCM messages on a single consensus system, the holding registrar may have some assets inside of it

### In Memory

The holding registrar is a completely in-memory abstractions. Changes happening here do not necessarily reflect changes to the underlying blockchain state. If you include a new asset into the holding, with the intention of moving or transferring that asset into



### Trust Assumptions


## Other XCM State

The holding registrar is not the only state managed by the XCM executor.

It also has information like:

- `context`: Contextual information about where the message is coming from
-
