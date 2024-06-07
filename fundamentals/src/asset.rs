//! # Fundamentals Lesson 2: Assets
//!
//! All locations in this module are relative to Polkadot parachain 2000.
//!
//! # Assets
//! 
//! Now that you are familiar with XCM Locations, we can take a look at how we use these locations to identify particular Assets within a consensus system.
//! 
//! ## Fungible Tokens
//! 
//! ## Non-Fungible Tokens
//! 
//! ## Asset Filters

use frame_support::parameter_types;
use xcm::latest::prelude::*;

// Polkadot Topography
//
//                             ┌───────────┐
//                             │  Relay A  │
//                             │  Polkadot │
//                             └─────┬─────┘
//                                   │
//                        ┌──────────┴──────────┐
//                        │                     │
//                  ┌─────┴─────┐         ┌─────┴─────┐
//                  │  AssetHub │         │  HydraDx  │
//                  │  Id 1000  │         │  Id 2034  │
//                  └─────┬─────┘         └───────────┘
//                        │
//                 ┌──────┴──────┐
//                 │             │
//           ┌─────┴─────┐ ┌─────┴─────┐
//           │  Pallet   │ │  Pallet   │
//           │  Assets   │ │    NFT    │
//           │           │ │           │
//           │ Pallet #2 │ │ Pallet #3 │
//           └─────┬─────┘ └─────┬─────┘
//                 │             │
//       ┌─────────┴───┐         └───┬─────────────┐
//       │             │             │             │
// ┌─────┴─────┐ ┌─────┴─────┐ ┌─────┴─────┐ ┌─────┴─────┐
// │   Asset   │ │   Asset   │ │Collection │ │Collection │
// │   wBTC    │ │   wDOT    │ │  Kitties  │ │  Zombies  │
// │           │ │           │ │           │ │           │
// │   Id 21   │ │   Id 100  │ │   Id 3    │ │   Id 66   │
// └───────────┘ └───────────┘ └───────────┘ └───────────┘

const HDX_DECIMALS: u32 = 12;
const DOT_DECIMALS: u32 = 10;

// Fungible Tokens
// Construct these assets from the perspective of Parachain A (1000).
parameter_types! {
	// `Assets` instance that contains no assets.
	pub EmptyAssets: Assets = vec![].into();
	// Native token of Polkadot Parachain A (1000).
	pub NativeToken: AssetId = Here.into();
	// The native token of the relay chain, i.e. DOT.
	pub DotToken: AssetId = Parent.into();
	// 100 of the parachain's native token
	pub OneHundredNative: Asset = (NativeToken::get(), 100u128 * 10u128.pow(HDX_DECIMALS)).into();
	// Some amount of the native token of the relay chain.
	pub OneHundredDot: Asset = (DotToken::get(), 100u128 * 10u128.pow(DOT_DECIMALS)).into();
}

// Non-Fungible Tokens
parameter_types! {
	// Location of NFT collection with id 42 inside of the uniques pallet in Polkadot parachain 1000.
	pub NftLocation: Location = Location::new(1, [Parachain(1000), PalletInstance(5), GeneralIndex(42)]);
	// The NFT with id 69 inside of that collection.
	pub Nft: Asset = (NftLocation::get(), 69u64).into();
}

// Asset Filters
parameter_types! {
	// A filter which will capture all possible assets.
	pub AllAssetsFilter: AssetFilter = AssetFilter::Wild(WildAsset::All);
	// A filter specific for the DOT Token
	pub DotFilter: AssetFilter = OneHundredDot::get().into();
	// A filter specific for the Native Token
	pub NativeFilter: AssetFilter = OneHundredNative::get().into();
}
