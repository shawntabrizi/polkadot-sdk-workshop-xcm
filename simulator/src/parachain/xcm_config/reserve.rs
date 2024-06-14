pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
	pub type TrustedReserves = ();
}

#[cfg(feature = "relay-token")]
mod sandbox {
	// We want to accept any location as a reserve for their own native token.
	// TODO: Finish type.
	pub type TrustedReserves = ();
}

#[cfg(any(
	feature = "other-parachain-tokens",
	feature = "register-assets",
	feature = "asset-hub",
	feature = "barrier"
))]
mod sandbox {
	use core::marker::PhantomData;
	use frame_support::{parameter_types, traits::ContainsPair};
	use sp_runtime::traits::Get;
	use xcm::latest::prelude::*;
	use xcm_builder::NativeAsset;

	pub struct ReserveAssetsFrom<T>(PhantomData<T>);
	impl<T: Get<Location>> ContainsPair<Asset, Location> for ReserveAssetsFrom<T> {
		fn contains(_asset: &Asset, _origin: &Location) -> bool {
			todo!()
		}
	}

	parameter_types! {
		pub AssetHubLocation: Location = Location::new(1, [Parachain(1000)]);
	}

	/// We trust other chains as reserves of their own asset AND assets from asset hub.
	// TODO: Finish type.
	pub type TrustedReserves = ();
}
