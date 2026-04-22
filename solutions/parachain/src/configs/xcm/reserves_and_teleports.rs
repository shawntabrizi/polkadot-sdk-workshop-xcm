use super::ASSET_HUB_ID;

use core::marker::PhantomData;

use frame_support::{parameter_types, traits::ContainsPair};
use sp_runtime::traits::Get;
use xcm::prelude::*;

pub struct NativeAssetFrom<T>(PhantomData<T>);
impl<T: Get<Location>> ContainsPair<Asset, Location> for NativeAssetFrom<T> {
	fn contains(asset: &Asset, location: &Location) -> bool {
		let loc = T::get();
		&loc == location &&
			matches!(asset, Asset { id: AssetId(asset_location), fun: Fungible(_) }
            if *asset_location == Location::here())
	}
}

pub struct RelayAssetFrom<T>(PhantomData<T>);
impl<T: Get<Location>> ContainsPair<Asset, Location> for RelayAssetFrom<T> {
	fn contains(asset: &Asset, location: &Location) -> bool {
		let loc = T::get();
		&loc == location &&
			matches!(asset, Asset { id: AssetId(asset_location), fun: Fungible(_) }
            if *asset_location == Location::parent())
	}
}

parameter_types! {
	pub AssetHubLocation: Location = Location::new(1, [Parachain(ASSET_HUB_ID)]);
}

pub type TrustedReserves = RelayAssetFrom<AssetHubLocation>;

/// We only allow teleports of our native asset PARA between here and AssetHub.
pub type TrustedTeleporters = NativeAssetFrom<AssetHubLocation>;
