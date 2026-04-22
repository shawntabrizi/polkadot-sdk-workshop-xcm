#![allow(unused_imports)]

use frame_support::traits::{Everything, ProcessMessageError};
use xcm::prelude::*;
use xcm_builder::{AllowTopLevelPaidExecutionFrom, TakeWeightCredit};
use xcm_executor::traits::{Properties, ShouldExecute};

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

pub type Barrier = (
	TakeWeightCredit,
	AllowAll, // TODO replace with correct barrier
);
