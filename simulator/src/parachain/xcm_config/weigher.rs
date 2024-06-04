pub use sandbox::*;

use crate::parachain::RuntimeCall;
use frame_support::parameter_types;
use xcm::latest::prelude::*;
use xcm_builder::FixedWeightBounds;

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

pub type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
