use frame_support::parameter_types;
use xcm::prelude::*;
use xcm_builder::{FrameTransactionalProcessor, FungibleAdapter, IsConcrete, SignedToAccountId32};

// We use the custom xcm config trait.
use super::AccountId;
use crate::xcm_executor::XcmConfig;

mod location_converter;
pub use location_converter::LocationConverter;

// Generated from `decl_test_network!`
// TODO: EnsureDecodableXcm when available
pub type XcmRouter = super::super::ParachainXcmRouter<super::MessageQueue>;

parameter_types! {
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub const ParentLocation: Location = Location::parent();
	pub UniversalLocation: InteriorLocation = [GlobalConsensus(RelayNetwork::get()), Parachain(super::MessageQueue::parachain_id().into())].into();
}

pub type LocalOriginToLocation = SignedToAccountId32<super::RuntimeOrigin, AccountId, RelayNetwork>;

type TestAssetTransactor =
	FungibleAdapter<super::Balances, IsConcrete<ParentLocation>, LocationConverter, AccountId, ()>;

pub struct Config;
impl XcmConfig for Config {
	type AssetTransactor = TestAssetTransactor;
	type TransactionalProcessor = FrameTransactionalProcessor;
}
