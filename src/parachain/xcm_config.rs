use frame_support::{
    parameter_types,
    traits::{Everything, EverythingBut, Nothing},
};
use polkadot_parachain_primitives::primitives::Sibling;
use xcm::latest::prelude::*;
use xcm_builder::{
    Account32Hash, AccountId32Aliases, AllowUnpaidExecutionFrom, ConvertedConcreteId,
    FixedRateOfFungible, FixedWeightBounds, FrameTransactionalProcessor, FungibleAdapter,
    IsConcrete, NativeAsset, NoChecking, NonFungiblesAdapter, ParentIsPreset,
    SiblingParachainConvertsVia, SignedAccountId32AsNative, SovereignSignedViaLocation,
};
use xcm_executor::traits::JustTry;

use pallet_xcm::XcmPassthrough;

// Stuff from our runtime.
use super::{
    AccountId, Balances, ForeignUniques, MsgQueue, PolkadotXcm, RuntimeCall, RuntimeOrigin,
};

parameter_types! {
    pub const KsmLocation: Location = Location::parent();
    pub const RelayNetwork: NetworkId = NetworkId::Kusama;
    pub UniversalLocation: InteriorLocation = Parachain(MsgQueue::parachain_id().into()).into();
}

pub type XcmRouter = crate::ParachainXcmRouter<super::MsgQueue>;

pub type SovereignAccountOf = (
    SiblingParachainConvertsVia<Sibling, AccountId>,
    AccountId32Aliases<RelayNetwork, AccountId>,
    ParentIsPreset<AccountId>,
);

pub type LocationToAccountId = (
    ParentIsPreset<AccountId>,
    SiblingParachainConvertsVia<Sibling, AccountId>,
    AccountId32Aliases<RelayNetwork, AccountId>,
    Account32Hash<(), AccountId>,
);

pub type XcmOriginToCallOrigin = (
    SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
    SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
    XcmPassthrough<RuntimeOrigin>,
);

pub type LocalAssetTransactor = (
    FungibleAdapter<Balances, IsConcrete<KsmLocation>, LocationToAccountId, AccountId, ()>,
    NonFungiblesAdapter<
        ForeignUniques,
        ConvertedConcreteId<Location, AssetInstance, JustTry, JustTry>,
        SovereignAccountOf,
        AccountId,
        NoChecking,
        (),
    >,
);

parameter_types! {
    pub const UnitWeightCost: Weight = Weight::from_parts(1, 1);
    pub KsmPerSecondPerByte: (AssetId, u128, u128) = (AssetId(Parent.into()), 1, 1);
    pub const MaxInstructions: u32 = 100;
    pub const MaxAssetsIntoHolding: u32 = 64;
    pub ForeignPrefix: Location = (Parent,).into();
}

pub type Barrier = AllowUnpaidExecutionFrom<Everything>;

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
    type AssetTransactor = LocalAssetTransactor;
    type OriginConverter = XcmOriginToCallOrigin;
    type IsReserve = (NativeAsset, TrustedReserves);
    type IsTeleporter = TrustedTeleporters;
    type UniversalLocation = UniversalLocation;
    type Barrier = Barrier;
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
