// Fundamentals Lesson 1

use frame_support::parameter_types;
use xcm::latest::prelude::*;

// TODO: Junctions

parameter_types! {
    // How would you define the location of a sibling parachain on Kusama with Id = 1337?
    pub Sib1337Location: Location = Parachain(1337).into();
    // How would you define `DotLocation`? (the location of the Polkadot relay chain and its native asset)
    pub DotLocation: Location = GlobalConsensus(NetworkId::Polkadot).into();
    // How would you define a parachain on Polkadot with Id = 69?
    pub Para69Location: Location = [GlobalConsensus(NetworkId::Polkadot), Parachain(69)].into();
}

// How would you define the location of account `crate::ALICE` on parachain 1337?
// TODO: Make better
parameter_types! {
    pub AliceBytes: [u8; 32] = crate::ALICE.into();
    pub Alice1337Location: Location = [Parachain(1337), AliceBytes::get().into()].into();
}

// Take a look at the configuration of the Relay Chain runtime.
// How would you define the location of the `Balances` pallet on the Relay Chain?
parameter_types! {
    pub RelayChainBalancesLocation: Location = [PalletInstance(1)].into();
}
