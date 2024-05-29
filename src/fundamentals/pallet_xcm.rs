//! # Fundamentals lesson 6: Pallet XCM
//!
//! Implement the core functionality of Pallet XCM

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
// TODO: Teach about Versioned Types
use crate::fundamentals::xcm_executor::ExecuteXcm;
pub use pallet::*;
use xcm::{prelude::*, VersionedAssets, VersionedLocation, VersionedXcm};

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		BadVersion,
		ExecutorError,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type XcmExecutor: ExecuteXcm<Self::RuntimeCall>;
		/// Required origin for executing XCM messages, including the teleport functionality. If
		/// successful, then it resolves to `Location` which exists as an interior location
		/// within this chain's XCM context.
		type ExecuteXcmOrigin: EnsureOrigin<
			<Self as frame_system::Config>::RuntimeOrigin,
			Success = Location,
		>;
	}
}

// TODO: Have students implement these functions (close as makes sense), that are also implemented
// in Pallet XCM. TODO: Potentially use XCM Executor implementation.
struct Sandbox<T: Config>(PhantomData<T>);
impl<T: Config> Sandbox<T> {
	fn execute(
		origin: OriginFor<T>,
		message: Box<VersionedXcm<T::RuntimeCall>>,
		_max_weight: Weight,
	) -> DispatchResult {
		let execute_origin = T::ExecuteXcmOrigin::ensure_origin(origin)?;
		let message = (*message).try_into().map_err(|()| Error::<T>::BadVersion)?;
		T::XcmExecutor::execute(execute_origin, message)
	}

	fn send(
		_origin: OriginFor<T>,
		_dest: Box<VersionedLocation>,
		_message: Box<VersionedXcm<()>>,
	) -> DispatchResult {
		unimplemented!();
	}

	fn teleport_asset(
		_origin: OriginFor<T>,
		_dest: Box<VersionedLocation>,
		_beneficiary: Box<VersionedLocation>,
		_assets: Box<VersionedAssets>,
		_fee_asset_item: u32,
	) -> DispatchResult {
		unimplemented!();
	}

	fn reserve_transfer_assets(
		_origin: OriginFor<T>,
		_dest: Box<VersionedLocation>,
		_beneficiary: Box<VersionedLocation>,
		_assets: Box<VersionedAssets>,
		_fee_asset_item: u32,
	) -> DispatchResult {
		unimplemented!();
	}
}
