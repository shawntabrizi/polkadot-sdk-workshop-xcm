pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
    pub type SovereignAccountOf = ();
    pub type AssetTransactor = ();
}

#[cfg(feature = "example")]
mod sandbox {
    use polkadot_parachain_primitives::primitives::Sibling;
    use xcm::latest::prelude::*;
    use xcm_builder::{
        AccountId32Aliases, ConvertedConcreteId, FungibleAdapter, IsConcrete, NoChecking,
        NonFungiblesAdapter, ParentIsPreset, SiblingParachainConvertsVia,
    };
    use xcm_executor::traits::JustTry;

    use crate::parachain::{
        AccountId, Balances, ForeignUniques, KsmLocation, LocationToAccountId, RelayNetwork,
    };

    pub type SovereignAccountOf = (
        SiblingParachainConvertsVia<Sibling, AccountId>,
        AccountId32Aliases<RelayNetwork, AccountId>,
        ParentIsPreset<AccountId>,
    );

    type LocalAssetTransactor = (
        FungibleAdapter<Balances, IsConcrete<KsmLocation>, LocationToAccountId, AccountId, ()>,
        NonFungiblesAdapter<
            ForeignUniques,
            ConvertedConcreteId<Location, AssetInstance, JustTry, JustTry>,
            SovereignAccountOf,
            AccountId,
            NoChecking,
            (),
        >,
    );

    pub type AssetTransactor = LocalAssetTransactor;
}
