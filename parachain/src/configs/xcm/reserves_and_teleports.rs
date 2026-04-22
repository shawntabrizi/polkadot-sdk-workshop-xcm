#![allow(dead_code)]

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

// Chains this parachain trusts to hold assets in reserve on its behalf — i.e. when they
// say "this asset is backed 1:1 by us", we believe them.
//
// TODO: Configure. AssetHub is the system's reserve for the relay chain's native token.
// One of the helper structs above matches the "relay asset arriving from X" shape.
pub type TrustedReserves = ();

// Chains this parachain trusts to teleport (destroy + recreate) its own native asset.
// Teleport trust is stronger than reserve trust — both sides must agree on the asset's
// canonical total issuance.
//
// TODO: Configure. We only teleport our native asset with AssetHub.
pub type TrustedTeleporters = ();
