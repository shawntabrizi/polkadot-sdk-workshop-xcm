use crate::{
	fundamentals::{pallet_xcm::pallet as fundamentals_pallet_xcm, xcm_executor::*},
	parachain,
	parachain::{
		constants, constants::KsmLocation, location_converter::LocationConverter, AccountId,
		Balances, ForeignUniques, LocalOriginToLocation, RuntimeOrigin, XcmRouter,
	},
	ParaA, ParaB,
};
use frame_support::{assert_ok, traits::fungible::Inspect};
use fundamentals_pallet_xcm::Pallet as PalletXcm;
use xcm::{latest::prelude::*, VersionedXcm};
use xcm_builder::{
	ConvertedConcreteId, EnsureXcmOrigin, FrameTransactionalProcessor, FungibleAdapter, IsConcrete,
	NoChecking, NonFungiblesAdapter,
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

use crate::parachain::Runtime as Test;

pub struct Config;
impl XcmConfig for Config {
	type RuntimeCall = parachain::RuntimeCall;
	type AssetTransactor = TestAssetTransactor;
	type TransactionalProcessor = FrameTransactionalProcessor;
	type Barrier = ();
}

// NOTE: This pallet isn't actually integrated into the Construct Runtime...
impl fundamentals_pallet_xcm::Config for Test {
	type XcmExecutor = XcmExecutor<Config>;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type UniversalLocation = constants::UniversalLocation;
}

#[test]
fn execute_works() {
	ParaA::execute_with(|| {
		// Alice and bob might have some non-zero starting balance.
		let alice_original_balance = Balances::balance(&crate::ALICE);
		const BOB: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([2u8; 32]);
		let bob_location: Location = AccountId32 { id: BOB.into(), network: None }.into();
		let bob_original_balance = Balances::balance(&BOB);

		let alice_origin: RuntimeOrigin = frame_system::RawOrigin::Signed(crate::ALICE).into();

		let asset: Asset = (Parent, 100u128).into();
		let message = Xcm::<parachain::RuntimeCall>::builder_unsafe()
			.transfer_asset(asset, bob_location)
			.build();
		let versioned_message = Box::new(VersionedXcm::V4(message));
		assert_ok!(PalletXcm::<Test>::execute(alice_origin, versioned_message, Weight::default()));

		// Alice's balance is updated
		assert_eq!(Balances::balance(&crate::ALICE), alice_original_balance - 100u128);
		assert_eq!(Balances::balance(&BOB), bob_original_balance + 100u128);
	});
}
