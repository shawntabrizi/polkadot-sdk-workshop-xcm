pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
	pub type OriginConverter = ();
}

#[cfg(any(feature = "example", feature = "relay-token"))]
mod sandbox {
	use crate::parachain::{
		constants::RelayNetwork, location_converter::LocationConverter, RuntimeOrigin,
	};
	use pallet_xcm::XcmPassthrough;
	use xcm_builder::{SignedAccountId32AsNative, SovereignSignedViaLocation};

	type XcmOriginToCallOrigin = (
		SovereignSignedViaLocation<LocationConverter, RuntimeOrigin>,
		SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
		XcmPassthrough<RuntimeOrigin>,
	);

	pub type OriginConverter = XcmOriginToCallOrigin;
}
