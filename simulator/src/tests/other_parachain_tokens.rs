use frame_support::{
	assert_ok,
	traits::fungible::Inspect,
};
use xcm::prelude::*;
use xcm_executor::traits::ConvertLocation;
use xcm_simulator::TestExt;

use crate::{
	ParaA, ParaB, MockNet, parachain,
	constants::{INITIAL_BALANCE, ALICE, BOB, CENTS},
};

#[test]
fn reserve_asset_transfer_works() {
    // Scenario:
    // ALICE on Parachain B holds some of Parachain B's native token.
    // She transfers some to BOB on Parachain A using a reserve transfer.
    // Parachain A keeps track of the derivatives of all sibling parachains.

    MockNet::reset();

	// ALICE starts with INITIAL_BALANCE on Parachain B.
    ParaB::execute_with(|| {
        assert_eq!(parachain::Balances::balance(&ALICE), INITIAL_BALANCE);
    });

	// BOB starts with 0 on Parachain A.
    ParaA::execute_with(|| {
        assert_eq!(parachain::Balances::balance(&BOB), 0);
    });

    // ALICE on Parachain B sends some of Parachain B's native token to BOB
    // on Parachain A.
	// The transfer is done with the `transfer_assets` extrinsic in the XCM pallet.
    ParaB::execute_with(|| {
        let destination: Location = (Parent, Parachain(1)).into();
        let beneficiary: Location =
			AccountId32 { id: BOB.clone().into(), network: Some(NetworkId::Kusama) }.into();
        // Note how we're using `Here` to reference the local native token.
        // This will be referenced differently by BOB on Parachain A.
        let assets: Assets = (Here, 50u128 * CENTS).into();
		assert_ok!(parachain::PolkadotXcm::transfer_assets(
			parachain::RuntimeOrigin::signed(ALICE),
			Box::new(VersionedLocation::V4(destination.clone())),
			Box::new(VersionedLocation::V4(beneficiary)),
			Box::new(VersionedAssets::V4(assets)),
			0,
			WeightLimit::Unlimited,
		));
        // ALICE now has less of the native token.
		assert_eq!(parachain::Balances::balance(&ALICE), INITIAL_BALANCE - 50 * CENTS);

		// The funds of the sovereign account of Parachain A increase by 50 cents,
		// the ones transferred over to BOB.
		// The funds in this sovereign account represent how many Parachain B native tokens
		// have been sent to this parachain.
		// If the parachain wants to send those assets somewhere else they have to go
		// via the reserve, and this balance is updated accordingly.
		// This is why the derivatives are backed one-to-one.
		let parachain_a_sovereign_account =
			parachain::LocationConverter::convert_location(&destination).unwrap();
		assert_eq!(parachain::Balances::balance(&parachain_a_sovereign_account), 50 * CENTS);
    });

    ParaA::execute_with(|| {
        let parachain_b_location: Location = (Parent, Parachain(2)).into();
		// On the parachain, BOB has received the derivative tokens
		assert_eq!(parachain::ForeignAssets::balance(parachain_b_location.clone(), &BOB), 50 * CENTS);

		// BOB gives back half to ALICE on Parachain B.
		let destination: Location = (Parent, Parachain(2)).into();
		let beneficiary: Location =
			AccountId32 { id: ALICE.clone().into(), network: Some(NetworkId::Kusama) }.into();
		// We specify `(Parent, Parachain(2))` because we are referencing Parachain B's native token.
		let assets: Assets = ((Parent, Parachain(2)), 25u128 * CENTS).into();
		assert_ok!(parachain::PolkadotXcm::transfer_assets(
			parachain::RuntimeOrigin::signed(BOB),
			Box::new(VersionedLocation::V4(destination)),
			Box::new(VersionedLocation::V4(beneficiary)),
			Box::new(VersionedAssets::V4(assets)),
			0,
			WeightLimit::Unlimited,
		));

		// BOB's balance decreased
		assert_eq!(parachain::ForeignAssets::balance(parachain_b_location, &BOB), 25 * CENTS);
    });
}
