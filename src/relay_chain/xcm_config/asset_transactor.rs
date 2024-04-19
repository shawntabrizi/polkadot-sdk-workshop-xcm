use xcm_builder::{
    AsPrefixedGeneralIndex, ConvertedConcreteId, FungibleAdapter, IsConcrete, NoChecking,
    NonFungiblesAdapter,
};
use xcm_executor::traits::JustTry;

use crate::relay_chain::{
    locations::{LocationToAccountId, TokenLocation},
    AccountId, Balances, Uniques,
};

pub type LocalAssetTransactor = (
    FungibleAdapter<Balances, IsConcrete<TokenLocation>, LocationToAccountId, AccountId, ()>,
    NonFungiblesAdapter<
        Uniques,
        ConvertedConcreteId<u32, u32, AsPrefixedGeneralIndex<(), u32, JustTry>, JustTry>,
        LocationToAccountId,
        AccountId,
        NoChecking,
        (),
    >,
);
