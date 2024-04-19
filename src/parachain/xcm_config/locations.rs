use frame_support::parameter_types;
use polkadot_parachain_primitives::primitives::Sibling;
use xcm::latest::prelude::*;
use xcm_builder::{Account32Hash, AccountId32Aliases, ParentIsPreset, SiblingParachainConvertsVia};

use crate::parachain::{AccountId, MsgQueue};

parameter_types! {
    pub const KsmLocation: Location = Location::parent();
    pub const RelayNetwork: NetworkId = NetworkId::Kusama;
    pub UniversalLocation: InteriorLocation = Parachain(MsgQueue::parachain_id().into()).into();
}

pub type LocationToAccountId = (
    ParentIsPreset<AccountId>,
    SiblingParachainConvertsVia<Sibling, AccountId>,
    AccountId32Aliases<RelayNetwork, AccountId>,
    Account32Hash<(), AccountId>,
);
