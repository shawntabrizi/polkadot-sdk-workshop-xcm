use frame_support::parameter_types;
use xcm::latest::prelude::*;
use xcm_builder::{HashedDescription, DescribeFamily, DescribeAllTerminal, AccountId32Aliases};

use crate::parachain::{AccountId, MsgQueue};

parameter_types! {
    pub const KsmLocation: Location = Location::parent();
    pub const RelayNetwork: NetworkId = NetworkId::Kusama;
    pub UniversalLocation: InteriorLocation = Parachain(MsgQueue::parachain_id().into()).into();
}

pub type LocationToAccountId = (
    HashedDescription<AccountId, DescribeFamily<DescribeAllTerminal>>,
    AccountId32Aliases<RelayNetwork, AccountId>,
);
