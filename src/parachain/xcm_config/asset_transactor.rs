pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
    pub type AssetTransactor = ();
}

#[cfg(feature = "example")]
mod sandbox {
    use xcm::latest::prelude::*;
    use xcm_builder::{
        ConvertedConcreteId, FungibleAdapter, IsConcrete, NoChecking,
        NonFungiblesAdapter,
    };
    use xcm_executor::traits::JustTry;

    use crate::parachain::{
        AccountId, Balances, ForeignUniques, KsmLocation, LocationToAccountId,
    };

    type LocalAssetTransactor = (
        FungibleAdapter<Balances, IsConcrete<KsmLocation>, LocationToAccountId, AccountId, ()>,
        NonFungiblesAdapter<
            ForeignUniques,
            ConvertedConcreteId<Location, AssetInstance, JustTry, JustTry>,
            LocationToAccountId,
            AccountId,
            NoChecking,
            (),
        >,
    );

    pub type AssetTransactor = LocalAssetTransactor;
}
