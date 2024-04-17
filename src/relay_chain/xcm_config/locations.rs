use frame_support::parameter_types;
use xcm::latest::prelude::*;
use xcm_builder::{Account32Hash, AccountId32Aliases, ChildParachainConvertsVia};

use polkadot_parachain_primitives::primitives::Id as ParaId;

parameter_types! {
    pub const TokenLocation: Location = Here.into_location();
    pub RelayNetwork: NetworkId = ByGenesis([0; 32]);
    pub UniversalLocation: InteriorLocation = Here;
    pub UnitWeightCost: u64 = 1_000;
}

pub type LocationToAccountId<AccountId> = (
    ChildParachainConvertsVia<ParaId, AccountId>,
    AccountId32Aliases<RelayNetwork, AccountId>,
    Account32Hash<(), AccountId>,
);
