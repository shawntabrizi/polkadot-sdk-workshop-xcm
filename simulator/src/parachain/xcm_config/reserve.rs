pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
	pub type TrustedReserves = ();
}

#[cfg(not(feature = "start"))]
mod sandbox {
	use core::marker::PhantomData;
	use frame_support::{parameter_types, traits::ContainsPair};
	use sp_runtime::traits::Get;
	use xcm::latest::prelude::*;
	use xcm_builder::NativeAsset;

	pub struct ReserveAssetsFrom<T>(PhantomData<T>);
	impl<T: Get<Location>> ContainsPair<Asset, Location> for ReserveAssetsFrom<T> {
		fn contains(asset: &Asset, origin: &Location) -> bool {
			let prefix = T::get();
			&prefix == origin
		}
	}

	parameter_types! {
		pub AssetHubLocation: Location = Location::new(1, [Parachain(1000)]);
	}

	/// We trust other chains as reserves of their own asset AND assets from asset hub.
	pub type TrustedReserves = (NativeAsset, ReserveAssetsFrom<AssetHubLocation>);
}
