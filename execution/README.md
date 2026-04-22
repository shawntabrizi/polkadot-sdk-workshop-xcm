# execution

Lessons 4–7 live here (writing XCM programs). This crate also hosts the integration tests that gate config lessons 8–10. Everything runs against the multi-chain [`emulator/`](../emulator) so tests exercise the real XCM executor + runtime, not a mock.

## What's here

| File | Role |
|---|---|
| [`src/tests/full.rs`](src/tests/full.rs) | **Lessons 4–7 live here.** Each `#[test] fn` has setup + locals + assertions pre-written; your job is the `let xcm = Xcm::builder()…build()` body. `transfer_n_times` is left solved as a worked reference for nested/recursive XCMs. |
| `src/tests/asset_transactor.rs` | **Read-only gate for lesson 8.** Passes once [`parachain/src/configs/xcm/asset_transactor.rs`](../parachain/src/configs/xcm/asset_transactor.rs) is filled in. |
| `src/tests/barrier.rs` | **Read-only gate for lesson 9.** Passes once [`parachain/src/configs/xcm/barrier.rs`](../parachain/src/configs/xcm/barrier.rs) rejects unpaid XCMs. |
| `src/tests/reserves_and_teleports.rs` | **Read-only gate for lesson 10.** Passes once [`parachain/src/configs/xcm/reserves_and_teleports.rs`](../parachain/src/configs/xcm/reserves_and_teleports.rs) is configured. |
| `src/tests/common.rs` | Shared `setup()` helper — initial balances, pool liquidity. |
| `src/tests/mod.rs`, `src/tests/weigher.rs`, `src/lib.rs` | Module wiring. |

## Running lessons

```sh
# Lessons 4–7 (write the XCM):
cargo test -p execution tests::full::cross_chain_transfer
cargo test -p execution tests::full::transfer_and_transact
cargo test -p execution tests::full::transfer_and_swap
cargo test -p execution tests::full::transfer_swap_and_back

# Lessons 8–10 (config):
cargo test -p execution tests::asset_transactor
cargo test -p execution tests::barrier
cargo test -p execution tests::reserves_and_teleports
```

## How the tests behave in starter state

Lesson-4–7 `let xcm = …` uses `builder_unsafe().build()` (a valid empty XCM). Tests panic in the assertion phase because the program doesn't actually move any assets. Lesson-8–10 tests fail because the config type aliases are `()` (no-op); the `TransactAsset` / `ContainsPair` tuple impls return `AssetNotFound` / `false` by default.

## Using the emulator

All tests pull the chain-and-account prelude from the emulator crate:

```rust
use emulator::prelude::*;
```

That gives you `CustomPara`, `AssetHubWestend`, `Westend` (plus their `…Sender`/`…Receiver` accounts and pallet aliases). See [`../emulator/README.md`](../emulator/README.md) for what each chain is configured with.

## Reference solutions

Mirror path: [`../solutions/execution/src/tests/`](../solutions/execution/src/tests/). Only the 4 lesson tests in `full.rs` have non-trivial bodies in starter state; the other test files are identical between starter and solution (they're gates, not exercises).
