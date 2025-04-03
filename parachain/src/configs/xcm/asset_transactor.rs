use crate::{AccountId, Balance, Balances, ForeignAssets};
use super::{LocationToAccountId, HereLocation, CheckingAccount};

use frame_support::traits::EverythingBut;
use xcm::prelude::*;
use xcm_builder::{IsConcrete, NoChecking, FungibleAdapter, FungiblesAdapter, StartsWith, MatchedConvertedConcreteId};
use xcm_executor::traits::JustTry;

/// Means for transacting assets on this chain.
pub type LocalFungibleTransactor = FungibleAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<HereLocation>,
	// Do a simple punn to convert an AccountId32 Location into a native chain account ID:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports.
	(),
>;

pub type ForeignFungiblesTransactor = FungiblesAdapter<
	// Use this fungibles impl.
	ForeignAssets,
	// Match all locations except `Here`.
	MatchedConvertedConcreteId<
		Location,
		Balance,
		EverythingBut<StartsWith<HereLocation>>,
		JustTry,
		JustTry,
	>,
	// Location converter.
	LocationToAccountId,
	// Needed for satisfying trait bounds.
	AccountId,
	// Not tracking teleports.
	NoChecking,
	// Still have to specify a checking account...
	CheckingAccount,
>;

pub type AssetTransactor = (LocalFungibleTransactor, ForeignFungiblesTransactor);
