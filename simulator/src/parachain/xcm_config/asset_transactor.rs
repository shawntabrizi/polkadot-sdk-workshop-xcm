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

#[cfg(feature = "relay-token")]
mod sandbox {
	use crate::parachain::{
		constants::KsmLocation, location_converter::LocationConverter, AccountId, Balances,
	};
	use xcm_builder::{
		IsConcrete, FungibleAdapter,
	};

	/// AssetTransactor for handling the relay chain token
	pub type FungibleTransactor = FungibleAdapter<
		// Use this implementation of the `fungible::*` traits.
		// `Balances` is the name given to the balances pallet in this particular recipe.
		// Any implementation of the traits would suffice.
		Balances,
		// This transactor deals with the native token of the Relay Chain.
		// This token is referenced by the Location of the Relay Chain relative to this chain
		// -- Location::parent().
		IsConcrete<KsmLocation>,
		// How to convert an XCM Location into a local account id.
		// This is also something that's configured in the XCM executor.
		LocationConverter,
		// The type for account ids, only needed because `fungible` is generic over it.
		AccountId,
		// Not tracking teleports.
		// This recipe only uses reserve asset transfers to handle the Relay Chain token.
		(),
	>;

	pub type AssetTransactor = FungibleTransactor;
}
