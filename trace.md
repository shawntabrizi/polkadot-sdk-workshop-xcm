# XCM Tracing

XCM includes many trace logs to let you follow the path of XCM execution.

```rust
log::trace!(target: "xcm::process_instruction", "=== {:?}", instr);
```

To expose these trace logs, you can run your command with the `RUST_LOG` prefix.

For example:

```sh
RUST_LOG=xcm=trace cargo test -p simulator --features other-parachain-tokens reserve_asset_transfer_works
```

You can also be more specific with which `xcm` logs you want, for example only logs which are `xcm::process_instruction`:

```sh
RUST_LOG=xcm::process_instruction=trace cargo test -p simulator --features other-parachain-tokens reserve_asset_transfer_works
```

The output would look like:

```sh
➜  polkadot-sdk-workshop-xcm git:(master) ✗ RUST_LOG=xcm::process_instruction=trace cargo test -p simulator --features other-parachain-tokens reserve_asset_transfer_works
	Finished `test` profile [unoptimized + debuginfo] target(s) in 0.46s
	Running unittests src/lib.rs (target/debug/deps/simulator-6d2921fa3f073366)

running 1 test
2024-06-14T02:10:26.152349Z TRACE xcm::process_instruction: === TransferAsset { assets: Assets([Asset { id: AssetId(Location { parents: 0, interior: Here }), fun: Fungible(500000000000) }]), beneficiary: Location { parents: 1, interior: X1([Parachain(1)]) } }
2024-06-14T02:10:26.152984Z TRACE xcm::process_instruction: === ReserveAssetDeposited(Assets([Asset { id: AssetId(Location { parents: 1, interior: X1([Parachain(2)]) }), fun: Fungible(500000000000) }]))
2024-06-14T02:10:26.153034Z TRACE xcm::process_instruction: === ClearOrigin
2024-06-14T02:10:26.153042Z TRACE xcm::process_instruction: === BuyExecution { fees: Asset { id: AssetId(Location { parents: 1, interior: X1([Parachain(2)]) }), fun: Fungible(500000000000) }, weight_limit: Unlimited }
2024-06-14T02:10:26.153054Z TRACE xcm::process_instruction: === DepositAsset { assets: Wild(AllCounted(1)), beneficiary: Location { parents: 0, interior: X1([AccountId32 { network: Some(Kusama), id: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1] }]) } }
2024-06-14T02:10:26.153335Z TRACE xcm::process_instruction: === WithdrawAsset(Assets([Asset { id: AssetId(Location { parents: 1, interior: X1([Parachain(2)]) }), fun: Fungible(250000000000) }]))
2024-06-14T02:10:26.153507Z TRACE xcm::process_instruction: === BurnAsset(Assets([Asset { id: AssetId(Location { parents: 1, interior: X1([Parachain(2)]) }), fun: Fungible(250000000000) }]))
2024-06-14T02:10:26.153633Z TRACE xcm::process_instruction: === WithdrawAsset(Assets([Asset { id: AssetId(Location { parents: 0, interior: Here }), fun: Fungible(250000000000) }]))
2024-06-14T02:10:26.153784Z TRACE xcm::process_instruction: === ClearOrigin
2024-06-14T02:10:26.153791Z TRACE xcm::process_instruction: === BuyExecution { fees: Asset { id: AssetId(Location { parents: 0, interior: Here }), fun: Fungible(250000000000) }, weight_limit: Unlimited }
2024-06-14T02:10:26.153799Z TRACE xcm::process_instruction: === DepositAsset { assets: Wild(AllCounted(1)), beneficiary: Location { parents: 0, interior: X1([AccountId32 { network: Some(Kusama), id: [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2] }]) } }
test tests::other_parachain_tokens::reserve_asset_transfer_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.01s
```

## Custom Printing

You might want to peer deeper into the XCM Executor beyond what the trace logs provide.

For that, you will want to have your own local copy of the `polkadot-sdk` so you can plug in your own logs or `println!`.

Here is a helpful example:

1. In `polkadot/xcm/xcm-executor/src/lib.rs`, find `fn process`.
2. Inside `fn process`, you can wrap `self.process_instruction(instr)` with prints:

	```rust
	println!("Universal Location: {:?}", Config::UniversalLocation::get());
	println!("Instruction: {:?}", instr);
	println!("Holding Before: {:?}", self.holding);
	let result = self.process_instruction(instr);
	println!("Holding After: {:?}\n\n", self.holding);
	result
	```

	In this case, we are printing the `UniversalLocation` to see which chain is actually executing the message, and we are looking at how the `holding` is affected by specific instructions.
3. Run any test using the `-- --nocapture` flag. For example:

	```sh
	cargo test -p xcm-simulator-example teleport_nft -- --nocapture
	```

	Output:

	```sh
	➜  polkadot-sdk git:(master) ✗ cargo test -p xcm-simulator-example teleport_nft -- --nocapture
		Finished `test` profile [unoptimized + debuginfo] target(s) in 0.68s
		Running unittests src/lib.rs (target/debug/deps/xcm_simulator_example-9d25b10e3fc85e3a)

	running 1 test
	Universal Location: X1([GlobalConsensus(ByGenesis([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))])
	Instruction: DescendOrigin(X1([AccountId32 { network: None, id: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1] }]))
	Holding Before: AssetsInHolding { fungible: {}, non_fungible: {} }
	Holding After: AssetsInHolding { fungible: {}, non_fungible: {} }


	Universal Location: X1([GlobalConsensus(ByGenesis([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))])
	Instruction: WithdrawAsset(Assets([Asset { id: AssetId(Location { parents: 0, interior: X1([GeneralIndex(1)]) }), fun: NonFungible(Index(69)) }]))
	Holding Before: AssetsInHolding { fungible: {}, non_fungible: {} }
	Holding After: AssetsInHolding { fungible: {}, non_fungible: {(AssetId(Location { parents: 0, interior: X1([GeneralIndex(1)]) }), Index(69))} }


	Universal Location: X1([GlobalConsensus(ByGenesis([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))])
	Instruction: InitiateTeleport { assets: Wild(AllCounted(1)), dest: Location { parents: 0, interior: X1([Parachain(1)]) }, xcm: Xcm([DepositAsset { assets: Wild(AllCounted(1)), beneficiary: Location { parents: 0, interior: X1([AccountId32 { network: None, id: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1] }]) } }]) }
	Holding Before: AssetsInHolding { fungible: {}, non_fungible: {(AssetId(Location { parents: 0, interior: X1([GeneralIndex(1)]) }), Index(69))} }
	Holding After: AssetsInHolding { fungible: {}, non_fungible: {} }


	Universal Location: X2([GlobalConsensus(Kusama), Parachain(1)])
	Instruction: ReceiveTeleportedAsset(Assets([Asset { id: AssetId(Location { parents: 1, interior: X1([GeneralIndex(1)]) }), fun: NonFungible(Index(69)) }]))
	Holding Before: AssetsInHolding { fungible: {}, non_fungible: {} }
	Holding After: AssetsInHolding { fungible: {}, non_fungible: {(AssetId(Location { parents: 1, interior: X1([GeneralIndex(1)]) }), Index(69))} }


	Universal Location: X2([GlobalConsensus(Kusama), Parachain(1)])
	Instruction: ClearOrigin
	Holding Before: AssetsInHolding { fungible: {}, non_fungible: {(AssetId(Location { parents: 1, interior: X1([GeneralIndex(1)]) }), Index(69))} }
	Holding After: AssetsInHolding { fungible: {}, non_fungible: {(AssetId(Location { parents: 1, interior: X1([GeneralIndex(1)]) }), Index(69))} }


	Universal Location: X2([GlobalConsensus(Kusama), Parachain(1)])
	Instruction: DepositAsset { assets: Wild(AllCounted(1)), beneficiary: Location { parents: 0, interior: X1([AccountId32 { network: None, id: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1] }]) } }
	Holding Before: AssetsInHolding { fungible: {}, non_fungible: {(AssetId(Location { parents: 1, interior: X1([GeneralIndex(1)]) }), Index(69))} }
	Holding After: AssetsInHolding { fungible: {}, non_fungible: {} }


	test tests::teleport_nft ... ok

	test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s
	```

You can also combine these two techniques, or add additional logs or prints wherever it makes sense.
