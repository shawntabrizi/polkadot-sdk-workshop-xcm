#![allow(unused_imports)]

use frame_support::traits::Everything;
use xcm_builder::{AllowTopLevelPaidExecutionFrom, TakeWeightCredit};

pub type Barrier = (
    TakeWeightCredit,
    // TODO: Put the new necessary barrier here. Hint: use the imports :)
);
