use xcm_builder::{SignedAccountId32AsNative, SovereignSignedViaLocation};

use super::locations::{LocationToAccountId, RelayNetwork};
use pallet_xcm::XcmPassthrough;

pub type XcmOriginToCallOrigin<AccountId, RuntimeOrigin> = (
    SovereignSignedViaLocation<LocationToAccountId<AccountId>, RuntimeOrigin>,
    SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
    XcmPassthrough<RuntimeOrigin>,
);
