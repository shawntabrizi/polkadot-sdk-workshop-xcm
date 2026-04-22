#![allow(unused_imports)]

use frame_support::traits::{Everything, ProcessMessageError};
use xcm::prelude::*;
use xcm_builder::{AllowTopLevelPaidExecutionFrom, TakeWeightCredit};
use xcm_executor::traits::{Properties, ShouldExecute};

/// A barrier that accepts anything. Wide open — replace with something real.
pub struct AllowAll;
impl ShouldExecute for AllowAll {
	fn should_execute<Call>(
		_origin: &Location,
		_instructions: &mut [Instruction<Call>],
		_max_weight: Weight,
		_properties: &mut Properties,
	) -> Result<(), ProcessMessageError> {
		Ok(())
	}
}

// A `Barrier` decides which incoming XCMs are allowed to start executing. `TakeWeightCredit`
// already accepts pre-paid messages; you need a second filter that accepts XCMs which pay
// for their own execution at the top level.
//
// TODO: Replace `AllowAll` with a real filter. `xcm-builder` ships one for this.
pub type Barrier = (
	TakeWeightCredit,
	AllowAll,
);
