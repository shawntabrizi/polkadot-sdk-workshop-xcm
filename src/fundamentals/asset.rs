//! # Fundamentals Lesson 2
//!
//! All locations in this module are relative to Polkadot parachain 2000.

use frame_support::parameter_types;
use xcm::latest::prelude::*;

// Fungible token representations.
parameter_types! {
    // `Assets` instance that contains no assets.
    pub EmptyAssets: Assets = vec![].into();
    // Native token of Polkadot parachain 2000.
    pub NativeToken: AssetId = Here.into();
    // The native token of the relay token, i.e. DOT.
    pub DotToken: AssetId = Parent.into();
    // Some amount of the native token.
    pub AmountInNativeToken: Asset = (NativeToken::get(), 100u128).into();
    // Some amount of the native token of the relay chain.
    pub AmountInDot: Asset = (DotToken::get(), 100u128).into();
}

// Non-fungible representation.
parameter_types! {
    // Location of NFT collection with id 42 inside of the uniques pallet in Polkadot parachain 1000.
    pub NftLocation: Location = Location::new(1, [Parachain(1000), PalletInstance(5), GeneralIndex(42)]);
    // The NFT with id 69 inside of that collection.
    pub Nft: Asset = (NftLocation::get(), 69u64).into();
}
