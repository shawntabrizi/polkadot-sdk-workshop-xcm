//! Exercises for crafting and executing XCMs.

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
    use emulator::{
        prelude::*,
        parachain_runtime::{UNITS as PARA_UNITS, CENTS as PARA_CENTS},
        westend_runtime_constants::currency::{UNITS as WND_UNITS, CENTS as WND_CENTS},
    };
    use codec::Encode;
    use frame_support::{assert_ok, traits::tokens::{fungible, fungibles}, weights::Weight};
    use xcm::{prelude::*, latest::AssetTransferFilter};

    #[test]
    fn cross_chain_transfer() {
        let initial_wnd_balance = 10 * WND_UNITS;
        let initial_para_balance = 10 * PARA_UNITS;
        let (sender, receiver) = setup(initial_wnd_balance, initial_para_balance);
        let transfer_amount = 1 * PARA_UNITS;

        // Withdraw asset parameters.
        let assets_to_withdraw = vec![
            (Here, transfer_amount).into(),
            (Parent, 10 * WND_UNITS).into()
        ];

        // Pay asset parameters.
        let fees_amount = 10 * PARA_CENTS;
        let fees_assets: Asset = (Here, fees_amount).into();

        // Transfer parameters.
        let destination = Location::new(1, [Parachain(1000)]);
        let remote_fees = AssetTransferFilter::ReserveWithdraw(Definite(
            (Parent, 10 * WND_CENTS).into())
        );
        let preserve_origin = false;
        let transfer_assets = vec![AssetTransferFilter::Teleport(Wild(AllCounted(1)))];
        let remote_xcm = Xcm::builder_unsafe()
            .deposit_asset(AllCounted(1), receiver.clone())
            .build();

        CustomPara::execute_with(|| {
            type CustomBalances = <CustomPara as CustomParaPallet>::Balances;
            let xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder()
                .withdraw_asset(assets_to_withdraw)
                .pay_fees(fees_assets)
                .initiate_transfer(
                    destination,
                    remote_fees,
                    preserve_origin,
                    transfer_assets,
                    remote_xcm
                )
                .build();
            assert_ok!(<CustomPara as CustomParaPallet>::PolkadotXcm::execute(
                <CustomPara as Chain>::RuntimeOrigin::signed(sender.clone()),
                Box::new(VersionedXcm::from(xcm)),
                Weight::MAX,
            ));

            assert_eq!(
                <CustomBalances as fungible::Inspect<_>>::balance(&sender),
                initial_para_balance - transfer_amount
            );
        });

        AssetHubWestend::execute_with(|| {
            type ForeignAssets = <AssetHubWestend as AssetHubWestendPallet>::ForeignAssets;
            let balance = <ForeignAssets as fungibles::Inspect<_>>::balance(
                Location::new(1, [Parachain(CustomPara::para_id().into())]),
                &receiver
            );
            assert_eq!(balance, transfer_amount - fees_amount);
        });
    }


    #[test]
    fn transfer_n_times() {
        // This is the `n`. Has to be odd so we deposit in the destination.
        let number_of_hops = 3;

        // Initial balances.
        let initial_wnd_balance = 10 * WND_UNITS;
        let initial_para_balance = 10 * PARA_UNITS;
        let (sender, receiver) = setup(initial_wnd_balance, initial_para_balance);
        let transfer_amount = 1 * PARA_UNITS;

        // `WithdrawAsset` parameters.
        let assets_to_withdraw: Assets = (Here, transfer_amount).into();

        // `PayFees` parameters.
        let fees_amount = 10 * PARA_CENTS;
        let fees_assets: Asset = (Here, fees_amount).into();

        // Transfer parameters.
        let destination = Location::new(1, [Parachain(1000)]); // Where we want to go.
        let return_destination = Location::new(1, [Parachain(2000)]); // Where we want to return.
        // Whether or not we are returning or going.
        let mut is_returning = false;
        let remote_fees_amount = 20 * PARA_CENTS;
        let remote_fees = AssetTransferFilter::Teleport(Definite(
            (Here, remote_fees_amount).into())
        );
        let remote_fees_returning_amount = 10 * PARA_CENTS;
        let remote_fees_returning = AssetTransferFilter::Teleport(Definite(
            ((Parent, Parachain(2000)), remote_fees_returning_amount).into()
        ));
        let preserve_origin = false;
        let transfer_assets = vec![AssetTransferFilter::Teleport(Wild(AllCounted(1)))];

        // We recursively go forwards and backwards.
        // This is our base case.
        let mut xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder_unsafe()
            .deposit_asset(AllCounted(1), receiver.clone());
        for _ in 0..number_of_hops {
            xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder_unsafe()
                .initiate_transfer(
                    if is_returning { return_destination.clone() } else { destination.clone() },
                    if is_returning { remote_fees_returning.clone() } else { remote_fees.clone() },
                    preserve_origin,
                    transfer_assets.clone(),
                    xcm.build().clone().into()
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

        CustomPara::execute_with(|| {
            type CustomBalances = <CustomPara as CustomParaPallet>::Balances;
            assert_ok!(<CustomPara as CustomParaPallet>::PolkadotXcm::execute(
                <CustomPara as Chain>::RuntimeOrigin::signed(sender.clone()),
                Box::new(VersionedXcm::from(xcm)),
                Weight::MAX,
            ));

            assert_eq!(
                <CustomBalances as fungible::Inspect<_>>::balance(&sender),
                initial_para_balance - transfer_amount
            );
        });

        AssetHubWestend::execute_with(|| {});
        CustomPara::execute_with(|| {});
        AssetHubWestend::execute_with(|| {});
        CustomPara::execute_with(|| {});
        AssetHubWestend::execute_with(|| {
            type ForeignAssets = <AssetHubWestend as AssetHubWestendPallet>::ForeignAssets;
            let balance = <ForeignAssets as fungibles::Inspect<_>>::balance(
                Location::new(1, [Parachain(CustomPara::para_id().into())]),
                &receiver
            );
            assert_eq!(
                balance,
                transfer_amount - fees_amount - 2 * remote_fees_amount - remote_fees_returning_amount
            );
        });
    }

    #[test]
    fn transfer_and_transact() {
        let initial_wnd_balance = 10 * WND_UNITS;
        let initial_para_balance = 10 * PARA_UNITS;
        let (sender, receiver) = setup(initial_wnd_balance, initial_para_balance);
        let transfer_amount = 1 * PARA_UNITS;

        // Withdraw asset parameters.
        let assets_to_withdraw = vec![
            (Here, transfer_amount).into(),
            (Parent, 10 * WND_UNITS).into()
        ];

        // Pay asset parameters.
        let fees_amount = 10 * PARA_CENTS;
        let fees_assets: Asset = (Here, fees_amount).into();

        // Transact parameters (remember this is on AssetHubWestend!).
        let origin_kind = OriginKind::SovereignAccount;
        let fallback_max_weight = None;
        let remark = b"Hello, world!".to_vec();
        let remark_hash = sp_io::hashing::blake2_256(&remark);
        let call = <AssetHubWestend as Chain>::RuntimeCall::System(
            frame_system::Call::<<AssetHubWestend as Chain>::Runtime>::remark_with_event {
                remark,
            },
        ).encode();

        // Transfer parameters.
        let destination = Location::new(1, [Parachain(1000)]);
        let remote_fees = AssetTransferFilter::ReserveWithdraw(Definite(
            (Parent, 10 * WND_CENTS).into())
        );
        let preserve_origin = true;
        let transfer_assets = vec![AssetTransferFilter::Teleport(Wild(AllCounted(1)))];
        let remote_xcm = Xcm::builder_unsafe()
            .transact(origin_kind, fallback_max_weight, call)
            .deposit_asset(AllCounted(1), receiver.clone())
            .build();

        CustomPara::execute_with(|| {
            type CustomBalances = <CustomPara as CustomParaPallet>::Balances;
            let xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder()
                .withdraw_asset(assets_to_withdraw)
                .pay_fees(fees_assets)
                .initiate_transfer(
                    destination,
                    remote_fees,
                    preserve_origin,
                    transfer_assets,
                    remote_xcm
                )
                .build();
            assert_ok!(<CustomPara as CustomParaPallet>::PolkadotXcm::execute(
                <CustomPara as Chain>::RuntimeOrigin::signed(sender.clone()),
                Box::new(VersionedXcm::from(xcm)),
                Weight::MAX,
            ));

            assert_eq!(
                <CustomBalances as fungible::Inspect<_>>::balance(&sender),
                initial_para_balance - transfer_amount
            );
        });

        // We check that the event from the transaction we made is actually emitted.
        AssetHubWestend::execute_with(|| {
            let sov_account_of_sender_on_custom_para = AssetHubWestend::sovereign_account_id_of(
                Location::new(1, [Parachain(2000), AccountId32 { network: None, id: CustomParaSender::get().into() }])
            );
            <AssetHubWestend as AssetHubWestendPallet>::System::assert_has_event(
                frame_system::Event::Remarked { sender: sov_account_of_sender_on_custom_para, hash: remark_hash.into() }.into()
            );
        });
    }

    #[test]
    fn transfer_swap_and_back() {
        let initial_wnd_balance = 10 * WND_UNITS;
        let initial_para_balance = 100 * PARA_UNITS;
        let (sender, _) = setup(initial_wnd_balance, initial_para_balance);
        let transfer_amount = 23 * PARA_UNITS;
        let fees_amount = 10 * PARA_CENTS;
        CustomPara::execute_with(|| {
            let xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder()
                .withdraw_asset(vec![
                    (Here, transfer_amount).into(),
                ])
                .pay_fees((Here, fees_amount))
                .initiate_transfer(
                    (Parent, Parachain(1000)),
                    AssetTransferFilter::Teleport(Definite(
                        (Here, 20 * PARA_CENTS).into()
                    )),
                    false,
                    vec![AssetTransferFilter::Teleport(Wild(AllCounted(1)))],
                    Xcm::builder_unsafe()
                        .exchange_asset(
                            Wild(AllCounted(1)),
                            (Parent, 10 * WND_UNITS),
                            true // Maximal.
                        )
                        .initiate_transfer(
                            (Parent, Parachain(2000)),
                            AssetTransferFilter::ReserveDeposit(Definite(
                                (Parent, 50 * WND_CENTS).into()
                            )),
                            false,
                            vec![AssetTransferFilter::ReserveDeposit(Wild(AllCounted(1)))],
                            Xcm::builder_unsafe()
                                .deposit_asset(AllCounted(1), sender.clone())
                                .build()
                        )
                        .build()
                )
                .build();
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
            let balance = <ForeignAssets as fungibles::Inspect<_>>::balance(Location::parent(), &sender);
            assert!(balance > initial_wnd_balance);
        });
    }

    fn setup(
        initial_wnd_balance: u128,
        initial_para_balance: u128,
    ) -> (AccountId, AccountId) {
        let custom_para_from_ah = AssetHubWestend::sibling_location_of(CustomPara::para_id());
        AssetHubWestend::force_create_foreign_asset(
             custom_para_from_ah.clone(),
             AssetHubWestendSender::get(),
             false,
             1,
             Vec::new()
        );
        let sov_account_custom_para_on_ah = AssetHubWestend::sovereign_account_id_of(custom_para_from_ah.clone());
        AssetHubWestend::fund_accounts(vec![
            (sov_account_custom_para_on_ah, initial_wnd_balance),
            (AssetHubWestendSender::get(), 1000 * WND_UNITS)
        ]);
        let sender = CustomParaSender::get();
        let receiver = AssetHubWestendReceiver::get();
        CustomPara::force_create_foreign_asset(
            Location::parent(),
            CustomParaSender::get(),
            false,
            1,
            Vec::new()
        );

        CustomPara::execute_with(|| {
            type CustomBalances = <CustomPara as CustomParaPallet>::Balances;
            assert_ok!(<CustomBalances as fungible::Mutate<_>>::mint_into(
                &sender,
                initial_para_balance,
            ));
        });

        CustomPara::mint_foreign_asset(
            <CustomPara as Chain>::RuntimeOrigin::signed(sender.clone()),
            Location::parent(),
            sender.clone(),
            initial_wnd_balance
        );

        AssetHubWestend::mint_foreign_asset(
            <AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
            custom_para_from_ah.clone(),
            AssetHubWestendSender::get(),
            201 * PARA_UNITS,
        );
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

