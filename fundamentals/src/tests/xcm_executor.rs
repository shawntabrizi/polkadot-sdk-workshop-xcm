use frame_support::{assert_ok, traits::fungible::Inspect};
use xcm::latest::prelude::*;
use xcm_builder::{
	ConvertedConcreteId, FrameTransactionalProcessor, FungibleAdapter, IsConcrete, NoChecking,
	NonFungiblesAdapter,
};
use xcm_executor::traits::JustTry;
use xcm_simulator::TestExt;

use chains::parachain::{
	self,
	constants::KsmLocation, location_converter::LocationConverter, AccountId, Balances,
	ForeignUniques,
};
use chains::network::{ParaA, ALICE}; // TODO: Need to make sure to use a `fundamentals` network.
use crate::xcm_executor::*;

type TestAssetTransactor = (
	FungibleAdapter<Balances, IsConcrete<KsmLocation>, LocationConverter, AccountId, ()>,
	NonFungiblesAdapter<
		ForeignUniques,
		ConvertedConcreteId<Location, AssetInstance, JustTry, JustTry>,
		LocationConverter,
		AccountId,
		NoChecking,
		(),
	>,
);

struct Config;
impl XcmConfig for Config {
	type RuntimeCall = parachain::RuntimeCall;
	type AssetTransactor = TestAssetTransactor;
	type TransactionalProcessor = FrameTransactionalProcessor;
	type Barrier = ();
}

#[test]
fn clear_origin_works() {
	let starting_origin: Location = AccountId32 { id: ALICE.into(), network: None }.into();
	let mut executor = XcmExecutor::<Config>::new(starting_origin.clone());

	let message = Xcm::<parachain::RuntimeCall>::builder_unsafe().clear_origin().build();

	assert_eq!(executor.context.origin, Some(starting_origin));
	assert_ok!(executor.execute(message));
	assert_eq!(executor.context.origin, None);
}

#[test]
fn withdraw_works() {
	ParaA::execute_with(|| {
		// Alice should have some non-zero starting balance.
		let alice_original_balance = Balances::balance(&ALICE);

		let message = Xcm::<parachain::RuntimeCall>::builder_unsafe()
			.withdraw_asset((Parent, 100u128))
			.build();
		let origin: Location = AccountId32 { id: ALICE.into(), network: None }.into();

		let mut executor = XcmExecutor::<Config>::new(origin);
		assert_ok!(executor.execute(message));
		assert_eq!(executor.holding.fungible.get(&Parent.into()), Some(&100u128));
		// Alice's balance is updated
		assert_eq!(Balances::balance(&ALICE), alice_original_balance - 100u128);
	});
}

#[test]
fn deposit_asset_works() {
	ParaA::execute_with(|| {
		// Alice might have some non-zero starting balance.
		let alice_original_balance = Balances::balance(&ALICE);

		let asset: Asset = (Parent, 100u128).into();
		let filter: AssetFilter = asset.into();
		let alice_location: Location =
			AccountId32 { id: ALICE.into(), network: None }.into();
		let message = Xcm::<parachain::RuntimeCall>::builder_unsafe()
			.deposit_asset(filter, alice_location)
			.build();

		let mut executor = XcmExecutor::<Config>::new(Parent);

		// Artificially place some assets into the holding.
		executor.holding.subsume((Parent, 100u128).into());
		assert_eq!(executor.holding.fungible.get(&Parent.into()), Some(&100u128));

		// Execute the deposit
		assert_ok!(executor.execute(message));
		// Holding is now empty
		assert_eq!(executor.holding.fungible.get(&Parent.into()), None);
		// Alice's balance is updated
		assert_eq!(Balances::balance(&ALICE), alice_original_balance + 100u128);
	});
}

#[test]
fn transfer_asset_works() {
	ParaA::execute_with(|| {
		// Alice and bob might have some non-zero starting balance.
		let alice_original_balance = Balances::balance(&ALICE);
		const BOB: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([2u8; 32]);

		let bob_original_balance = Balances::balance(&BOB);

		let asset: Asset = (Parent, 100u128).into();
		let alice_location: Location =
			AccountId32 { id: ALICE.into(), network: None }.into();
		let bob_location: Location = AccountId32 { id: BOB.into(), network: None }.into();

		let message = Xcm::<parachain::RuntimeCall>::builder_unsafe()
			.transfer_asset(asset, bob_location)
			.build();

		let mut executor = XcmExecutor::<Config>::new(alice_location);

		// Execute the transfer
		assert_ok!(executor.execute(message));
		// Holding stays empty
		assert_eq!(executor.holding.fungible.get(&Parent.into()), None);
		// Alice and Bob have their balances updated
		assert_eq!(Balances::balance(&ALICE), alice_original_balance - 100u128);
		assert_eq!(Balances::balance(&BOB), bob_original_balance + 100u128);
	});
}
