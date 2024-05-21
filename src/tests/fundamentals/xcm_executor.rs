use crate::{
	fundamentals::xcm_executor::*,
	parachain,
	parachain::{
		constants::KsmLocation, location_converter::LocationConverter, AccountId, Balances,
		ForeignUniques,
	},
	ParaA,
};
use frame_support::assert_ok;
use xcm::latest::prelude::*;
use xcm_builder::{
	ConvertedConcreteId, FrameTransactionalProcessor, FungibleAdapter, IsConcrete, NoChecking,
	NonFungiblesAdapter,
};
use xcm_executor::traits::JustTry;
use xcm_simulator::TestExt;

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
	// TODO
}

// TODO FIX
#[test]
fn withdraw_works() {
	ParaA::execute_with(|| {
		let message = Xcm::<parachain::RuntimeCall>::builder_unsafe()
			.withdraw_asset((Parent, 100u128))
			.build();
		let origin: Location = AccountId32 { id: crate::ALICE.into(), network: None }.into();

		let mut executor = XcmExecutor::<Config>::new(origin);
		assert_ok!(executor.execute(message));
		assert_eq!(executor.holding.fungible.get(&Parent.into()), Some(&100u128));
	});
}

#[test]
fn buy_execution_works() {
	// TODO
}

#[test]
fn deposit_asset_works() {
	// TODO
}

#[test]
fn transact_works() {
	// TODO
}
