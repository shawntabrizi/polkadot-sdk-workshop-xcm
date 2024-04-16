use xcm_builder::{SignedAccountId32AsNative, SovereignSignedViaLocation};

use pallet_xcm::XcmPassthrough;

pub type XcmOriginToCallOrigin<AccountId, RuntimeOrigin> = (
    SovereignSignedViaLocation<super::locations::LocationToAccountId<AccountId>, RuntimeOrigin>,
    SignedAccountId32AsNative<super::locations::RelayNetwork, RuntimeOrigin>,
    XcmPassthrough<RuntimeOrigin>,
);
