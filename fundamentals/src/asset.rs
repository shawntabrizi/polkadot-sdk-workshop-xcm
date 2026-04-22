#![allow(dead_code)]
//! # Fundamentals Lesson 2
//!
//! All locations in this module are relative to AssetHub (Polkadot parachain 1000).

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

const DOT_DECIMALS: u128 = 10_000_000_000;
const USDT_DECIMALS: u128 = 1_000_000;

// Fungible Tokens
// Construct these assets from the perspective of AssetHub (parachain 1000).
parameter_types! {
	// ✅ Worked example — an `Assets` collection containing nothing.
	// An empty `Vec<Asset>` coerces into `Assets` via `.into()`.
	pub EmptyAssets: Assets = vec![].into();

	// TODO: The `AssetId` for USDT, from AssetHub's own view.
	// Hint: USDT is asset 1984 inside the Assets pallet (index 50) on this chain.
	//       An `AssetId` wraps a `Location`.
	pub Usdt: AssetId = todo!();

	// TODO: The `AssetId` for DOT (the relay's native token), from AssetHub's view.
	// Hint: DOT is native to the relay — what's the relay's location from here?
	pub DotToken: AssetId = todo!();

	// TODO: 100 USDT as a single fungible `Asset`.
	// Hint: an `Asset` is "what" + "how much". `USDT_DECIMALS` is declared above.
	pub OneHundredUsdt: Asset = todo!();

	// TODO: 100 DOT, following the same pattern as `OneHundredUsdt`.
	pub OneHundredDot: Asset = todo!();
}

// Non-Fungible Tokens
parameter_types! {
	// TODO: The location of NFT collection 3, inside the NFT pallet (index 52) on
	//       Polkadot parachain 1000.
	pub NftLocation: Location = todo!();

	// TODO: The NFT with id 69 inside that collection, as a non-fungible `Asset`.
	// Hint: for NFTs the fungibility carries an instance identifier.
	pub Nft: Asset = todo!();
}

// Asset Filters
parameter_types! {
	// TODO: A filter that matches every possible asset.
	// Hint: `AssetFilter` has a wildcard variant.
	pub AllAssetsFilter: AssetFilter = todo!();

	// TODO: A filter that matches only the DOT asset.
	// Hint: a specific `Asset` can become a single-item filter — try `.into()` on the
	//       asset you already constructed above.
	pub DotFilter: AssetFilter = todo!();

	// TODO: A filter that matches only USDT (same pattern as `DotFilter`).
	pub UsdtFilter: AssetFilter = todo!();
}
