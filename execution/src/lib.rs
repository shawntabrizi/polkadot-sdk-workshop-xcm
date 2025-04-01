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
    use frame_support::{assert_ok, traits::tokens::{fungible, fungibles}, weights::Weight};
    use xcm::{prelude::*, latest::AssetTransferFilter};

    #[test]
    fn cross_chain_transfer() {
        let initial_wnd_balance = 10 * WND_UNITS;
        let initial_para_balance = 10 * PARA_UNITS;
        let (sender, receiver) = setup(initial_wnd_balance, initial_para_balance);
        let transfer_amount = 1 * PARA_UNITS;
        let fees_amount = 10 * PARA_CENTS;
        CustomPara::execute_with(|| {
            type CustomBalances = <CustomPara as CustomParaPallet>::Balances;
            let xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder()
                .withdraw_asset(vec![
                    (Here, transfer_amount).into(),
                    (Parent, 10 * WND_UNITS).into()
                ])
                .pay_fees((Here, fees_amount))
                .initiate_transfer(
                    (Parent, Parachain(1000)),
                    AssetTransferFilter::ReserveWithdraw(Definite(
                        (Parent, 10 * WND_CENTS).into())
                    ),
                    false,
                    vec![AssetTransferFilter::Teleport(Wild(AllCounted(1)))],
                    Xcm::builder_unsafe()
                        .deposit_asset(AllCounted(1), receiver.clone())
                        .build()
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
    fn transfer_and_transact() {
        // TODO
    }

    #[test]
    fn transfer_swap_and_back() {
        let initial_wnd_balance = 10 * WND_UNITS;
        let initial_para_balance = 100 * PARA_UNITS;
        let (sender, receiver) = setup(initial_wnd_balance, initial_para_balance);
        let transfer_amount = 23 * PARA_UNITS;
        let fees_amount = 10 * PARA_CENTS;
        CustomPara::execute_with(|| {
            type CustomBalances = <CustomPara as CustomParaPallet>::Balances;
            let xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder()
                .withdraw_asset(vec![
                    (Here, transfer_amount).into(),
                ])
                .pay_fees((Here, fees_amount))
                .initiate_transfer(
                    (Parent, Parachain(1000)),
                    AssetTransferFilter::Teleport(Definite(
                        (Here, 10 * PARA_CENTS).into()
                    )),
                    false,
                    vec![AssetTransferFilter::Teleport(Wild(AllCounted(1)))],
                    Xcm::builder_unsafe()
                        .exchange_asset(
                            Wild(AllCounted(1)),
                            (Parent, 10 * WND_UNITS),
                            true
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
    }

    #[test]
    fn transfer_n_times() {
        // TODO
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

