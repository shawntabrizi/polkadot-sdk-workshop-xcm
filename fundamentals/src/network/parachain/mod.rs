//! Parachain runtime mock.

mod xcm_config;
pub use xcm_config::*;

use core::marker::PhantomData;
use frame_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{
		AsEnsureOriginWithArg, ConstU128, ContainsPair, EnsureOrigin, EnsureOriginWithArg,
		Everything, Nothing,
	},
	weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight},
};
use frame_system::{EnsureRoot, EnsureSigned};
use sp_core::ConstU32;
use sp_runtime::{
	traits::{Get, IdentityLookup},
	AccountId32,
};
use sp_std::prelude::*;
use xcm::latest::prelude::*;
use xcm_builder::{EnsureXcmOrigin, SignedToAccountId32};

use crate::xcm_executor::{XcmExecutor};
use crate::pallet_xcm::pallet as fundamentals_pallet_xcm;
use super::mock_message_queue;

pub type AccountId = AccountId32;
pub type Balance = u128;

type Block = frame_system::mocking::MockBlock<Runtime>;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Runtime {
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
}

impl mock_message_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<Config>;
}

impl fundamentals_pallet_xcm::Config for Runtime {
	type XcmExecutor = XcmExecutor<Config>;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type UniversalLocation = UniversalLocation;
}

construct_runtime! {
    pub struct Runtime {
        System: frame_system = 0,
        Balances: pallet_balances = 1,
        MessageQueue: mock_message_queue = 2,
        XcmPallet: fundamentals_pallet_xcm = 3,
    }
}
