//! # Fundamentals Lesson 1: Locations
//!
//! In the world of XCM, the first fundamental primitive you need to learn about is Locations.
//! 
//! As a software developer, Locations are most similar to filepaths in a filesystem.
//! 
//! When writing a program, filepaths can help you locate folders, which contain many items, and the items themselves.
//! 
//! This is very similar to Locations in XCM, which can be used to locate consensus systems, like Relay Chains, Parachains, Smart Contracts, etc... and also specific users, assets, applications, and groups.
//! 
//! Let's explore how Locations work inside of XCM.
//!
//! ## Relative Locations
//!
//! By default, locations in XCM are always relative.
//! 
//! This means, when considering how to construct a location, you must first establish your relative perspective.
//! 
//! For example, take the following topography:
//! 
//! ```text
//!                   ┌───────────┐
//!                   │  Relay A  │
//!                   │  Polkadot │
//!                   └─────┬─────┘
//!                         │
//!              ┌──────────┴──────────┐
//!              │                     │
//!        ┌─────┴─────┐         ┌─────┴─────┐
//!        │  Para A   │         │  Para B   │
//!        │  Id 1000  │         │  Id 1337  │
//!        └─────┬─────┘         └─────┬─────┘
//!              │                     │
//!       ┌──────┴──────┐              ├───────────┐
//!       │             │              │           │
//! ┌─────┴─────┐ ┌─────┴─────┐ ┌──────┴────┐ ┌────┴──────┐
//! │   Alice   │ │  Pallet   │ │    Bob    │ │  Pallet   │
//! │ AcctKey32 │ │  Assets   │ │ AcctKey20 │ │   EVM     │
//! │           │ │           │ │           │ │           │
//! │ 0x11111...│ │ Pallet #2 │ │ 0x22222...│ │ Pallet #5 │
//! └───────────┘ └─────┬─────┘ └───────────┘ └─────┬─────┘
//!                     │                           │
//!               ┌─────┴─────┐               ┌─────┴─────┐
//!               │   Asset   │               │   Smart   │
//!               │   wBTC    │               │ Contract  │
//!               │           │               │           │
//!               │   Id 21   │               │ 0x55555...│
//!               └───────────┘               └───────────┘
//! ```
//! 
//! Let's treat this like a file system, and say we want to locate `Asset wBTC - ID 21`.
//! 
//! From the perspective of the Polkadot relay chain:
//! 
//! ```text
//! ./{Para A}/{Pallet Assets}/{Asset wBTC}
//! ```
//! 
//! But from the perspective of Smart Contract `0x555...`, which might be a DEX used to trade `wBTC`:
//! 
//! ```text
//! ../../../{Para A}/{Pallet Assets}/{Asset wBTC}
//! ```
//!
//! ## The Location Format
//! 
//! So nothing new to learn about with locations, we just need to break down how they are represented in the XCM system.
//! 
//! ### Junction
//! 
//! The building blocks of a location are Junctions.
//! 
//! ```rust
//! pub enum Junction {
//! 	Parachain(#[codec(compact)] u32),
//! 	PalletInstance(u8),
//! 	GeneralIndex(#[codec(compact)] u128),
//! 	// ... there are more junctions
//! }
//! ```

use crate::constants::ALICE;
use frame_support::parameter_types;
use xcm::latest::prelude::*;

// Relay A Topography:
//                   ┌───────────┐
//                   │  Relay A  │
//                   │  Polkadot │
//                   └─────┬─────┘
//                         │
//              ┌──────────┴──────────┐
//              │                     │
//        ┌─────┴─────┐         ┌─────┴─────┐
//        │  Para A   │         │  Para B   │
//        │  Id 1000  │         │  Id 1337  │
//        └─────┬─────┘         └─────┬─────┘
//              │                     │
//       ┌──────┴──────┐              ├───────────┐
//       │             │              │           │
// ┌─────┴─────┐ ┌─────┴─────┐ ┌──────┴────┐ ┌────┴──────┐
// │   Alice   │ │  Pallet   │ │    Bob    │ │  Pallet   │
// │ AcctKey32 │ │  Assets   │ │ AcctKey20 │ │   EVM     │
// │           │ │           │ │           │ │           │
// │ 0x11111...│ │ Pallet #2 │ │ 0x22222...│ │ Pallet #5 │
// └───────────┘ └─────┬─────┘ └───────────┘ └─────┬─────┘
//                     │                           │
//               ┌─────┴─────┐               ┌─────┴─────┐
//               │   Asset   │               │   Smart   │
//               │   wBTC    │               │ Contract  │
//               │           │               │           │
//               │   Id 21   │               │ 0x55555...│
//               └───────────┘               └───────────┘

/// All these locations are relative to the Polkadot Relay Chain.
pub mod relative_to_polkadot_relay {
	use super::*;

	parameter_types! {
		// The Polkadot parachain A with id 1000.
		pub PolkadotPara1000: Location = Parachain(1000).into();
		// The Polkadot parachain B with id 1337.
		pub PolkadotPara1337: Location = Parachain(1337).into();
		// The Polkadot relay chain.
		pub PolkadotRelay: Location = Here.into();
		// A 32 byte account on para 1000 with all bytes equal to 1 (Alice).
		pub AliceBytes: [u8; 32] = ALICE.into();
		pub PolkadotPara1337Alice: Location = Location::new(0, [Parachain(1337), AliceBytes::get().into()]);
		// The location of the `Assets` pallet on the relay chain.
		pub PolkadotRelayBalancesPallet: Location = PalletInstance(2).into();
		// The asset with index `21` of the Assets pallet on parachain A with id 1000.
		pub PolkadotPara1000Asset21: Location = (Parachain(1000), PalletInstance(2), GeneralIndex(21)).into();
		// The Kusama parachain with id 69.
		pub KusamaPara69: Location = (Parent, GlobalConsensus(Kusama), Parachain(69)).into();
	}
}

/// All these locations are relative to a Polkadot parachain with id 1000.
pub mod relative_to_polkadot_para_1000 {
	use super::*;

	parameter_types! {
		// The Polkadot parachain with id 1000.
		pub PolkadotPara1000: Location = Here.into();
		// The Polkadot parachain with id 1337.
		pub PolkadotPara1337: Location = (Parent, Parachain(1337)).into();
		// The Polkadot relay chain.
		pub PolkadotRelay: Location = Parent.into();
		// A 32 byte account on para 1337.
		pub AliceBytes: [u8; 32] = ALICE.into();
		pub PolkadotPara1337Alice: Location = Location::new(1, [Parachain(1337), AliceBytes::get().into()]);
		// The location of the `Balances` pallet on the relay chain.
		pub PolkadotRelayBalancesPallet: Location = (Parent, PalletInstance(1)).into();
		// The asset with index `1984` of the Assets pallet on the Polkadot parachain with id 1000.
		pub PolkadotPara1000Asset1984: Location = (PalletInstance(2), GeneralIndex(1984)).into();
		// The Kusama parachain with id 69.
		pub KusamaPara69: Location = (Parent, Parent, GlobalConsensus(Kusama), Parachain(69)).into();
	}
}

/// All these locations are relative to an EVM Smart Contract on Parachain B, secured by Polkadot.
pub mod relative_to_polkadot_para_2000_sc {
	use super::*;

	parameter_types! {
		// TODO
	}
}

// Relay B Topography:
//                ┌───────────┐
//                │  Relay B  │
//                │  Kusama   │
//                └─────┬─────┘
//                      │
//       ┌──────────────┼──────────────┐
//       │              │              │
// ┌─────┴─────┐  ┌─────┴─────┐  ┌─────┴─────┐
// │  Para C   │  │  User 1   │  │           │
// │  Id 1000  │  │ AcctKey32 │  │ Plurality │
// └─────┬─────┘  │           │  │           │
//       │        │ 0x11111...│  │           │
// ┌─────┴─────┐  └───────────┘  └───────────┘
// │  Pallet   │
// │    NFT    │
// │           │
// │ Pallet #3 │
// └─────┬─────┘
//       │
// ┌─────┴─────┐
// │    NFT    │
// │  Kitties  │
// │           │
// │   Id 21   │
// └───────────┘

/// All these locations are relative to the Kusama Relay Chain.
pub mod relative_to_kusama_relay {
	use super::*;

	parameter_types! {
		// TODO
	}
}

/// All these locations are relative to a Kusama parachain with id 1000.
pub mod relative_to_kusama_para_1000 {
	use super::*;

	parameter_types! {
		// TODO
	}
}

// Absolute Topography
//                    ┌ ─ ─ ─ ─ ─┐
//                      Absolute
//                      Position
//                    └ ─ ─ ┬ ─ ─┘
//                          │
//      ┌─────────────┬─────┴─────┬────────────┐
//      │             │           │            │
// ┌────┴─────┐ ┌─────┴────┐ ┌────┴─────┐ ┌────┴─────┐
// │ Relay A  │ │ Relay B  │ │          │ │          │
// │ Polkadot │ │ Kusama   │ │ Bitcoin  │ │ Ethereum │
// └────┬─────┘ └─────┬────┘ └────┬─────┘ └─────┬────┘
//      │             │           │             │
//     ...           ...         ...           ...

/// All these locations are absolute.
/// Absolute locations have no parents and always start with the `GlobalConsensus` junction.
pub mod absolute {
	use super::*;

	parameter_types! {
		// The Polkadot parachain with id 1000.
		pub PolkadotPara1000: Location = [GlobalConsensus(Polkadot), Parachain(1000)].into();
		// The Polkadot parachain with id 1337.
		pub PolkadotPara1337: Location = [GlobalConsensus(Polkadot), Parachain(1337)].into();
		// The Polkadot relay chain.
		pub PolkadotRelay: Location = [GlobalConsensus(Polkadot)].into();
		// A 32 byte account on para 1337.
		pub AliceBytes: [u8; 32] = ALICE.into();
		pub PolkadotPara1337Alice: Location = [GlobalConsensus(Polkadot), Parachain(1337), AliceBytes::get().into()].into();
		// The location of the `Balances` pallet on the relay chain.
		pub PolkadotRelayBalancesPallet: Location = [GlobalConsensus(Polkadot), PalletInstance(1)].into();
		// The asset with index `1984` of the Assets pallet on the Polkadot parachain with id 1000.
		pub PolkadotPara1000Asset1984: Location = [GlobalConsensus(Polkadot), Parachain(1000), PalletInstance(2), GeneralIndex(1984)].into();
		// The Kusama parachain with id 69.
		pub KusamaPara69: Location = [GlobalConsensus(Kusama), Parachain(69)].into();
	}
}
