use frame_support::parameter_types;
use xcm::latest::prelude::*;
use xcm_builder::FixedWeightBounds;

parameter_types! {
    pub const UnitWeightCost: Weight = Weight::from_parts(1, 1);
    pub KsmPerSecondPerByte: (AssetId, u128, u128) = (AssetId(Parent.into()), 1, 1);
    pub const MaxInstructions: u32 = 100;
    pub const MaxAssetsIntoHolding: u32 = 64;
}

pub type Weigher<RuntimeCall> = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
