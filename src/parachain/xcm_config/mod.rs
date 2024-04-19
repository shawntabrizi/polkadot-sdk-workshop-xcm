pub mod asset_transactor;
pub mod barrier;
pub mod limits;
pub mod locations;
pub mod origin_converter;
pub mod reserve;
pub mod teleporter;

pub use asset_transactor::*;
pub use limits::*;
pub use locations::*;

use frame_support::traits::{Everything, Nothing};
use xcm_builder::{FixedRateOfFungible, FrameTransactionalProcessor};

// Types from our runtime.
use crate::parachain::{MsgQueue, PolkadotXcm, RuntimeCall};

// Generated from `decl_test_network!`
pub type XcmRouter = crate::ParachainXcmRouter<MsgQueue>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    type AssetTransactor = asset_transactor::AssetTransactor;
    type OriginConverter = origin_converter::OriginConverter;
    type IsReserve = reserve::TrustedReserves;
    type IsTeleporter = teleporter::TrustedTeleporters;
    type UniversalLocation = UniversalLocation;
    type Barrier = barrier::Barrier;
    type Weigher = limits::Weigher;
    type Trader = FixedRateOfFungible<limits::KsmPerSecondPerByte, ()>;
    type ResponseHandler = ();
    type AssetTrap = ();
    type AssetLocker = PolkadotXcm;
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
