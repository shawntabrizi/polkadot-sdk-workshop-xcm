use crate::parachain::MsgQueue;
use frame_support::parameter_types;
use xcm::latest::prelude::*;

parameter_types! {
    pub KsmPerSecondPerByte: (AssetId, u128, u128) = (AssetId(Parent.into()), 1, 1);
    pub const MaxAssetsIntoHolding: u32 = 64;
}

// You are a parachain on Kusama, these are fixed constants for you.
parameter_types! {
    pub const KsmLocation: Location = Location::parent();
    pub const RelayNetwork: NetworkId = NetworkId::Kusama;
    pub UniversalLocation: InteriorLocation = [GlobalConsensus(RelayNetwork::get()), Parachain(MsgQueue::parachain_id().into())].into();
}

#[cfg(feature = "lesson-constants")]
pub mod sandbox {
    use super::*;

    parameter_types! {
        // How would you define the location of a sibling parachain on Kusama with Id = 1337?
        pub Sib1337Location: Location = Parachain(1337).into();
        // How would you define `DotLocation`? (the location of the Polkadot relay chain and its native asset)
        pub DotLocation: Location = GlobalConsensus(NetworkId::Polkadot).into();
        // How would you define a parachain on Polkadot with Id = 69?
        pub Para69Location: Location = [GlobalConsensus(NetworkId::Polkadot), Parachain(69)].into();
    }
}
