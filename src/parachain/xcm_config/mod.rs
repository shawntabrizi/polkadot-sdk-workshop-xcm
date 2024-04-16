pub mod asset_transactor;
pub mod barrier;
pub mod locations;

pub use asset_transactor::*;
pub use locations::*;

use frame_support::{
    parameter_types,
    traits::{Everything, EverythingBut, Nothing},
};
use xcm::latest::prelude::*;
use xcm_builder::{
    FixedRateOfFungible, FixedWeightBounds, FrameTransactionalProcessor, NativeAsset,
    SignedAccountId32AsNative, SovereignSignedViaLocation,
};

use pallet_xcm::XcmPassthrough;

// Stuff from our runtime.
use super::{
    AccountId, Balances, ForeignUniques, MsgQueue, PolkadotXcm, RuntimeCall, RuntimeOrigin,
};

pub type XcmRouter = crate::ParachainXcmRouter<super::MsgQueue>;

pub type XcmOriginToCallOrigin = (
    SovereignSignedViaLocation<locations::LocationToAccountId<AccountId>, RuntimeOrigin>,
    SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
    XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
    pub const UnitWeightCost: Weight = Weight::from_parts(1, 1);
    pub KsmPerSecondPerByte: (AssetId, u128, u128) = (AssetId(Parent.into()), 1, 1);
    pub const MaxInstructions: u32 = 100;
    pub const MaxAssetsIntoHolding: u32 = 64;
    pub ForeignPrefix: Location = (Parent,).into();
}

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
    type OriginConverter = XcmOriginToCallOrigin;
    type IsReserve = (NativeAsset, TrustedReserves);
    type IsTeleporter = TrustedTeleporters;
    type UniversalLocation = UniversalLocation;
    type Barrier = barrier::Barrier;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    type Trader = FixedRateOfFungible<KsmPerSecondPerByte, ()>;
    type ResponseHandler = ();
    type AssetTrap = ();
    type AssetLocker = PolkadotXcm;
    type AssetExchanger = ();
    type AssetClaims = ();
    type SubscriptionService = ();
    type PalletInstancesInfo = ();
    type FeeManager = ();
    type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
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
