use super::locations::TokenLocation;
use frame_support::parameter_types;
use xcm::latest::prelude::*;
use xcm_builder::FixedWeightBounds;

parameter_types! {
    pub const BaseXcmWeight: Weight = Weight::from_parts(1_000, 1_000);
    pub TokensPerSecondPerByte: (AssetId, u128, u128) =
        (AssetId(TokenLocation::get()), 1_000_000_000_000, 1024 * 1024);
    pub const MaxInstructions: u32 = 100;
    pub const MaxAssetsIntoHolding: u32 = 64;
}

pub type Weigher<RuntimeCall> = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
