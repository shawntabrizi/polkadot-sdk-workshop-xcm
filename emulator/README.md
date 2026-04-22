# emulator

Wires up a three-chain XCM test network that `../execution/` tests run against:

| Chain | Para ID | Role |
|---|---|---|
| `Westend` | (relay) | The relay chain. Supplies native token WND and the reserve for it. |
| `AssetHubWestend` | 1000 | System parachain for assets. Hosts `ForeignAssets` used for cross-chain transfers. |
| `CustomPara` | 2000 | The parachain defined by [`../parachain/`](../parachain). Where most "your" actions start. |

Built on [`xcm-emulator`](https://github.com/paritytech/polkadot-sdk/tree/master/cumulus/xcm/xcm-emulator) (`decl_test_networks!` + `decl_test_sender_receiver_accounts_parameter_types!`). The emulator runs a real message queue, real XCM executor, real pallets — just without the consensus / networking layers of a full node.

## What's here

```
src/
├── lib.rs                      # Prelude — re-exports chains, pallet aliases, account helpers, constants.
├── network/mod.rs              # `decl_test_networks!` wiring, sender/receiver accounts.
├── westend/
│   ├── mod.rs                  # Pallet alias macros for the Westend relay.
│   └── genesis.rs              # Genesis storage: initial balances, validators.
├── asset_hub_westend/
│   ├── mod.rs                  # Pallet aliases for AssetHubWestend.
│   └── genesis.rs              # Registers the `PARA/WND` pool + liquidity so swap lessons have something to trade.
└── parachain/
    ├── mod.rs                  # Pallet aliases for CustomPara.
    └── genesis.rs              # CustomPara's genesis (para_id = 2000).
```

## Using it from a test

All you typically need is the prelude:

```rust
use emulator::prelude::*;

// Chains: Westend, AssetHubWestend, CustomPara
// Accounts: WestendSender, WestendReceiver, AssetHubWestendSender, AssetHubWestendReceiver, CustomParaSender, CustomParaReceiver
// Pallet aliases: WestendPallet, AssetHubWestendPallet, CustomParaPallet
// Constants: PARA_UNITS, PARA_CENTS, WND_UNITS, WND_CENTS
// Macros: assert_expected_events!, Chain, Parachain, TestExt

CustomPara::execute_with(|| {
    // runs on CustomPara — can reach its pallets, origins, storage
});
```

See [`execution/src/tests/common.rs`](../execution/src/tests/common.rs) for the shared `setup()` helper that puts initial balances and pool liquidity in place before each scenario.

## What you probably shouldn't edit

Everything here. The emulator configuration is plumbing the workshop relies on. If you need to change behavior, you almost certainly want to edit the parachain runtime or the XCM that runs across it — not the emulator itself.

Exceptions: if you're extending the workshop (e.g. adding a new chain or a new initial asset), this is the right place.
