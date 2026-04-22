use super::RelayLocation;

use frame_support::traits::{EverythingBut, Equals};
use xcm_builder::{AllowTopLevelPaidExecutionFrom, TakeWeightCredit};

pub type Barrier = (
    TakeWeightCredit,
    AllowTopLevelPaidExecutionFrom<EverythingBut<Equals<RelayLocation>>>,
);
