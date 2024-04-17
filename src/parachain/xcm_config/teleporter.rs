use frame_support::parameter_types;
use xcm::latest::prelude::*;

parameter_types! {
    pub NftCollectionOne: AssetFilter
        = Wild(AllOf { fun: WildNonFungible, id: AssetId((Parent, GeneralIndex(1)).into()) });
    pub NftCollectionOneForRelay: (AssetFilter, Location)
        = (NftCollectionOne::get(), (Parent,).into());
}

pub type TrustedTeleporters = xcm_builder::Case<NftCollectionOneForRelay>;
