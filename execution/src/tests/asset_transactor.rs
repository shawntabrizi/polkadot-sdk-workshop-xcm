use super::common;

use emulator::prelude::*;
use frame_support::assert_ok;
use xcm::prelude::*;

// Here we want to make sure we can handle the native asset.
// To test this, we only try to withdraw it and nothing else.
// We'll need to configure the `AssetTransactor` on `CustomPara`.
#[test]
fn can_handle_native_asset() {
	// We setup the initial balances of the sender on `CustomPara`.
	let initial_wnd_balance = 10 * WND_UNITS;
	let initial_para_balance = 10 * PARA_UNITS;
	let (sender, _) = common::setup(initial_wnd_balance, initial_para_balance);

	// The amount we want to withdraw of the native asset.
	let withdraw_amount = 1 * PARA_UNITS;
	let assets_to_withdraw: Assets = vec![
	    (Here, withdraw_amount).into()
	].into();

	let xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder_unsafe()
	    .withdraw_asset(assets_to_withdraw)
	    .build();

	CustomPara::execute_with(|| {
	    assert_ok!(<CustomPara as CustomParaPallet>::PolkadotXcm::execute(
	        <CustomPara as Chain>::RuntimeOrigin::signed(sender.clone()),
	        Box::new(VersionedXcm::from(xcm)),
            Weight::MAX,
	    ));
	});
}

#[test]
fn can_handle_relay_asset() {
	// We setup the initial balances of the sender on `CustomPara`.
	let initial_wnd_balance = 10 * WND_UNITS;
	let initial_para_balance = 10 * PARA_UNITS;
	let (sender, _) = common::setup(initial_wnd_balance, initial_para_balance);

	// The amount we want to withdraw of the native asset.
	let withdraw_amount = 1 * WND_UNITS;
	let assets_to_withdraw: Assets = vec![
	    (Parent, withdraw_amount).into()
	].into();

	let xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder_unsafe()
	    .withdraw_asset(assets_to_withdraw)
	    .build();

	CustomPara::execute_with(|| {
	    assert_ok!(<CustomPara as CustomParaPallet>::PolkadotXcm::execute(
	        <CustomPara as Chain>::RuntimeOrigin::signed(sender.clone()),
	        Box::new(VersionedXcm::from(xcm)),
            Weight::MAX,
	    ));
	});
}
