pub use sandbox::*;

use crate::parachain::RuntimeCall;
use frame_support::parameter_types;
use xcm::latest::prelude::*;
use xcm_builder::FixedWeightBounds;

parameter_types! {
    pub KsmPerSecondPerByte: (AssetId, u128, u128) = (AssetId(Parent.into()), 1, 1);
    pub const MaxAssetsIntoHolding: u32 = 64;
}

pub type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;

#[cfg(feature = "start")]
mod sandbox {
    use super::*;

    parameter_types! {
        pub const UnitWeightCost: Weight = Weight::zero();
        pub const MaxInstructions: u32 = 0;
    }
}

#[cfg(feature = "example")]
mod sandbox {
    use super::*;

    parameter_types! {
        pub const UnitWeightCost: Weight = Weight::from_parts(1, 1);
        pub const MaxInstructions: u32 = 100;
    }
}
