use frame_support::{
	assert_ok,
	traits::fungible::Inspect,
};
use xcm::prelude::*;
use xcm_executor::traits::ConvertLocation;
use xcm_simulator::TestExt;

use crate::{
	Relay, ParaA, MockNet, parachain, relay_chain,
	constants::{INITIAL_BALANCE, ALICE, BOB, CENTS},
};

#[test]
fn reserve_asset_transfer_works() {
	// Scenario:
	// ALICE on the relay chain holds some of Relay Chain's native tokens.
	// She transfers them to BOB's account on Parachain A using a reserve transfer.
	// BOB receives Relay Chain native token derivatives on the parachain,
	// which are backed one-to-one with the real tokens on the Relay Chain.

	// We reset storage and messages.
	MockNet::reset();

	// ALICE starts with INITIAL_BALANCE on the relay chain.
	Relay::execute_with(|| {
		assert_eq!(relay_chain::Balances::balance(&ALICE), INITIAL_BALANCE);
	});

	// BOB starts with 0 on the parachain.
	ParaA::execute_with(|| {
		assert_eq!(parachain::Balances::balance(&BOB), 0);
	});

	// ALICE on the Relay Chain sends some Relay Chain native tokens to BOB on Parachain A.
	// The transfer is done with the `transfer_assets` extrinsic in the XCM pallet.
	// The extrinsic figures out it should do a reserve asset transfer
	// with the local chain as reserve.
	Relay::execute_with(|| {
		// The parachain id of `ParaA`, defined in `lib.rs`.
		let destination: Location = Parachain(1).into();
		let beneficiary: Location =
			AccountId32 { id: BOB.clone().into(), network: Some(NetworkId::Kusama) }.into();
		// We need to use `u128` here for the conversion to work properly.
		// If we don't specify anything, it will be a `u64`, which the conversion
		// will turn into a non-fungible token instead of a fungible one.
		let assets: Assets = (Here, 50u128 * CENTS).into();
		assert_ok!(relay_chain::XcmPallet::transfer_assets(
			relay_chain::RuntimeOrigin::signed(ALICE),
			Box::new(VersionedLocation::V4(destination.clone())),
			Box::new(VersionedLocation::V4(beneficiary)),
			Box::new(VersionedAssets::V4(assets)),
			0,
			WeightLimit::Unlimited,
		));

		// ALICE now has less Relay Chain tokens.
		assert_eq!(relay_chain::Balances::balance(&ALICE), INITIAL_BALANCE - 50 * CENTS);

		// The funds of the sovereign account of the parachain increase by 50 cents,
		// the ones transferred over to BOB.
		// The funds in this sovereign account represent how many Relay Chain tokens
		// have been sent to this parachain.
		// If the parachain wants to send those assets somewhere else they have to go
		// via the reserve, and this balance is updated accordingly.
		// This is why the derivatives are backed one-to-one.
		let parachains_sovereign_account =
			relay_chain::LocationConverter::convert_location(&destination).unwrap();
		assert_eq!(relay_chain::Balances::balance(&parachains_sovereign_account), 50 * CENTS);
	});

	ParaA::execute_with(|| {
		// On the parachain, BOB has received the derivative tokens
		assert_eq!(parachain::Balances::balance(&BOB), 50 * CENTS);

		// BOB gives back half to ALICE on the relay chain
		let destination: Location = Parent.into();
		let beneficiary: Location =
			AccountId32 { id: ALICE.clone().into(), network: Some(NetworkId::Kusama) }.into();
		// We specify `Parent` because we are referencing the Relay Chain token.
		// This chain doesn't have a token of its own, so we always refer to this token,
		// and we do so by the Location of the Relay Chain.
		let assets: Assets = (Parent, 25u128 * CENTS).into();
		assert_ok!(parachain::PolkadotXcm::transfer_assets(
			parachain::RuntimeOrigin::signed(BOB),
			Box::new(VersionedLocation::V4(destination)),
			Box::new(VersionedLocation::V4(beneficiary)),
			Box::new(VersionedAssets::V4(assets)),
			0,
			WeightLimit::Unlimited,
		));

		// BOB's balance decreased
		assert_eq!(parachain::Balances::balance(&BOB), 25 * CENTS);
	});

	Relay::execute_with(|| {
		// ALICE's balance increases
		assert_eq!(
			relay_chain::Balances::balance(&ALICE),
			INITIAL_BALANCE - 50 * CENTS + 25 * CENTS
		);

		// The funds in the parachain's sovereign account decrease.
		let parachain: Location = Parachain(1).into();
		let parachains_sovereign_account =
			relay_chain::location_converter::LocationConverter::convert_location(&parachain).unwrap();
		assert_eq!(relay_chain::Balances::balance(&parachains_sovereign_account), 25 * CENTS);
	});
}
