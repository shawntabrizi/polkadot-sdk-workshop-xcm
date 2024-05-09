// Fundamentals Lesson 1

use frame_support::parameter_types;
use xcm::latest::prelude::*;

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
