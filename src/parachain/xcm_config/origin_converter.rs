pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
    pub type OriginConverter = ();
}

#[cfg(feature = "example")]
mod sandbox {
    use crate::parachain::locations::{LocationToAccountId, RelayNetwork};
    use crate::parachain::{AccountId, RuntimeOrigin};
    use pallet_xcm::XcmPassthrough;
    use xcm_builder::{SignedAccountId32AsNative, SovereignSignedViaLocation};

    pub type XcmOriginToCallOrigin = (
        SovereignSignedViaLocation<LocationToAccountId<AccountId>, RuntimeOrigin>,
        SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
        XcmPassthrough<RuntimeOrigin>,
    );

    pub type OriginConverter = XcmOriginToCallOrigin;
}
