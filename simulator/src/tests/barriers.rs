use frame_support::{assert_ok, traits::fungible::Inspect};
use xcm::prelude::*;
use xcm_simulator::TestExt;

use crate::{
	asset_hub,
	constants::{ALICE, CENTS, INITIAL_BALANCE},
	parachain, relay_chain, AssetHub, MockNet, ParaC, Relay,
};

#[test]
fn unpaid_xcm_from_sibling_should_fail() {
	MockNet::reset();

	ParaC::execute_with(|| {
		let destination: Location = (Parent, Parachain(1000)).into();
		let message = Xcm::builder_unsafe()
			.withdraw_asset((Parent, 100u128))
			.deposit_asset(All, AccountId32 { id: ALICE.into(), network: Some(Kusama) })
			.build();
		// The message can be sent.
		assert_ok!(parachain::PolkadotXcm::send_xcm(Here, destination, message,));
	});

	AssetHub::execute_with(|| {
		// But it doesn't really get executed.
		assert_eq!(asset_hub::Balances::balance(&ALICE), INITIAL_BALANCE);
	});
}

#[test]
fn paid_xcm_from_sibling_should_work() {
	MockNet::reset();

	let total_amount = 50 * CENTS;
	let fee_amount = 2 * CENTS;

	ParaC::execute_with(|| {
		let destination: Location = (Parent, Parachain(1000)).into();
		let message = Xcm::builder()
			.withdraw_asset((Parent, total_amount))
			.buy_execution((Parent, fee_amount), Unlimited)
			.deposit_asset(All, AccountId32 { id: ALICE.into(), network: Some(Kusama) })
			.build();
		// The message can be sent.
		assert_ok!(parachain::PolkadotXcm::send_xcm(Here, destination, message,));
	});

	AssetHub::execute_with(|| {
		// This time it does get executed.
		assert_eq!(asset_hub::Balances::balance(&ALICE), INITIAL_BALANCE + total_amount);
	});
}

#[test]
fn unpaid_xcm_from_parent_should_work() {
	MockNet::reset();

	let total_amount = 50 * CENTS;

	Relay::execute_with(|| {
		let destination: Location = [Parachain(1000)].into();
		let message = Xcm::builder_unpaid()
			.unpaid_execution(Unlimited, None)
			.withdraw_asset((Parent, total_amount))
			.deposit_asset(All, AccountId32 { id: ALICE.into(), network: Some(Kusama) })
			.build();
		// The message can be sent.
		assert_ok!(relay_chain::XcmPallet::send_xcm(Here, destination, message,));
	});

	AssetHub::execute_with(|| {
		// This time it does get executed.
		assert_eq!(asset_hub::Balances::balance(&ALICE), INITIAL_BALANCE + total_amount);
	});
}
