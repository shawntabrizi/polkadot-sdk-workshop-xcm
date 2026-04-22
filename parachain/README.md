# parachain

A minimal parachain runtime used as the "CustomPara" end of every cross-chain scenario in this workshop. Built from stock `polkadot-sdk` pallets (system, balances, assets, collator-selection, xcm, message-queue, …) — the only interesting files for the workshop are the XCM config pieces.

## What's here

```
src/
├── lib.rs                 # Runtime composition (construct_runtime!), Executive, TxExtension. Read-only.
├── apis.rs                # impl_runtime_apis! — standard runtime APIs. Read-only.
├── configs/
│   ├── mod.rs             # Per-pallet Config impls (system, balances, assets, session, collator-selection, …). Read-only.
│   └── xcm/
│       ├── mod.rs         # XcmConfig, pallet-xcm::Config, XcmRouter. Mostly read-only — the type aliases it uses are what lessons 8–10 fill in.
│       ├── asset_transactor.rs        # ← LESSON 8 — edit this
│       ├── barrier.rs                 # ← LESSON 9 — edit this
│       └── reserves_and_teleports.rs  # ← LESSON 10 — edit this
├── benchmarks.rs          # define_benchmarks! — not exercised by the workshop. Read-only.
└── weights/               # Generated weight files. Read-only.
```

## What you edit

Only the three files under `src/configs/xcm/`:

- `asset_transactor.rs` — how assets move in/out of this parachain. Starter leaves `pub type AssetTransactor = ();` (no-op); both concrete adapters (`LocalFungibleTransactor`, `ForeignFungiblesTransactor`) are already defined above.
- `barrier.rs` — which incoming XCMs can start executing. Starter uses a permissive `AllowAll` placeholder.
- `reserves_and_teleports.rs` — who this parachain trusts to back assets as reserves or to teleport them. Starter leaves both types as `()`.

Each of these compiles cleanly in starter state thanks to tuple impls of `TransactAsset` / `ContainsPair` — so `cargo check --workspace` stays green. The corresponding tests in [`../execution/src/tests/`](../execution/src/tests/) fail at runtime until you fill these in.

## What you don't edit

Everything else. The runtime composition, pallet configs, and APIs are fixed — they exist to make the parachain a plausible target for the XCMs you write. Treat them as "black box" unless you're doing maintenance on the workshop itself.

A few relevant pieces for context when writing XCM configs:

- `RelayLocation = Location::parent()` — the relay chain.
- `HereLocation = Location::here()` — this parachain's own native token.
- `ASSET_HUB_ID = 1000` — used by `reserves_and_teleports.rs`.
- `LocationToAccountId` — how a foreign location becomes a local `AccountId`.
- `CheckingAccount` — the teleport-accounting account.

## Running

This crate builds wasm as part of `cargo check --workspace`. Its only tests are the integration tests in `../execution/`, which exercise the runtime via the emulator. There are no direct unit tests here.

## Reference solutions

Mirror path: [`../solutions/parachain/src/configs/xcm/`](../solutions/parachain/src/configs/xcm/).
