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
		// ✅ Worked example — Polkadot parachain 1000, from the relay's view.
		// `Parachain(id)` is a junction; `.into()` wraps it in a Location with 0 parents.
		pub PolkadotPara1000: Location = Parachain(1000).into();

		// TODO: The Polkadot parachain B with id 2004.
		pub PolkadotPara2004: Location = todo!();

		// TODO: The Polkadot relay chain, from its own perspective.
		pub PolkadotRelay: Location = todo!();

		// A 32 byte account on para 1000 with all bytes equal to 1 (Alice).
		// Hint: `ALICE` is already imported as a `[u8; 32]` constant.
		pub AliceBytes: [u8; 32] = todo!();

		// TODO: Alice's account *on* parachain 1000, from the relay's view.
		// Hint: this location names *two* things: which parachain, and which account on it.
		pub PolkadotPara1000Alice: Location = todo!();

		// TODO: The `Assets` pallet (index 50) on parachain 1000.
		// Hint: a Location can be built from a tuple of junctions via `.into()`.
		pub PolkadotPara1000AssetsPallet: Location = todo!();

		// TODO: Asset 1984 inside the Assets pallet on parachain 1000.
		// Hint: assets inside a pallet are addressed by a numeric index; there's a junction for that.
		pub PolkadotPara1000Asset1984: Location = todo!();

		// TODO: The Kusama parachain with id 1000, from Polkadot's relay view.
		// Hint: to reach something in a different consensus system, you have to leave yours first.
		pub KusamaPara1000: Location = todo!();
	}
}

/// All these locations are relative to a Polkadot parachain with id 1000.
///
/// Notice that every entry below names the *same thing* as the previous module — only the
/// perspective changes. Locations are relative; the answers will not match.
pub mod relative_to_polkadot_para_1000 {
	use super::*;

	parameter_types! {
		// TODO: Parachain 1000, from its own perspective.
		pub PolkadotPara1000: Location = todo!();

		// TODO: Sibling parachain 2004, from para 1000's view.
		// Hint: to reach a sibling, you first have to leave your own parachain.
		pub PolkadotPara2004: Location = todo!();

		// TODO: The Polkadot relay chain, from para 1000's view.
		pub PolkadotRelay: Location = todo!();

		// Same `AliceBytes` as the previous module.
		pub AliceBytes: [u8; 32] = todo!();

		// TODO: Alice's account *on this parachain*.
		pub PolkadotPara1000Alice: Location = todo!();

		// TODO: The `Assets` pallet on this parachain.
		pub PolkadotPara1000AssetsPallet: Location = todo!();

		// TODO: Asset 1984 on this parachain.
		pub PolkadotPara1000Asset1984: Location = todo!();

		// TODO: Kusama parachain 1000, from Polkadot parachain 1000.
		// Hint: count the boundaries you have to cross: out of your parachain, out of Polkadot.
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
		// TODO: Polkadot parachain 1000, absolutely.
		pub PolkadotPara1000: Location = todo!();

		// TODO: Polkadot parachain 2004, absolutely.
		pub PolkadotPara2004: Location = todo!();

		// TODO: The Polkadot relay chain, absolutely.
		pub PolkadotRelay: Location = todo!();

		pub AliceBytes: [u8; 32] = todo!();

		// TODO: Alice's account on Polkadot parachain 1000, absolutely.
		pub PolkadotPara1000Alice: Location = todo!();

		// TODO: The `Assets` pallet on Polkadot parachain 1000.
		pub PolkadotPara1000AssetsPallet: Location = todo!();

		// TODO: Asset 1984 on that pallet on that parachain.
		pub PolkadotPara1000Asset1984: Location = todo!();

		// TODO: Kusama parachain 1000, absolutely.
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
