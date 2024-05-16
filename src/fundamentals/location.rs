//! # Fundamentals Lesson 1

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
//        │  Id 1000  │         │  Id 2000  │
//        └─────┬─────┘         └─────┬─────┘
//              │                     │
//       ┌──────┴──────┐              ├───────────┐
//       │             │              │           │
// ┌─────┴─────┐ ┌─────┴─────┐ ┌──────┴────┐ ┌────┴──────┐
// │  User 1   │ │  Pallet   │ │  User 2   │ │  Pallet   │
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

use frame_support::parameter_types;
use xcm::latest::prelude::*;

/// All these locations are relative to the Polkadot Relay Chain.
pub mod relative_to_polkadot_relay {
	use super::*;

	parameter_types! {
		// The Polkadot parachain with id 1000.
		pub PolkadotPara1000: Location = Parachain(1000).into();
		// The Polkadot parachain with id 1337.
		pub PolkadotPara1337: Location = Parachain(1337).into();
		// The Polkadot relay chain.
		pub PolkadotRelay: Location = Here.into();
		// A 32 byte account on para 1337.
		pub AliceBytes: [u8; 32] = crate::ALICE.into();
		pub PolkadotPara1337Alice: Location = Location::new(0, [Parachain(1337), AliceBytes::get().into()]);
		// The location of the `Balances` pallet on the relay chain.
		pub PolkadotRelayBalancesPallet: Location = PalletInstance(1).into();
		// The asset with index `1984` of the Assets pallet on the Polkadot parachain with id 1000.
		pub PolkadotPara1000Asset1984: Location = (Parachain(1000), PalletInstance(2), GeneralIndex(1984)).into();
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
		pub AliceBytes: [u8; 32] = crate::ALICE.into();
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
// │ Polkadot │ │  Kusama  │ │ Bitcoin  │ │ Ethereum │
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
		pub AliceBytes: [u8; 32] = crate::ALICE.into();
		pub PolkadotPara1337Alice: Location = [GlobalConsensus(Polkadot), Parachain(1337), AliceBytes::get().into()].into();
		// The location of the `Balances` pallet on the relay chain.
		pub PolkadotRelayBalancesPallet: Location = [GlobalConsensus(Polkadot), PalletInstance(1)].into();
		// The asset with index `1984` of the Assets pallet on the Polkadot parachain with id 1000.
		pub PolkadotPara1000Asset1984: Location = [GlobalConsensus(Polkadot), Parachain(1000), PalletInstance(2), GeneralIndex(1984)].into();
		// The Kusama parachain with id 69.
		pub KusamaPara69: Location = [GlobalConsensus(Kusama), Parachain(69)].into();
	}
}
