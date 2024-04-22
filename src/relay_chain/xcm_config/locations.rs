use frame_support::parameter_types;
use xcm::latest::prelude::*;
use xcm_builder::{HashedDescription, DescribeFamily, DescribeAllTerminal, AccountId32Aliases};

use crate::relay_chain::AccountId;

parameter_types! {
    pub const TokenLocation: Location = Here.into_location();
    pub RelayNetwork: NetworkId = ByGenesis([0; 32]);
    pub UniversalLocation: InteriorLocation = Here;
    pub UnitWeightCost: u64 = 1_000;
}

pub type LocationToAccountId = (
    HashedDescription<AccountId, DescribeFamily<DescribeAllTerminal>>,
    AccountId32Aliases<RelayNetwork, AccountId>,
);
