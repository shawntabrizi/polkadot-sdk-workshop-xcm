pub use sandbox::*;

#[cfg(not(feature = "asset-hub"))]
mod sandbox {
	pub type TrustedTeleporters = ();
}

#[cfg(feature = "asset-hub")]
mod sandbox {
	use frame_support::parameter_types;
	use xcm::latest::prelude::*;

	parameter_types! {
		// Filter for our native token.
		pub NativeToken: AssetFilter = todo!();
		// Location for asset hub.
		pub AssetHubLocation: Location = todo!();
		// A filter, and a location we trust as teleporter for that filter.
		pub AssetHubTrustedTeleporter: (AssetFilter, Location) = todo!();
	}

	pub type TrustedTeleporters = xcm_builder::Case<AssetHubTrustedTeleporter>;
}
