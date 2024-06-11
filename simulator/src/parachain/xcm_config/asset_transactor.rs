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
	use xcm_builder::{FungibleAdapter, IsConcrete};

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

#[cfg(any(feature = "other-parachain-tokens", feature = "register-assets"))]
mod sandbox {
	use frame_support::{parameter_types, traits::EverythingBut};
	use xcm::prelude::*;
	use xcm_builder::{
		FungibleAdapter, FungiblesAdapter, IsConcrete, MatchedConvertedConcreteId, NoChecking,
		StartsWith,
	};
	use xcm_executor::traits::JustTry;

	use crate::parachain::{
		location_converter::LocationConverter, AccountId, Balance, Balances, ForeignAssets,
		PolkadotXcm,
	};

	/// AssetTransactor for handling the chain's native token.
	pub type FungibleTransactor = FungibleAdapter<
		// Use this implementation of the `fungible::*` traits.
		// `Balances` is the name given to the balances pallet in this particular example.
		// Any implementation of the traits would suffice.
		Balances,
		// This transactor deals with the native token of the Relay Chain.
		// This token is referenced by the Location of the Relay Chain relative to this chain
		// -- Location::here().
		IsConcrete<LocalPrefix>,
		// How to convert an XCM Location into a local account id.
		// This is also something that's configured in the XCM executor.
		LocationConverter,
		// The type for account ids, only needed because `fungible` is generic over it.
		AccountId,
		// Not tracking teleports.
		// This recipe only uses reserve asset transfers to handle the Relay Chain token.
		(),
	>;

	parameter_types! {
		pub LocalPrefix: Location = Location::here();
		pub CheckingAccount: AccountId = PolkadotXcm::check_account();
	}

	/// Type that matches foreign assets.
	/// We do this by matching on all possible Locations and excluding the ones
	/// inside our local chain.
	pub type ForeignAssetsMatcher = MatchedConvertedConcreteId<
		xcm::v4::Location,                      // Asset id.
		Balance,                                // Balance type.
		EverythingBut<StartsWith<LocalPrefix>>, // Location matcher.
		JustTry,                                // How to convert from Location to AssetId.
		JustTry,                                // How to convert from u128 to Balance.
	>;

	/// AssetTransactor for handling other parachains' native tokens.
	pub type ForeignFungiblesTransactor = FungiblesAdapter<
		// Use this implementation of the `fungibles::*` traits.
		// `Balances` is the name given to the balances pallet in this particular example.
		ForeignAssets,
		// This transactor deals with the native token of sibling parachains.
		ForeignAssetsMatcher,
		// How we convert from a Location to an account id.
		LocationConverter,
		// The `AccountId` type.
		AccountId,
		// Not tracking teleports since we only use reserve asset transfers.
		NoChecking,
		// The account for checking.
		CheckingAccount,
	>;

	pub type AssetTransactor = (FungibleTransactor, ForeignFungiblesTransactor);
}
