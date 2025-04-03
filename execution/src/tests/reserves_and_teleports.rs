//! Tests for configuring IsReserve and IsTeleporter.

use emulator::prelude::*;
use frame_support::{assert_ok, traits::{fungible, fungibles}};
use xcm::{prelude::*, latest::AssetTransferFilter};

#[test]
fn teleport_of_para_from_asset_hub_works() {
    let initial_para_balance = 20 * PARA_UNITS;
    let initial_wnd_balance = 10 * WND_UNITS;
    let transfer_amount = 10 * PARA_UNITS;
    let local_fees_amount = 10 * WND_CENTS;
    let remote_fees_amount = 10 * PARA_CENTS;
    let para_location = Location::new(1, [Parachain(CustomPara::para_id().into())]);
    let assets_to_withdraw: Assets = vec![
        (para_location.clone(), transfer_amount).into(),
        (Parent, local_fees_amount).into()
    ].into();
    let sender = AssetHubWestendSender::get();
    let receiver = CustomParaReceiver::get();
    let xcm = Xcm::<<AssetHubWestend as Chain>::RuntimeCall>::builder()
        .withdraw_asset(assets_to_withdraw)
        .pay_fees((Parent, local_fees_amount))
        .initiate_transfer(
            para_location.clone(),
            AssetTransferFilter::Teleport(Definite((para_location.clone(), remote_fees_amount).into())),
            false,
            vec![AssetTransferFilter::Teleport(Wild(AllCounted(1)))],
            Xcm::<()>::builder_unsafe()
                .deposit_asset(AllCounted(1), receiver.clone())
                .build()
        )
        .build();
	AssetHubWestend::force_create_foreign_asset(
		para_location.clone(),
		AssetHubWestendSender::get(),
		false,
		1,
		Vec::new(),
	);
    AssetHubWestend::execute_with(|| {
        type Balances = <AssetHubWestend as AssetHubWestendPallet>::Balances;
        assert_ok!(<Balances as fungible::Mutate<_>>::mint_into(
            &sender,
            initial_wnd_balance
        ));
        type ForeignAssets = <AssetHubWestend as AssetHubWestendPallet>::ForeignAssets;
        assert_ok!(<ForeignAssets as fungibles::Mutate<_>>::mint_into(
            para_location,
            &sender,
            initial_para_balance
        ));
		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::PolkadotXcm::execute(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(sender.clone()),
			Box::new(VersionedXcm::from(xcm)),
			Weight::MAX,
		));
    });

    CustomPara::execute_with(|| {
        type Balances = <CustomPara as CustomParaPallet>::Balances;
        let balance = <Balances as fungible::Inspect<_>>::balance(&receiver);
        assert_eq!(balance, transfer_amount - remote_fees_amount);
    });
}

#[test]
fn reserve_deposit_wnd_from_asset_hub_works() {
    let initial_wnd_balance = 20 * WND_UNITS;
    let transfer_amount = 10 * WND_UNITS;
    let local_fees_amount = 10 * WND_CENTS;
    let remote_fees_amount = 10 * WND_CENTS;
    let para_location = Location::new(1, [Parachain(CustomPara::para_id().into())]);
    let assets_to_withdraw: Assets = vec![
        (Parent, transfer_amount + local_fees_amount).into()
    ].into();
    let sender = AssetHubWestendSender::get();
    let receiver = CustomParaReceiver::get();
    let xcm = Xcm::<<AssetHubWestend as Chain>::RuntimeCall>::builder()
        .withdraw_asset(assets_to_withdraw)
        .pay_fees((Parent, local_fees_amount))
        .initiate_transfer(
            para_location.clone(),
            AssetTransferFilter::ReserveDeposit(Definite((Parent, remote_fees_amount).into())),
            false,
            vec![AssetTransferFilter::ReserveDeposit(Wild(AllCounted(1)))],
            Xcm::<()>::builder_unsafe()
                .deposit_asset(AllCounted(1), receiver.clone())
                .build()
        )
        .build();
    // We need to register `WND` on the parachain.
	CustomPara::force_create_foreign_asset(
		Location::parent(),
		CustomParaSender::get(),
		false,
		1,
		Vec::new(),
	);
	// To cover ED on receiver.
	CustomPara::execute_with(|| {
		type Balances = <CustomPara as CustomParaPallet>::Balances;
		assert_ok!(
			<Balances as fungible::Mutate<_>>::mint_into(&receiver, 1 * PARA_UNITS)
		);
	});
    AssetHubWestend::execute_with(|| {
        type Balances = <AssetHubWestend as AssetHubWestendPallet>::Balances;
        assert_ok!(<Balances as fungible::Mutate<_>>::mint_into(
            &sender,
            initial_wnd_balance
        ));
		assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::PolkadotXcm::execute(
			<AssetHubWestend as Chain>::RuntimeOrigin::signed(sender.clone()),
			Box::new(VersionedXcm::from(xcm)),
			Weight::MAX,
		));
    });

    CustomPara::execute_with(|| {
        type ForeignAssets = <CustomPara as CustomParaPallet>::ForeignAssets;
        let balance = <ForeignAssets as fungibles::Inspect<_>>::balance(Parent.into(), &receiver);
        assert_eq!(balance, transfer_amount - remote_fees_amount);
    });
}
