use emulator::prelude::*;
use frame_support::{assert_ok, traits::fungible};
use xcm::prelude::*;

/// A helper function for setting up initial balances and liquidity pools.
pub fn setup(initial_wnd_balance: u128, initial_para_balance: u128) -> (AccountId, AccountId) {
	// We are going to be making transfers between `CustomPara` and `AssetHubWestend`,
	// of both `PARA` and `WND`, the native tokens of each chain respectively.
	// In order to do this with `PARA`, we need to register the token in the asset hub.
	// We do this using its xcm location.
	let custom_para_from_ah = AssetHubWestend::sibling_location_of(CustomPara::para_id());
	AssetHubWestend::force_create_foreign_asset(
		custom_para_from_ah.clone(),
		AssetHubWestendSender::get(),
		false,
		1,
		Vec::new(),
	);
	// We then need to fund its sovereign account, so `WND` can be reserve transferred
	// into the parachain.
	let sov_account_custom_para_on_ah =
		AssetHubWestend::sovereign_account_id_of(custom_para_from_ah.clone());
	AssetHubWestend::fund_accounts(vec![
		(sov_account_custom_para_on_ah, initial_wnd_balance),
		// This is balance for later adding liquidity to the pools.
		(AssetHubWestendSender::get(), 1000 * WND_UNITS),
	]);
	let sender = CustomParaSender::get();
	let receiver = AssetHubWestendReceiver::get();
	// We also register `WND` on the parachain.
	CustomPara::force_create_foreign_asset(
		Location::parent(),
		CustomParaSender::get(),
		false,
		1,
		Vec::new(),
	);

	// We mint the initial `PARA` balance passed in to the sender.
	CustomPara::execute_with(|| {
		type Balances = <CustomPara as CustomParaPallet>::Balances;
		assert_ok!(
			<Balances as fungible::Mutate<_>>::mint_into(&sender, initial_para_balance,)
		);
	});
	// We mint the initial `WND` balance passed in to the sender.
	CustomPara::mint_foreign_asset(
		<CustomPara as Chain>::RuntimeOrigin::signed(sender.clone()),
		Location::parent(),
		sender.clone(),
		initial_wnd_balance,
	);
	// We mint some initial `PARA` balance to an asset hub account.
	AssetHubWestend::mint_foreign_asset(
		<AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
		custom_para_from_ah.clone(),
		AssetHubWestendSender::get(),
		201 * PARA_UNITS,
	);
	// We create the pool between `WND` and `PARA` and add liquidity to it.
	AssetHubWestend::execute_with(|| {
		type AssetConversion = <AssetHubWestend as AssetHubWestendPallet>::AssetConversion;
		type RuntimeOrigin = <AssetHubWestend as Chain>::RuntimeOrigin;
		assert_ok!(AssetConversion::create_pool(
			RuntimeOrigin::signed(AssetHubWestendSender::get()),
			Box::new(Location::parent()),
			Box::new(custom_para_from_ah.clone())
		));
		assert_ok!(AssetConversion::add_liquidity(
			RuntimeOrigin::signed(AssetHubWestendSender::get()),
			Box::new(Location::parent()),
			Box::new(custom_para_from_ah.clone()),
			100 * WND_UNITS,
			200 * PARA_UNITS, // Custom para asset is worth half of WND.
			0,
			0,
			AssetHubWestendSender::get()
		));
	});

	(sender, receiver)
}
