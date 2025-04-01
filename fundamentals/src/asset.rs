//! # Fundamentals Lesson 2
//!
//! All locations in this module are relative to Polkadot parachain 2000.

use frame_support::parameter_types;
use xcm::latest::prelude::*;

// Polkadot Topography
//
//                              ┌───────────┐
//                              │  Relay A  │
//                              │  Polkadot │
//                              └─────┬─────┘
//                                    │
//                         ┌──────────┴──────────┐
//                         │                     │
//                   ┌─────┴─────┐         ┌─────┴─────┐
//                   │  AssetHub │         │  HydraDx  │
//                   │  Id 1000  │         │  Id 2034  │
//                   └─────┬─────┘         └───────────┘
//                         │
//                  ┌──────┴──────┐
//                  │             │
//           ┌──────┴─────┐ ┌─────┴──────┐
//           │   Pallet   │ │   Pallet   │
//           │   Assets   │ │    NFT     │
//           │            │ │            │
//           │ Pallet #50 │ │ Pallet #52 │
//           └─────┬──────┘ └─────┬──────┘
//                 │              │
//       ┌─────────┴───┐          └────┬──────────────┐
//       │             │               │              │
// ┌─────┴─────┐ ┌─────┴─────┐  ┌──────┴─────┐ ┌──────┴─────┐
// │   Asset   │ │   Asset   │  │ Collection │ │ Collection │
// │   USDC    │ │   USDT    │  │   Kitties  │ │  Zombies   │
// │           │ │           │  │            │ │            │
// │ Id 1337   │ │  Id 1984  │  │    Id 3    │ │    Id 66   │
// └───────────┘ └───────────┘  └────────────┘ └────────────┘

const DOT_DECIMALS: u32 = 10;
const USDT_DECIMALS: u32 = 6;

// Fungible Tokens
// Construct these assets from the perspective of AssetHub (1000).
parameter_types! {
	// `Assets` instance that contains no assets.
	pub EmptyAssets: Assets = // TODO
	// USDT.
	pub Usdt: AssetId = // TODO
	// The native token of the relay chain, i.e. DOT.
	pub DotToken: AssetId = // TODO
	// 100 USDT.
	pub OneHundredUsdt: Asset = // TODO
	// Some amount of the native token of the relay chain.
	pub OneHundredDot: Asset = // TODO
}

// Non-Fungible Tokens
parameter_types! {
	// Location of NFT collection with id 3 inside of the NFT pallet in Polkadot parachain 1000.
	pub NftLocation: Location = // TODO
	// The NFT with id 69 inside of that collection.
	pub Nft: Asset = // TODO
}

// Asset Filters
parameter_types! {
	// A filter which will capture all possible assets.
	pub AllAssetsFilter: AssetFilter = // TODO
	// A filter specific for the DOT Token.
	pub DotFilter: AssetFilter = // TODO
	// A filter specific for USDT.
	pub UsdtFilter: AssetFilter = // TODO
}
