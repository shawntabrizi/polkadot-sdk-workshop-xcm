# Locations

In the world of XCM, the first fundamental primitive you need to learn about is Locations.

As a software developer, Locations are most similar to filepaths in a filesystem.

When writing a program, filepaths can help you locate folders, which contain many items, and the items themselves.

This is very similar to Locations in XCM, which can be used to locate consensus systems, like Relay Chains, Parachains, Smart Contracts, etc... and also specific users, assets, applications, and groups.

Let's explore how Locations work inside of XCM.

## Relative Locations

By default, locations in XCM are always relative.

This means, when considering how to construct a location, you must first establish your relative perspective.

For example, take the following topography:

```text
                  ┌───────────┐
                  │  Relay A  │
                  │  Polkadot │
                  └─────┬─────┘
                        │
             ┌──────────┴──────────┐
             │                     │
       ┌─────┴─────┐         ┌─────┴─────┐
       │  Para A   │         │  Para B   │
       │  Id 1000  │         │  Id 1337  │
       └─────┬─────┘         └─────┬─────┘
             │                     │
      ┌──────┴──────┐              ├───────────┐
      │             │              │           │
┌─────┴─────┐ ┌─────┴─────┐ ┌──────┴────┐ ┌────┴──────┐
│   Alice   │ │  Pallet   │ │    Bob    │ │  Pallet   │
│ AcctKey32 │ │  Assets   │ │ AcctKey20 │ │   EVM     │
│           │ │           │ │           │ │           │
│ 0x11111...│ │ Pallet #2 │ │ 0x22222...│ │ Pallet #5 │
└───────────┘ └─────┬─────┘ └───────────┘ └─────┬─────┘
                    │                           │
              ┌─────┴─────┐               ┌─────┴─────┐
              │   Asset   │               │   Smart   │
              │   USDT    │               │ Contract  │
              │           │               │           │
              │  Id 1984  │               │ 0x55555...│
              └───────────┘               └───────────┘
```

Let's treat this like a file system, and say we want to locate `Asset wBTC - ID 21`.

From the perspective of the Polkadot relay chain:

```text
./{Para A}/{Pallet Assets}/{Asset wBTC}
```

But from the perspective of Smart Contract `0x555...`, which might be a DEX used to trade `wBTC`:

```text
../../../{Para A}/{Pallet Assets}/{Asset wBTC}
```

## The Location Format

So nothing new to learn about with locations, we just need to break down how they are represented in the XCM system.

### Junction

The building blocks of a location are Junctions.

```rust
pub enum Junction {
	Parachain(#[codec(compact)] u32),
	PalletInstance(u8),
	GeneralIndex(#[codec(compact)] u128),
	// ... there are more junctions
}
```
