use frame_support::{
	assert_ok, construct_runtime, derive_impl, parameter_types,
	traits::{fungible::Inspect, ConstU128},
};
use sp_runtime::{traits::IdentityLookup, BuildStorage};
use xcm::latest::prelude::*;
use xcm_builder::{AccountId32Aliases, FrameTransactionalProcessor, FungibleAdapter, IsConcrete};

construct_runtime! {
	pub struct Runtime {
		System: frame_system = 0,
		Balances: pallet_balances = 1,
	}
}

use crate::{
	constants::{ALICE, INITIAL_BALANCE},
	xcm_executor::{XcmConfig, XcmExecutor},
};

type AccountId = sp_runtime::AccountId32;
type Balance = u128;
type Block = frame_system::mocking::MockBlock<Runtime>;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Runtime {
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
}

fn new_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();

	pallet_balances::GenesisConfig::<Runtime> { balances: vec![(ALICE, INITIAL_BALANCE)] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		sp_tracing::try_init_simple();
		System::set_block_number(1);
	});
	ext
}

parameter_types! {
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub const KsmLocation: Location = Location::parent();
}

type LocationConverter = AccountId32Aliases<RelayNetwork, AccountId>;

type TestAssetTransactor =
	(FungibleAdapter<Balances, IsConcrete<KsmLocation>, LocationConverter, AccountId, ()>,);

struct Config;
impl XcmConfig for Config {
	type AssetTransactor = TestAssetTransactor;
	type TransactionalProcessor = FrameTransactionalProcessor;
}

#[test]
fn clear_origin_works() {
	let starting_origin: Location = AccountId32 { id: ALICE.into(), network: None }.into();
	let mut executor = XcmExecutor::<Config>::new(starting_origin.clone());

	let message = Xcm::<()>::builder_unsafe().clear_origin().build();

	assert_eq!(executor.context.origin, Some(starting_origin));
	assert_ok!(executor.process(message));
	assert_eq!(executor.context.origin, None);
}

#[test]
fn withdraw_works() {
	new_ext().execute_with(|| {
		// Alice should have some non-zero starting balance.
		let alice_original_balance = Balances::balance(&ALICE);

		let message = Xcm::<()>::builder_unsafe().withdraw_asset((Parent, 100u128)).build();
		let origin: Location = AccountId32 { id: ALICE.into(), network: None }.into();

		let mut executor = XcmExecutor::<Config>::new(origin);
		assert_ok!(executor.process(message));
		assert_eq!(executor.holding.fungible.get(&Parent.into()), Some(&100u128));
		// Alice's balance is updated
		assert_eq!(Balances::balance(&ALICE), alice_original_balance - 100u128);
	});
}

#[test]
fn deposit_asset_works() {
	new_ext().execute_with(|| {
		// Alice might have some non-zero starting balance.
		let alice_original_balance = Balances::balance(&ALICE);

		let asset: Asset = (Parent, 100u128).into();
		let filter: AssetFilter = asset.into();
		let alice_location: Location = AccountId32 { id: ALICE.into(), network: None }.into();
		let message = Xcm::<()>::builder_unsafe().deposit_asset(filter, alice_location).build();

		let mut executor = XcmExecutor::<Config>::new(Parent);

		// Artificially place some assets into the holding.
		executor.holding.subsume((Parent, 100u128).into());
		assert_eq!(executor.holding.fungible.get(&Parent.into()), Some(&100u128));

		// Execute the deposit
		assert_ok!(executor.process(message));
		// Holding is now empty
		assert_eq!(executor.holding.fungible.get(&Parent.into()), None);
		// Alice's balance is updated
		assert_eq!(Balances::balance(&ALICE), alice_original_balance + 100u128);
	});
}

#[test]
fn transfer_asset_works() {
	new_ext().execute_with(|| {
		// Alice and bob might have some non-zero starting balance.
		let alice_original_balance = Balances::balance(&ALICE);
		const BOB: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([2u8; 32]);

		let bob_original_balance = Balances::balance(&BOB);

		let asset: Asset = (Parent, 100u128).into();
		let alice_location: Location = AccountId32 { id: ALICE.into(), network: None }.into();
		let bob_location: Location = AccountId32 { id: BOB.into(), network: None }.into();

		let message = Xcm::<()>::builder_unsafe().transfer_asset(asset, bob_location).build();

		let mut executor = XcmExecutor::<Config>::new(alice_location);

		// Execute the transfer
		assert_ok!(executor.process(message));
		// Holding stays empty
		assert_eq!(executor.holding.fungible.get(&Parent.into()), None);
		// Alice and Bob have their balances updated
		assert_eq!(Balances::balance(&ALICE), alice_original_balance - 100u128);
		assert_eq!(Balances::balance(&BOB), bob_original_balance + 100u128);
	});
}
