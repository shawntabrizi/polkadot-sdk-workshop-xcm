use frame_support::parameter_types;
use xcm::prelude::*;
use xcm_builder::{IsConcrete, FungibleAdapter, FrameTransactionalProcessor};

// We use the custom xcm config trait.
use crate::xcm_executor::XcmConfig;
use super::AccountId;

mod location_converter;
pub use location_converter::LocationConverter;

// Generated from `decl_test_network!`
// TODO: EnsureDecodableXcm when available
pub type XcmRouter = super::super::RelayChainXcmRouter;

parameter_types! {
    pub const RelayNetwork: NetworkId = NetworkId::Kusama;
    pub const HereLocation: Location = Location::here();
    pub UniversalLocation: InteriorLocation = RelayNetwork::get().into();
}

type TestAssetTransactor = FungibleAdapter<super::Balances, IsConcrete<HereLocation>, LocationConverter, AccountId, ()>;

pub struct Config;
impl XcmConfig for Config {
    type RuntimeCall = super::RuntimeCall;
    type AssetTransactor = TestAssetTransactor;
    type TransactionalProcessor = FrameTransactionalProcessor;
    type Barrier = ();
}
