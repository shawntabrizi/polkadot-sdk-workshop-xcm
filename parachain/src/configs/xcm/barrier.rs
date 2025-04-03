#![allow(unused_imports)]

use super::RelayLocation;

use frame_support::traits::{EverythingBut, Equals};
use xcm_builder::{AllowTopLevelPaidExecutionFrom, TakeWeightCredit};

pub type Barrier = (
    TakeWeightCredit,
    // TODO: Put the new necessary barrier here. Hint: use the imports :)
);
