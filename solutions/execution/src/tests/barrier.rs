//! Tests for configuring the barrier.

use codec::Encode;
use emulator::prelude::*;
use frame_support::assert_ok;
use xcm::prelude::*;

#[test]
fn require_fee_payment() {
    // We send a message to our custom parachain which doesn't intend
    // to pay for fees.
    AssetHubWestend::execute_with(|| {
        let destination = Location::new(1, [Parachain(CustomPara::para_id().into())]);
        let call = <CustomPara as Chain>::RuntimeCall::System(frame_system::Call::<
            <CustomPara as Chain>::Runtime,
        >::remark_with_event {
            remark: b"Is there anyone there? :(".to_vec(),
        }).encode();
        let message = Xcm::<()>::builder_unsafe()
            .transact(
                OriginKind::SovereignAccount,
                None,
                call
            )
            .build();
	    assert_ok!(<AssetHubWestend as AssetHubWestendPallet>::PolkadotXcm::send(
	        <AssetHubWestend as Chain>::RuntimeOrigin::signed(AssetHubWestendSender::get()),
            Box::new(VersionedLocation::from(destination)),
	        Box::new(VersionedXcm::from(message)),
	    ));
    });

    // We check that the message should be blocked by the barrier.
    CustomPara::execute_with(|| {
        type RuntimeEvent = <CustomPara as Chain>::RuntimeEvent;
        assert_expected_events!(
            CustomPara,
            vec![
                RuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed {
                    success: false,
                    ..
                }) => {},
            ]
        );
    });
}
