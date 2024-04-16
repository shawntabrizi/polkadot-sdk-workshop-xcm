pub mod asset_transactor;
pub mod barrier;
pub mod limits;
pub mod locations;
pub mod origin_converter;

pub use asset_transactor::*;
pub use limits::*;
pub use locations::*;

use frame_support::{
    parameter_types,
    traits::{Everything, EverythingBut, Nothing},
};
use xcm::latest::prelude::*;
use xcm_builder::{FixedRateOfFungible, FrameTransactionalProcessor, NativeAsset};

// Stuff from our runtime.
use super::{
    AccountId, Balances, ForeignUniques, MsgQueue, PolkadotXcm, RuntimeCall, RuntimeOrigin,
};

pub type XcmRouter = crate::ParachainXcmRouter<super::MsgQueue>;

parameter_types! {
    pub NftCollectionOne: AssetFilter
        = Wild(AllOf { fun: WildNonFungible, id: AssetId((Parent, GeneralIndex(1)).into()) });
    pub NftCollectionOneForRelay: (AssetFilter, Location)
        = (NftCollectionOne::get(), (Parent,).into());
}
pub type TrustedTeleporters = xcm_builder::Case<NftCollectionOneForRelay>;
pub type TrustedReserves = EverythingBut<xcm_builder::Case<NftCollectionOneForRelay>>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    type AssetTransactor = asset_transactor::LocalAssetTransactor<AccountId, Balances>;
    type OriginConverter = origin_converter::XcmOriginToCallOrigin<AccountId, RuntimeOrigin>;
    type IsReserve = (NativeAsset, TrustedReserves);
    type IsTeleporter = TrustedTeleporters;
    type UniversalLocation = UniversalLocation;
    type Barrier = barrier::Barrier;
    type Weigher = limits::Weigher<RuntimeCall>;
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
