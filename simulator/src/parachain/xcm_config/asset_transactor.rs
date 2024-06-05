pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
	pub type AssetTransactor = ();
}

#[cfg(feature = "example")]
mod sandbox {
	use crate::parachain::{
		constants::KsmLocation, location_converter::LocationConverter, AccountId, Balances,
		ForeignUniques,
	};
	use xcm::latest::prelude::*;
	use xcm_builder::{
		ConvertedConcreteId, FungibleAdapter, IsConcrete, NoChecking, NonFungiblesAdapter,
	};
	use xcm_executor::traits::JustTry;

	type LocalAssetTransactor = (
		FungibleAdapter<Balances, IsConcrete<KsmLocation>, LocationConverter, AccountId, ()>,
		NonFungiblesAdapter<
			ForeignUniques,
			ConvertedConcreteId<Location, AssetInstance, JustTry, JustTry>,
			LocationConverter,
			AccountId,
			NoChecking,
			(),
		>,
	);

	pub type AssetTransactor = LocalAssetTransactor;
}
