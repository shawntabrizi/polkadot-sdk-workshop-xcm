use xcm_builder::{
    AsPrefixedGeneralIndex, ConvertedConcreteId, FungibleAdapter, IsConcrete, NoChecking,
    NonFungiblesAdapter,
};
use xcm_executor::traits::JustTry;

use super::{Balances, LocationToAccountId, TokenLocation, Uniques};

pub type LocalAssetTransactor<AccountId> = (
    FungibleAdapter<
        Balances,
        IsConcrete<TokenLocation>,
        LocationToAccountId<AccountId>,
        AccountId,
        (),
    >,
    NonFungiblesAdapter<
        Uniques,
        ConvertedConcreteId<u32, u32, AsPrefixedGeneralIndex<(), u32, JustTry>, JustTry>,
        LocationToAccountId<AccountId>,
        AccountId,
        NoChecking,
        (),
    >,
);
