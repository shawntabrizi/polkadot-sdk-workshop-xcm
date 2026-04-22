# polkadot-sdk-workshop-xcm

A hands-on workshop for learning Polkadot SDK's XCM by filling in TODOs and running tests until everything passes. Work from `master`, edit files in place, check your work with `./run_tests.sh`.

## Get started

```sh
git clone https://github.com/shawntabrizi/polkadot-sdk-workshop-xcm.git
cd polkadot-sdk-workshop-xcm
cargo check --workspace   # should pass: starter state compiles cleanly
./run_tests.sh            # runs every lesson; expect most to fail until completed
```

Requires the Rust toolchain pinned in [`rust-toolchain.toml`](rust-toolchain.toml) (1.84.0, `wasm32-unknown-unknown` target). `rustup` will set it up automatically.

## How the workshop works

Each lesson is a set of TODOs in a specific file. A test command gates completion. You're done with a lesson when the test exits green.

The repo ships with the starter state checked in. Fill in the TODOs in each file, rerun the test, iterate. If you get stuck, the reference solution for every lesson lives under [`solutions/`](solutions/) — try to resist peeking.

## Lessons

| # | Lesson | File to edit | Test command |
|---|---|---|---|
| 1 | [Locations](#1-locations) | `fundamentals/src/location.rs` | `cargo test -p fundamentals --no-default-features --features location` |
| 2 | [Assets](#2-assets) | `fundamentals/src/asset.rs` | `cargo test -p fundamentals --no-default-features --features asset` |
| 3 | [Instructions](#3-instructions) | `fundamentals/src/instruction.rs` | `cargo test -p fundamentals --no-default-features --features instruction` |
| 4 | [Cross-chain transfer](#4-cross-chain-transfer) | `execution/src/tests/full.rs` → `cross_chain_transfer` | `cargo test -p execution tests::full::cross_chain_transfer` |
| 5 | [Transfer and transact](#5-transfer-and-transact) | `execution/src/tests/full.rs` → `transfer_and_transact` | `cargo test -p execution tests::full::transfer_and_transact` |
| 6 | [Transfer and swap](#6-transfer-and-swap) | `execution/src/tests/full.rs` → `transfer_and_swap` | `cargo test -p execution tests::full::transfer_and_swap` |
| 7 | [Transfer, swap, and back](#7-transfer-swap-and-back) | `execution/src/tests/full.rs` → `transfer_swap_and_back` | `cargo test -p execution tests::full::transfer_swap_and_back` |
| 8 | [Config: AssetTransactor](#8-asset-transactor) | `parachain/src/configs/xcm/asset_transactor.rs` | `cargo test -p execution tests::asset_transactor` |
| 9 | [Config: Barrier](#9-barrier) | `parachain/src/configs/xcm/barrier.rs` | `cargo test -p execution tests::barrier` |
| 10 | [Config: Reserves & Teleporters](#10-reserves--teleporters) | `parachain/src/configs/xcm/reserves_and_teleports.rs` | `cargo test -p execution tests::reserves_and_teleports` |

`./run_tests.sh` runs the whole list in order and reports which lessons are failing.

### 1. Locations

Learn how XCM identifies chains, accounts, pallets, and assets relative to a perspective. Fill in the `parameter_types!` blocks in [`fundamentals/src/location.rs`](fundamentals/src/location.rs). See [`docs/fundamentals/location.md`](docs/fundamentals/location.md) for the conceptual intro.

### 2. Assets

Represent fungible tokens, NFTs, and filters that target sets of assets. Fill in [`fundamentals/src/asset.rs`](fundamentals/src/asset.rs).

### 3. Instructions

Compose XCM programs from primitive instructions (`WithdrawAsset`, `DepositAsset`, `BuyExecution`, …). Fill in [`fundamentals/src/instruction.rs`](fundamentals/src/instruction.rs).

### 4–7. Writing XCM programs

Each of these lessons is a test in [`execution/src/tests/full.rs`](execution/src/tests/full.rs) with all the setup, locals, and assertions pre-written. The starter body is an empty `builder_unsafe().build()` with a `// TODO: Add instructions` comment — your job is to fill in the right XCM. The locals defined above the `let xcm = ...` line (`assets_to_withdraw`, `fees_assets`, `destination`, `remote_fees`, etc.) name exactly what you'll need.

- **4. Cross-chain transfer**: move a parachain's native token to the Asset Hub.
- **5. Transfer and transact**: bundle an asset transfer with a remote `Transact` call.
- **6. Transfer and swap**: transfer assets to Asset Hub and swap them there.
- **7. Transfer, swap, and back**: transfer → swap → return assets to the origin parachain.

The `transfer_n_times` test in the same file is left solved as a worked reference for the builder pattern and nested XCMs.

### 8–10. Runtime XCM config

Configure the parachain runtime's XCM executor. Each lesson targets one type:

- **8. AssetTransactor**: how assets are deposited/withdrawn. Edit [`parachain/src/configs/xcm/asset_transactor.rs`](parachain/src/configs/xcm/asset_transactor.rs).
- **9. Barrier**: which incoming XCMs are allowed to execute. Edit [`parachain/src/configs/xcm/barrier.rs`](parachain/src/configs/xcm/barrier.rs).
- **10. Reserves & Teleporters**: which chains this parachain trusts for reserve-backed transfers and teleports. Edit [`parachain/src/configs/xcm/reserves_and_teleports.rs`](parachain/src/configs/xcm/reserves_and_teleports.rs).

In the starter state each of these is `()` (a no-op tuple), so the runtime compiles but the relevant tests fail at runtime. Replace with a real implementation.

## Repository layout

Each of the main folders has its own README with context for contributors and students. The short version:

| Folder | What lives there |
|---|---|
| [`fundamentals/`](fundamentals/README.md) | Lessons 1–3 (location, asset, instruction) — pure-Rust tests, no emulator. |
| [`execution/`](execution/README.md) | Lessons 4–7 (XCM programs) plus integration tests for config lessons 8–10. Uses the emulator. |
| [`parachain/`](parachain/README.md) | The parachain runtime. The config-lesson files live in `src/configs/xcm/`; everything else is standard runtime scaffolding. |
| [`emulator/`](emulator/README.md) | Multi-chain XCM emulator setup (Westend + AssetHubWestend + CustomPara). Used by `execution/` tests. |
| [`solutions/`](solutions/README.md) | Reference solutions that mirror the lesson-file paths. Consumed by `scripts/check-solutions.sh`. |
| [`scripts/`](scripts/README.md) | Maintenance scripts. `check-solutions.sh` is the load-bearing one. |
| [`docs/`](docs/README.md) | Long-form conceptual notes that don't fit in a file header. Sparse right now. |

## Debugging

See [`trace.md`](trace.md) for how to turn on `RUST_LOG=xcm=trace` and step through executor state during a test.

## Reference

- [XCM spec](https://github.com/polkadot-fellows/xcm) — instruction definitions, RFCs, and format changes.
- [polkadot-sdk](https://github.com/paritytech/polkadot-sdk) — the XCM executor, builder, and pallet source.

## Maintainer notes

The SDK pin lives in a single `[workspace.dependencies]` block in the root [`Cargo.toml`](Cargo.toml). To bump:

1. Change the `rev = "..."` in every `workspace.dependencies` entry.
2. Run `./scripts/check-solutions.sh` — temporarily swaps `solutions/` files over the in-place starters, runs the full test suite, then restores. Any solution that breaks is what needs updating after the bump.

`./scripts/check-solutions.sh` is meant for CI and for pre-bump validation.
