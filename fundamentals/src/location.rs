#![allow(unused_imports)]
//! # Fundamentals Lesson 1
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
//        ┌─────┴─────┐         ┌─────┴──────┐
//        │ AssetHub  │         │  Moonbeam  │
//        │  Id 1000  │         │  Id 2004   │
//        └─────┬─────┘         └──────┬─────┘
//              │                      │
//       ┌──────┴──────┐               ├───────────┐
//       │             │               │           │
// ┌─────┴─────┐ ┌─────┴──────┐ ┌──────┴────┐ ┌────┴───────┐
// │   Alice   │ │  Pallet    │ │    Bob    │ │  Pallet    │
// │ AcctKey32 │ │  Assets    │ │ AcctKey20 │ │   EVM      │
// │           │ │            │ │           │ │            │
// │ 0x11111...│ │ Pallet #50 │ │ 0x22222...│ │ Pallet #51 │
// └───────────┘ └─────┬──────┘ └───────────┘ └─────┬──────┘
//                     │                            │
//               ┌─────┴─────┐               ┌──────┴─────┐
//               │   Asset   │               │   Smart    │
//               │   USDT    │               │ Contract   │
//               │           │               │            │
//               │  Id 1984  │               │ 0x55555... │
//               └───────────┘               └────────────┘

/// All these locations are relative to the Polkadot Relay Chain.
pub mod relative_to_polkadot_relay {
	use super::*;

	parameter_types! {
		// The Polkadot parachain A with id 1000.
		pub PolkadotPara1000: Location = todo!();
		// The Polkadot parachain B with id 2004.
		pub PolkadotPara2004: Location = todo!();
		// The Polkadot relay chain.
		pub PolkadotRelay: Location = todo!();
		// A 32 byte account on para 1000 with all bytes equal to 1 (Alice).
		pub AliceBytes: [u8; 32] = todo!();
		pub PolkadotPara1000Alice: Location = todo!();
		// The location of the `Assets` pallet on the relay chain.
		pub PolkadotPara1000AssetsPallet: Location = todo!();
		// The asset with index `1984` of the Assets pallet on polkadot parachain with id 1000.
		pub PolkadotPara1000Asset1984: Location = todo!();
		// The Kusama parachain with id 1000.
		pub KusamaPara1000: Location = todo!();
	}
}

/// All these locations are relative to a Polkadot parachain with id 1000.
pub mod relative_to_polkadot_para_1000 {
	use super::*;

	parameter_types! {
		// The Polkadot parachain with id 1000.
		pub PolkadotPara1000: Location = todo!();
		// The Polkadot parachain with id 2004.
		pub PolkadotPara2004: Location = todo!();
		// The Polkadot relay chain.
		pub PolkadotRelay: Location = todo!();
		// A 32 byte account on para 1000.
		pub AliceBytes: [u8; 32] = todo!();
		pub PolkadotPara1000Alice: Location = todo!();
		// The location of the `Balances` pallet on the relay chain.
		pub PolkadotPara1000AssetsPallet: Location = todo!();
		// The asset with index `1984` of the Assets pallet on the Polkadot parachain with id 1000.
		pub PolkadotPara1000Asset1984: Location = todo!();
		// The Kusama parachain with id 1000.
		pub KusamaPara1000: Location = todo!();
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
// │ AssetHub  │  │  User 1   │  │           │
// │  Id 1000  │  │ AcctKey32 │  │ Plurality │
// └─────┬─────┘  │           │  │           │
//       │        │ 0x11111...│  │           │
// ┌─────┴──────┐ └───────────┘  └───────────┘
// │  Pallet    │
// │    NFT     │
// │            │
// │ Pallet #52 │
// └─────┬──────┘
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
//                      Location
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
		pub PolkadotPara1000: Location = todo!();
		// The Polkadot parachain with id 2004.
		pub PolkadotPara2004: Location = todo!();
		// The Polkadot relay chain.
		pub PolkadotRelay: Location = todo!();
		// A 32 byte account on para 1000.
		pub AliceBytes: [u8; 32] = todo!();
		pub PolkadotPara1000Alice: Location = todo!();
		// The location of the `Balances` pallet on the relay chain.
		pub PolkadotPara1000AssetsPallet: Location = todo!();
		// The asset with index `1984` of the Assets pallet on the Polkadot parachain with id 1000.
		pub PolkadotPara1000Asset1984: Location = todo!();
		// The Kusama parachain with id 1000.
		pub KusamaPara1000: Location = todo!();
	}
}

pub mod manipulation {
	use super::*;
	use sp_runtime::AccountId32;

	// Extract the account id from a Location, if it is the last junction in the Location.
	pub fn extract_last_account_id(location: Location) -> Option<AccountId32> {
		match location.last() {
			Some(Junction::AccountId32 { id, .. }) => Some((*id).into()),
			_ => None,
		}
	}

	// From the perspective of a parachain, check if another location is a sibling parachain, and
	// return the id.
	pub fn check_sibling_parachains(maybe_sibling: Location) -> Option<u32> {
		match maybe_sibling.unpack() {
			(1, [Parachain(id)]) => Some(*id),
			_ => None,
		}
	}

	// Append `who` to the current `origin`.
	pub fn descend_origin(origin: &mut Location, who: Location) -> Result<(), XcmError> {
		(*origin).append_with(who).map_err(|_| XcmError::LocationFull)
	}
}
