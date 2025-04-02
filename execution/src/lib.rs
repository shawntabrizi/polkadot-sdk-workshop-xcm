//! Exercises for crafting and executing XCMs.
#![allow(unused_variables)]

#[cfg(test)]
mod tests {
	//! This prelude will bring in these three emulated chains:
	//! - Westend, AssetHubWestend, CustomPara
	//!
	//! They all come with accounts called [..]Sender and [..]Receiver,
	//! i.e AssetHubWestendSender.
	//! Calling `::get()` will get the account.
	//!
	//! To get access to their pallets you need to do [..]Pallet and then access
	//! them as associated types.
	use codec::Encode;
	use emulator::{
		parachain_runtime::{CENTS as PARA_CENTS, UNITS as PARA_UNITS},
		prelude::*,
		westend_runtime_constants::currency::{CENTS as WND_CENTS, UNITS as WND_UNITS},
	};
	use frame_support::{
		assert_ok,
		traits::tokens::{fungible, fungibles},
		weights::Weight,
	};
	use xcm::{latest::AssetTransferFilter, prelude::*};

	// Scenario:
	// A sender on our `CustomPara` wants to send 1 unit of its native parachain token to
	// the asset hub.
	#[test]
	fn cross_chain_transfer() {
		// We setup the initial balances of the sender on `CustomPara`.
		let initial_wnd_balance = 10 * WND_UNITS;
		let initial_para_balance = 10 * PARA_UNITS;
		let (sender, receiver) = setup(initial_wnd_balance, initial_para_balance);
		// The amount that wants to be cross-chain transferred to `AssetHubWestend`.
		let transfer_amount = 1 * PARA_UNITS;

		// Parameters of the `WithdrawAsset` instruction.
		// These assets will be taken from the sender account and put into
		// the holding register.
		let assets_to_withdraw: Assets = vec![
			(Here, transfer_amount).into(),  // The assets we want to transfer.
			(Parent, 10 * WND_CENTS).into(), // We use for remote fees.
		].into();

		// Parameters of the `PayFees` instruction.
		// These assets will be taken from the holding register, local execution
		// will be paid for with them and the rest will go to the fees register.
		let fees_amount = 10 * PARA_CENTS;
		let fees_assets: Asset = (Here, fees_amount).into();

		// Parameters of the `InitiateTransfer` instruction.
		// The location of the asset hub.
		let destination = Location::new(1, [Parachain(1000)]);
		// We'll pay fees on the asset hub with its native token: WND.
		// In order to transfer it, we use a reserve asset transfer, since
		// the asset hub is a reserve for it.
		let remote_fees =
			AssetTransferFilter::ReserveWithdraw(Definite((Parent, 10 * WND_CENTS).into()));
		// We don't need to preserve the origin for doing just a regular transfer.
		let preserve_origin = false;
		// We want to transfer all the assets we withdrew.
		// This transfer is a teleport since assets registered on asset hub can be
		// teleported back and forth.
		// Remember, we trust asset hub since it's part of the system, and we already
		// trust the system.
		let transfer_assets = vec![AssetTransferFilter::Teleport(Wild(AllCounted(1)))];
		// The XCM meant to execute on the destination, so on the asset hub.
		let remote_xcm =
			Xcm::<()>::builder_unsafe().deposit_asset(AllCounted(1), receiver.clone()).build();

		// We assemble everything into the XCM we'll execute locally.
		let xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder_unsafe()
			// TODO: Add instructions.
			.build();

		// This lets us execute calls on `CustomPara`.
		// It's the main feature provided by the XCM emulator.
		CustomPara::execute_with(|| {
			// We can use our runtime's pallets like this.
			type CustomBalances = <CustomPara as CustomParaPallet>::Balances;
			// We execute it via the use of the xcm pallet.
			assert_ok!(<CustomPara as CustomParaPallet>::PolkadotXcm::execute(
				<CustomPara as Chain>::RuntimeOrigin::signed(sender.clone()),
				Box::new(VersionedXcm::from(xcm)),
				Weight::MAX,
			));

			// We check that `transfer_amount` actually left the sender's account.
			assert_eq!(
				<CustomBalances as fungible::Inspect<_>>::balance(&sender),
				initial_para_balance - transfer_amount
			);
		});

		// We look at the other chain...
		AssetHubWestend::execute_with(|| {
			type ForeignAssets = <AssetHubWestend as AssetHubWestendPallet>::ForeignAssets;
			let balance = <ForeignAssets as fungibles::Inspect<_>>::balance(
				Location::new(1, [Parachain(CustomPara::para_id().into())]),
				&receiver,
			);
			// ...to check that the receiver got `transfer_amount - fees_amount`.
			assert_eq!(balance, transfer_amount - fees_amount);
		});
	}

	// Now that we know how to make a cross-chain transfer. We'll see that we can nest them.
	// This is a toy example but it shows how you can nest XCMs and how you can manipulate the
	// builder to craft these XCMs.
	#[test]
	fn transfer_n_times() {
		// This is the `n`. Has to be odd so we deposit in the destination.
		// You can try and see what happens when you change it.
		let number_of_hops = 3;

		// Initial setup.
		let initial_wnd_balance = 10 * WND_UNITS;
		let initial_para_balance = 10 * PARA_UNITS;
		let (sender, receiver) = setup(initial_wnd_balance, initial_para_balance);
		let transfer_amount = 1 * PARA_UNITS;

		// `WithdrawAsset` parameters.
		let assets_to_withdraw: Assets = (Here, transfer_amount).into();

		// `PayFees` parameters.
		let fees_amount = 10 * PARA_CENTS;
		let fees_assets: Asset = (Here, fees_amount).into();

		// `InitiateTransfer` parameters.
		// Where we want to go.
		// Where we want to return.
		let destination = Location::new(1, [Parachain(1000)]);
		let return_destination = Location::new(1, [Parachain(2000)]);
		// We'll be using this toggle in our loop to know whether we are going
		// to the asset hub or returning to our custom parachain.
		let mut is_returning = false;
		// This time we use `PARA` for the remote fees, instead of `WND`.
		// This is because the asset hub supports paying fees in any asset you can
		// exchange for `WND`.
		// We've setup a pool between `PARA` and `WND` and added liquidity to it in the `setup`
		// helper function.
		// You can refer to it for more details.
		let remote_fees_amount = 20 * PARA_CENTS;
		let remote_fees =
			AssetTransferFilter::Teleport(Definite((Here, remote_fees_amount).into()));
		// We'll also use `PARA` for the returning fees.
		let remote_fees_returning_amount = 10 * PARA_CENTS;
		let remote_fees_returning = AssetTransferFilter::Teleport(Definite(
			((Parent, Parachain(2000)), remote_fees_returning_amount).into(),
		));
		// No need to preserve the origin.
		let preserve_origin = false;
		let transfer_assets = vec![AssetTransferFilter::Teleport(Wild(AllCounted(1)))];

		// We recursively go forwards and backwards.
		// This is our base case.
		let mut xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder_unsafe()
			.deposit_asset(AllCounted(1), receiver.clone());
		// Then we loop and assemble our XCM.
		for _ in 0..number_of_hops {
			xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder_unsafe().initiate_transfer(
				if is_returning { return_destination.clone() } else { destination.clone() },
				if is_returning { remote_fees_returning.clone() } else { remote_fees.clone() },
				preserve_origin,
				transfer_assets.clone(),
				xcm.build().clone().into(),
			);
			is_returning = !is_returning;
		}
		// We finally build it.
		let recursive_xcm = xcm.build();
		let mut xcm = Xcm::builder_unsafe()
			.withdraw_asset(assets_to_withdraw)
			.pay_fees(fees_assets)
			.build();
		xcm.inner_mut().extend(recursive_xcm.into_iter());

		// We check that the `transfer_amount` was transferred out of the sender's
		// account.
		CustomPara::execute_with(|| {
			assert_ok!(<CustomPara as CustomParaPallet>::PolkadotXcm::execute(
				<CustomPara as Chain>::RuntimeOrigin::signed(sender.clone()),
				Box::new(VersionedXcm::from(xcm)),
				Weight::MAX,
			));

			type Balances = <CustomPara as CustomParaPallet>::Balances;
			assert_eq!(
				<Balances as fungible::Inspect<_>>::balance(&sender),
				initial_para_balance - transfer_amount
			);
		});

		// We need to do this to let the message in `AssetHubWestend` be processed
		// and the new one forwarded back to `CustomPara`.
		AssetHubWestend::execute_with(|| {});
		// We poke the process for every message.
		CustomPara::execute_with(|| {});
		AssetHubWestend::execute_with(|| {});
		CustomPara::execute_with(|| {});
		// Until we reach the final destination.
		// Here, we check that the account on asset hub receives the funds minus fees from
		// all the hops.
		AssetHubWestend::execute_with(|| {
			type ForeignAssets = <AssetHubWestend as AssetHubWestendPallet>::ForeignAssets;
			let balance = <ForeignAssets as fungibles::Inspect<_>>::balance(
				Location::new(1, [Parachain(CustomPara::para_id().into())]),
				&receiver,
			);
			assert_eq!(
				balance,
				transfer_amount -
					fees_amount - 2 * remote_fees_amount -
					remote_fees_returning_amount
			);
		});
	}

	// Scenario:
	// An account on `CustomPara` wants to execute a call on `AssetHubWestend`.
	// For this use case, we have the `Transact` instruction.
	// It's an escape hatch that lets you do anything you need to do with XCM,
	// since you drop down into the FRAME subsystem.
	#[test]
	fn transfer_and_transact() {
		let initial_wnd_balance = 10 * WND_UNITS;
		let initial_para_balance = 10 * PARA_UNITS;
		let (sender, receiver) = setup(initial_wnd_balance, initial_para_balance);
		let transfer_amount = 1 * PARA_UNITS;

		// `WithdrawAsset` parameters.
		let assets_to_withdraw: Assets =
			vec![(Here, transfer_amount).into(), (Parent, 10 * WND_UNITS).into()].into();

		// `PayFees` parameters.
		let fees_amount = 10 * PARA_CENTS;
		let fees_assets: Asset = (Here, fees_amount).into();

		// `Transact` parameters (remember this is on AssetHubWestend!).
		// How to convert the location into a FRAME origin.
		// In this case, we want a two step process where we:
		// - Convert the location into an account (the sovereign account)
		// - Convert that account into a FRAME Signed origin.
		let origin_kind = OriginKind::SovereignAccount;
		// An optional value we only need when we want backwards compatibility
		// with versions older than 5.
		let fallback_max_weight = None;
		// We want to execute the `remark_with_event` call on the asset hub.
		let remark = b"Hello, world!".to_vec();
		let remark_hash = sp_io::hashing::blake2_256(&remark);
		let call = <AssetHubWestend as Chain>::RuntimeCall::System(frame_system::Call::<
			<AssetHubWestend as Chain>::Runtime,
		>::remark_with_event {
			remark,
		})
		.encode();

		// `InitiateTransfer` parameters.
		let destination = Location::new(1, [Parachain(1000)]);
		let remote_fees =
			AssetTransferFilter::ReserveWithdraw(Definite((Parent, 10 * WND_CENTS).into()));
		// This time we NEED to preserve the origin.
		// If not, the Transact won't know how to get a FRAME origin
		// to execute the call.
		let preserve_origin = true;
		let transfer_assets = vec![AssetTransferFilter::Teleport(Wild(AllCounted(1)))];
		let remote_xcm = Xcm::<()>::builder_unsafe()
			.transact(origin_kind, fallback_max_weight, call)
			.deposit_asset(AllCounted(1), receiver.clone())
			.build();

		// We assemble the XCM with all the previous values.
		let xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder_unsafe()
			// TODO: Add instructions.
			.build();

		// We execute the XCM and assert that the `transfer_amount` is taken
		// out of the senders account.
		CustomPara::execute_with(|| {
			assert_ok!(<CustomPara as CustomParaPallet>::PolkadotXcm::execute(
				<CustomPara as Chain>::RuntimeOrigin::signed(sender.clone()),
				Box::new(VersionedXcm::from(xcm)),
				Weight::MAX,
			));

			type Balances = <CustomPara as CustomParaPallet>::Balances;
			assert_eq!(
				<Balances as fungible::Inspect<_>>::balance(&sender),
				initial_para_balance - transfer_amount
			);
		});

		// We check that the event from the transaction we made is actually emitted.
		AssetHubWestend::execute_with(|| {
			let sov_account_of_sender_on_custom_para =
				AssetHubWestend::sovereign_account_id_of(Location::new(
					1,
					[
						Parachain(2000),
						AccountId32 { network: None, id: CustomParaSender::get().into() },
					],
				));
			<AssetHubWestend as AssetHubWestendPallet>::System::assert_has_event(
				frame_system::Event::Remarked {
					sender: sov_account_of_sender_on_custom_para,
					hash: remark_hash.into(),
				}
				.into(),
			);
		});
	}

	// Scenario:
	// An account on our custom parachain wants to exchange some assets but
	// can't do that locally.
	// We can use the `ExchangeAsset` instruction to swap between two assets.
	// Keep in mind this only works if the underlying chain implements this operation.
	#[test]
	fn transfer_and_swap() {
		let initial_wnd_balance = 10 * WND_UNITS;
		let initial_para_balance = 100 * PARA_UNITS;
		let (sender, _) = setup(initial_wnd_balance, initial_para_balance);
		let transfer_amount = 23 * PARA_UNITS;

		let assets_to_withdraw: Assets = (Here, transfer_amount).into();

		let fees_amount = 10 * PARA_CENTS;
		let fees_asset: Asset = (Here, fees_amount).into();

		let destination: Location = (Parent, Parachain(1000)).into();
		let remote_fees =
			Some(AssetTransferFilter::Teleport(Definite((Here, 20 * PARA_CENTS).into())));
		let preserve_origin = false;
		let assets_to_transfer = vec![AssetTransferFilter::Teleport(Wild(AllCounted(1)))];
		let remote_xcm = Xcm::<()>::builder_unsafe()
			.exchange_asset(
				Wild(AllCounted(1)),
				(Parent, 10 * WND_UNITS),
				true, // Maximal.
			)
			.deposit_asset(AllCounted(1), sender.clone())
			.build();

		let xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder_unsafe()
			// TODO: Add instructions.
			.build();

		// We get the initial WND amount so we can compare it later.
		let initial_wnd_on_ah = AssetHubWestend::execute_with(|| {
			type Balances = <CustomPara as CustomParaPallet>::Balances;
			let balance = <Balances as fungible::Inspect<_>>::balance(&sender);
			balance
		});
		CustomPara::execute_with(|| {
			assert_ok!(<CustomPara as CustomParaPallet>::PolkadotXcm::execute(
				<CustomPara as Chain>::RuntimeOrigin::signed(sender.clone()),
				Box::new(VersionedXcm::from(xcm)),
				Weight::MAX,
			));
		});
		AssetHubWestend::execute_with(|| {
			// We check if we have more `WND`.
			type Balances = <CustomPara as CustomParaPallet>::Balances;
			let balance = <Balances as fungible::Inspect<_>>::balance(&sender);
			assert!(balance > initial_wnd_on_ah);
		});
	}

	// Scenario:
	// Now, the account on `CustomPara` wants to send some assets to the asset hub
	// to swap them, but then wants to get them back **in the same message**.
	#[test]
	fn transfer_swap_and_back() {
		let initial_wnd_balance = 10 * WND_UNITS;
		let initial_para_balance = 100 * PARA_UNITS;
		let (sender, _) = setup(initial_wnd_balance, initial_para_balance);
		let transfer_amount = 23 * PARA_UNITS;
		let fees_amount = 10 * PARA_CENTS;
		let xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder_unsafe()
			// TODO: Add instructions.
			.build();

		CustomPara::execute_with(|| {
			assert_ok!(<CustomPara as CustomParaPallet>::PolkadotXcm::execute(
				<CustomPara as Chain>::RuntimeOrigin::signed(sender.clone()),
				Box::new(VersionedXcm::from(xcm)),
				Weight::MAX,
			));
		});
		// We let the message in `AssetHubWestend` process.
		AssetHubWestend::execute_with(|| {});
		CustomPara::execute_with(|| {
			// We check if we got the WND back.
			type ForeignAssets = <CustomPara as CustomParaPallet>::ForeignAssets;
			let balance =
				<ForeignAssets as fungibles::Inspect<_>>::balance(Location::parent(), &sender);
			assert!(balance > initial_wnd_balance);
		});
	}

	// A helper function for setting up initial balances and liquidity pools.
	fn setup(initial_wnd_balance: u128, initial_para_balance: u128) -> (AccountId, AccountId) {
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
}
