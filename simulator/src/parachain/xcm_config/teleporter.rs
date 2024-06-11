pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
	pub type TrustedTeleporters = ();
}

#[cfg(not(feature = "start"))]
mod sandbox {
	use frame_support::parameter_types;
	use xcm::latest::prelude::*;

	parameter_types! {
		pub NftCollectionOne: AssetFilter
			= Wild(AllOf { fun: WildNonFungible, id: AssetId((Parent, GeneralIndex(1)).into()) });
		pub NftCollectionOneForRelay: (AssetFilter, Location)
			= (NftCollectionOne::get(), (Parent,).into());
	}

	// TODO: Should revisit.
	pub type TrustedTeleporters = xcm_builder::Case<NftCollectionOneForRelay>;
}
