pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
	pub type TrustedTeleporters = ();
}

#[cfg(not(feature = "start"))]
mod sandbox {
	use frame_support::parameter_types;
	use xcm::latest::prelude::*;

	parameter_types! {
		pub NativeToken: AssetFilter = Wild(AllOf { fun: WildFungible, id: AssetId(Here.into()) });
		pub AssetHubLocation: Location = Location::new(1, [Parachain(1000)]);
		pub AssetHubTrustedTeleporter: (AssetFilter, Location) = (NativeToken::get(), AssetHubLocation::get());
	}

	pub type TrustedTeleporters = xcm_builder::Case<AssetHubTrustedTeleporter>;
}
