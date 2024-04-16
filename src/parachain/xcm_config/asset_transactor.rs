use polkadot_parachain_primitives::primitives::Sibling;
use xcm::latest::prelude::*;
use xcm_builder::{
    AccountId32Aliases, ConvertedConcreteId, FungibleAdapter, IsConcrete, NoChecking,
    NonFungiblesAdapter, ParentIsPreset, SiblingParachainConvertsVia,
};
use xcm_executor::traits::JustTry;

use super::{ForeignUniques, KsmLocation, LocationToAccountId, RelayNetwork};

pub type SovereignAccountOf<AccountId> = (
    SiblingParachainConvertsVia<Sibling, AccountId>,
    AccountId32Aliases<RelayNetwork, AccountId>,
    ParentIsPreset<AccountId>,
);

pub type LocalAssetTransactor<AccountId, BalancesPallet> = (
    FungibleAdapter<
        BalancesPallet,
        IsConcrete<KsmLocation>,
        LocationToAccountId<AccountId>,
        AccountId,
        (),
    >,
    NonFungiblesAdapter<
        ForeignUniques,
        ConvertedConcreteId<Location, AssetInstance, JustTry, JustTry>,
        SovereignAccountOf<AccountId>,
        AccountId,
        NoChecking,
        (),
    >,
);
