pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
	pub type AssetTransactor = ();
}

#[cfg(feature = "relay-token")]
mod sandbox {
	use crate::parachain::{
		constants::KsmLocation, location_converter::LocationConverter, AccountId, Balances,
	};
	use xcm_builder::{FungibleAdapter, IsConcrete};

	/// AssetTransactor for handling the relay chain token.
	/// In this case, we don't have a native token, we only use the relay chain token.
	// TODO: Finish type.
	pub type FungibleTransactor = FungibleAdapter<
		// What implementation of the `fungible::*` traits do we want to use?.
		(),
		// What tokens should be handled by this transactor?
		(),
		// How do we convert an XCM Location into a local account id?
		(),
		// The type for account ids, only needed because `fungible` is generic over it.
		AccountId,
		// Not tracking teleports.
		// This recipe only uses reserve asset transfers to handle the Relay Chain token.
		// This can actually be left as the unit type.
		(),
	>;

	pub type AssetTransactor = FungibleTransactor;
}

#[cfg(any(
	feature = "other-parachain-tokens",
	feature = "register-assets",
	feature = "asset-hub",
	feature = "barrier"
))]
mod sandbox {
	use frame_support::{parameter_types, traits::EverythingBut};
	use xcm::prelude::*;
	use xcm_builder::{
		FungibleAdapter, FungiblesAdapter, IsConcrete, MatchedConvertedConcreteId, MintLocation,
		NoChecking, StartsWith,
	};
	use xcm_executor::traits::JustTry;

	use crate::parachain::{
		location_converter::LocationConverter, AccountId, Balance, Balances, ForeignAssets,
		PolkadotXcm,
	};

	parameter_types! {
		pub LocalPrefix: Location = Location::here();
		pub CheckingAccount: AccountId = PolkadotXcm::check_account();
		pub LocalCheckAccount: (AccountId, MintLocation) = (CheckingAccount::get(), MintLocation::Local);
	}

	/// AssetTransactor for handling the chain's native token.
	// TODO: Finish type.
	pub type FungibleTransactor = FungibleAdapter<
		// What implementation of the `fungible::*` traits do we want to use?
		(),
		// What tokens should be handled by this transactor?
		(),
		// How do we convert an XCM Location into a local account id?
		(),
		// The type for account ids, only needed because `fungible` is generic over it.
		AccountId,
		// Tracking teleports.
		(),
	>;

	/// Type that matches foreign assets.
	/// We do this by matching on all possible Locations and excluding the ones
	/// inside our local chain.
	// TODO: Finish type.
	pub type ForeignAssetsMatcher = MatchedConvertedConcreteId<
		(), // Asset id.
		(), // Balance type.
		(), // Location matcher.
		(), // How to convert from Location to AssetId.
		(), // How to convert from u128 to Balance.
	>;

	/// AssetTransactor for handling other parachains' native tokens.
	// TODO: Finish type.
	pub type ForeignFungiblesTransactor = FungiblesAdapter<
		// What implementation of the `fungible::*` traits do we want to use?
		(),
		// What tokens should be handled by this transactor?
		(),
		// How we convert from a Location to an account id.
		(),
		// The `AccountId` type.
		AccountId,
		// Not tracking teleports since we only use reserve asset transfers for these types
		// of assets.
		(),
		// The account for checking.
		(),
	>;

	pub type AssetTransactor = (FungibleTransactor, ForeignFungiblesTransactor);
}
