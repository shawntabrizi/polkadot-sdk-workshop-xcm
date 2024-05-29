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
		InvalidOrigin,
		RouterError,
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

		/// Required origin for sending XCM messages. If successful, it resolves to `Location`
		/// which exists as an interior location within this chain's XCM context.
		type SendXcmOrigin: EnsureOrigin<
			<Self as frame_system::Config>::RuntimeOrigin,
			Success = Location,
		>;
		/// The type used to actually dispatch an XCM to its destination.
		type XcmRouter: SendXcm;
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::default())]
		pub fn execute(
			origin: OriginFor<T>,
			message: Box<VersionedXcm<T::RuntimeCall>>,
			_max_weight: Weight,
		) -> DispatchResult {
			let execute_origin = T::ExecuteXcmOrigin::ensure_origin(origin)?;
			let message = (*message).try_into().map_err(|()| Error::<T>::BadVersion)?;
			T::XcmExecutor::execute(execute_origin, message)
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::default())]
		pub fn send(
			origin: OriginFor<T>,
			dest: Box<VersionedLocation>,
			message: Box<VersionedXcm<()>>,
		) -> DispatchResult {
			let origin_location = T::SendXcmOrigin::ensure_origin(origin)?;
			let interior: Junctions =
				origin_location.clone().try_into().map_err(|_| Error::<T>::InvalidOrigin)?;
			let dest = Location::try_from(*dest).map_err(|()| Error::<T>::BadVersion)?;
			let mut message: Xcm<()> =
				(*message).try_into().map_err(|()| Error::<T>::BadVersion)?;
			if interior != Junctions::Here {
				message.0.insert(0, DescendOrigin(interior.clone()));
			}
			let (ticket, _) = T::XcmRouter::validate(&mut Some(dest), &mut Some(message))
				.map_err(|_| Error::<T>::RouterError)?;
			let _message_id = T::XcmRouter::deliver(ticket).map_err(|_| Error::<T>::RouterError)?;
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(Weight::default())]
		pub fn teleport_asset(
			_origin: OriginFor<T>,
			_dest: Box<VersionedLocation>,
			_beneficiary: Box<VersionedLocation>,
			_assets: Box<VersionedAssets>,
			_fee_asset_item: u32,
		) -> DispatchResult {
			unimplemented!();
		}

		#[pallet::call_index(3)]
		#[pallet::weight(Weight::default())]
		pub fn reserve_transfer_assets(
			_origin: OriginFor<T>,
			_dest: Box<VersionedLocation>,
			_beneficiary: Box<VersionedLocation>,
			_assets: Box<VersionedAssets>,
			_fee_asset_item: u32,
		) -> DispatchResult {
			unimplemented!();
		}
	}
}
