use codec::Encode;
use frame_support::{assert_ok, traits::fungible::Inspect};
use xcm::prelude::*;
use xcm_executor::traits::ConvertLocation;
use xcm_simulator::TestExt;

use crate::{
	constants::{ALICE, CENTS, CHARLIE, INITIAL_BALANCE},
	parachain, MockNet, ParaA, ParaC,
};

#[test]
fn reserve_asset_transfer_works() {
	// Scenario:
	// Parachain C wants to register its native token in Parachain A's ForeignAssets pallet.

	MockNet::reset();

	ParaC::execute_with(|| {
		assert_eq!(parachain::Balances::balance(&CHARLIE), INITIAL_BALANCE);
	});

	ParaA::execute_with(|| {
		let para_c_location: Location = (Parent, Parachain(3)).into();
		assert_eq!(parachain::ForeignAssets::balance(para_c_location, &ALICE), 0);
	});

	ParaC::execute_with(|| {
		// Parachain C registers its native token on Parachain A.
		let destination: Location = (Parent, Parachain(1)).into();
		let call = parachain::RuntimeCall::ForeignAssets(pallet_assets::Call::<
			parachain::Runtime,
			pallet_assets::Instance2,
		>::create {
			id: (Parent, Parachain(3)).into(),
			admin: parachain::LocationConverter::convert_location(&destination).unwrap(),
			min_balance: 1,
		});
		let estimated_weight = Weight::from_parts(276_838_000, 3_675);
		let message = Xcm::<()>::builder()
			.withdraw_asset((Here, 50u128 * CENTS))
			.buy_execution((Here, 1u128 * CENTS), Unlimited)
			.transact(OriginKind::Xcm, estimated_weight, call.encode())
			.build();
		assert_ok!(parachain::PolkadotXcm::send_xcm(Here, destination, message,));

		// CHARLIE on Parachain C sends some funds to ALICE on Parachain A.
		let destination: Location = (Parent, Parachain(1)).into();
		let beneficiary: Location =
			AccountId32 { id: ALICE.clone().into(), network: Some(NetworkId::Kusama) }.into();
		// Note how we're using `Here` to reference the local native token.
		// This will be referenced differently by BOB on Parachain A.
		let assets: Assets = (Here, 50u128 * CENTS).into();
		let parachain_a_sovereign_account =
			parachain::LocationConverter::convert_location(&destination).unwrap();
		let old_sov_account_balance = parachain::Balances::balance(&parachain_a_sovereign_account);
		assert_ok!(parachain::PolkadotXcm::transfer_assets(
			parachain::RuntimeOrigin::signed(CHARLIE),
			Box::new(VersionedLocation::V4(destination.clone())),
			Box::new(VersionedLocation::V4(beneficiary)),
			Box::new(VersionedAssets::V4(assets)),
			0,
			WeightLimit::Unlimited,
		));
		// CHARLIE now has less of the native token.
		assert_eq!(parachain::Balances::balance(&CHARLIE), INITIAL_BALANCE - 50 * CENTS);

		// The funds of the sovereign account of Parachain A increase by 50 cents,
		// the ones transferred over to ALICE.
		// The funds in this sovereign account represent how many of Parachain C's native tokens
		// have been sent to this parachain.
		// If the parachain wants to send those assets somewhere else they have to go
		// via the reserve, and this balance is updated accordingly.
		// This is why the derivatives are backed one-to-one.
		let parachain_a_sovereign_account =
			parachain::LocationConverter::convert_location(&destination).unwrap();
		let new_sov_account_balance = parachain::Balances::balance(&parachain_a_sovereign_account);
		assert_eq!(new_sov_account_balance, old_sov_account_balance + 50 * CENTS);
	});

	ParaA::execute_with(|| {
		let parachain_c_location: Location = (Parent, Parachain(3)).into();
		// On the parachain, ALICE has received the derivative tokens.
		assert_eq!(
			parachain::ForeignAssets::balance(parachain_c_location.clone(), &ALICE),
			50 * CENTS
		);

		// ALICE gives back half to CHARLIE on Parachain C.
		let destination: Location = (Parent, Parachain(3)).into();
		let beneficiary: Location =
			AccountId32 { id: CHARLIE.clone().into(), network: Some(NetworkId::Kusama) }.into();
		// We specify `(Parent, Parachain(3))` because we are referencing Parachain C's native
		// token.
		let assets: Assets = ((Parent, Parachain(3)), 25u128 * CENTS).into();
		assert_ok!(parachain::PolkadotXcm::transfer_assets(
			parachain::RuntimeOrigin::signed(ALICE),
			Box::new(VersionedLocation::V4(destination)),
			Box::new(VersionedLocation::V4(beneficiary)),
			Box::new(VersionedAssets::V4(assets)),
			0,
			WeightLimit::Unlimited,
		));

		// ALICE's balance decreased.
		assert_eq!(parachain::ForeignAssets::balance(parachain_c_location, &ALICE), 25 * CENTS);
	});
}
