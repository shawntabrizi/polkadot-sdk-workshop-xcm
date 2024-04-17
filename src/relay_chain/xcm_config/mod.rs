pub mod asset_transactor;
pub mod barrier;
pub mod limits;
pub mod locations;
pub mod origin_converter;

pub use locations::*;

use frame_support::traits::{Everything, Nothing};

use xcm_builder::{FixedRateOfFungible, FrameTransactionalProcessor};
use xcm_executor::Config;

// Types from our runtime.
use super::{AccountId, Balances, RuntimeCall, RuntimeOrigin};
// Pallets from our runtime.
use super::{Uniques, XcmPallet};

// Generated from `decl_test_network!`
pub type XcmRouter = crate::RelayChainXcmRouter;

pub struct XcmConfig;
impl Config for XcmConfig {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    type AssetTransactor = asset_transactor::LocalAssetTransactor<AccountId>;
    type OriginConverter = origin_converter::LocalOriginConverter<AccountId, RuntimeOrigin>;
    type IsReserve = ();
    type IsTeleporter = ();
    type UniversalLocation = UniversalLocation;
    type Barrier = barrier::Barrier;
    type Weigher = limits::Weigher<RuntimeCall>;
    type Trader = FixedRateOfFungible<limits::TokensPerSecondPerByte, ()>;
    type ResponseHandler = ();
    type AssetTrap = ();
    type AssetLocker = XcmPallet;
    type AssetExchanger = ();
    type AssetClaims = ();
    type SubscriptionService = ();
    type PalletInstancesInfo = ();
    type FeeManager = ();
    type MaxAssetsIntoHolding = limits::MaxAssetsIntoHolding;
    type MessageExporter = ();
    type UniversalAliases = Nothing;
    type CallDispatcher = RuntimeCall;
    type SafeCallFilter = Everything;
    type Aliasers = Nothing;
    type TransactionalProcessor = FrameTransactionalProcessor;
    type HrmpNewChannelOpenRequestHandler = ();
    type HrmpChannelAcceptedHandler = ();
    type HrmpChannelClosingHandler = ();
}
