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
        let custom_para_from_ah = AssetHubWestend::sibling_location_of(CustomPara::para_id());
        AssetHubWestend::force_create_foreign_asset(
             custom_para_from_ah.clone(),
             AssetHubWestendSender::get(),
             false,
             1,
             Vec::new()
        );
        let sov_account_custom_para_on_ah = AssetHubWestend::sovereign_account_id_of(custom_para_from_ah.clone());
        let initial_balance = 10 * WND_UNITS;
        AssetHubWestend::fund_accounts(vec![(sov_account_custom_para_on_ah, initial_balance)]);
        let sender = CustomParaSender::get();
        let receiver = AssetHubWestendReceiver::get();

        CustomPara::execute_with(|| {
            type CustomBalances = <CustomPara as CustomParaPallet>::Balances;
            let starting_balance = 10 * PARA_UNITS;
            assert_ok!(<CustomBalances as fungible::Mutate<_>>::mint_into(
                &sender,
                starting_balance,
            ));
            let transfer_amount = 1 * PARA_UNITS;
            let fees_amount = 10 * PARA_CENTS;
            let remote_fees: Assets = (Parent, 10 * WND_CENTS).into();
            let xcm = Xcm::<<CustomPara as Chain>::RuntimeCall>::builder()
                .withdraw_asset((Here, transfer_amount))
                .pay_fees((Here, fees_amount))
                .initiate_transfer(
                    (Parent, Parachain(1000)),
                    AssetTransferFilter::ReserveWithdraw(Definite(remote_fees)),
                    false,
                    vec![AssetTransferFilter::ReserveDeposit(Wild(AllCounted(1)))],
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
                starting_balance - transfer_amount
            );
        });

        AssetHubWestend::execute_with(|| {
            type ForeignAssets = <AssetHubWestend as AssetHubWestendPallet>::ForeignAssets;
            let balance = <ForeignAssets as fungibles::Inspect<_>>::balance(custom_para_from_ah, &receiver);
            dbg!(&balance);
        });
    }

    #[test]
    fn transfer_and_transact() {
        // TODO
    }

    #[test]
    fn transfer_swap_and_back() {
        // TODO
    }

    #[test]
    fn transfer_n_times() {
        // TODO
    }
}

