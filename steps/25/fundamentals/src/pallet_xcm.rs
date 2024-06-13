//! # Fundamentals lesson 6: Pallet XCM
//!
//! Implement the core functionality of Pallet XCM

use crate::xcm_executor::ExecuteXcm;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use xcm::{prelude::*, VersionedAssets, VersionedLocation, VersionedXcm};

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Something to execute an XCM message.
		type XcmExecutor: ExecuteXcm;
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
		/// This chain's Universal Location.
		type UniversalLocation: Get<InteriorLocation>;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The version of the `Versioned` value used is not able to be interpreted.
		BadVersion,
		/// Origin is invalid for sending.
		InvalidOrigin,
		/// Could not re-anchor the assets to declare the fees for the destination chain.
		CannotReanchor,
		/// A general error indicating something went wrong with the XCM Executor.
		ExecutorError,
		/// A general error indicating something went wrong with the XCM Router.
		RouterError,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Execute an XCM from a local, signed, origin.
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::default())]
		pub fn execute(
			origin: OriginFor<T>,
			message: Box<VersionedXcm<()>>,
			_max_weight: Weight,
		) -> DispatchResult {
			let message = (*message).try_into().map_err(|()| Error::<T>::BadVersion)?;
			// Actually execute the XCM.
			Self::do_execute(origin, message)
		}

		/// Send an XCM to another consensus system.
		#[pallet::call_index(1)]
		#[pallet::weight(Weight::default())]
		pub fn send(
			origin: OriginFor<T>,
			dest: Box<VersionedLocation>,
			message: Box<VersionedXcm<()>>,
		) -> DispatchResult {
			let dest = Location::try_from(*dest).map_err(|()| Error::<T>::BadVersion)?;
			let message: Xcm<()> = (*message).try_into().map_err(|()| Error::<T>::BadVersion)?;
			// Actually send the XCM.
			Self::do_send(origin, dest, message)
		}

		/// Teleport some assets from the local chain to some destination chain.
		#[pallet::call_index(2)]
		#[pallet::weight(Weight::default())]
		pub fn teleport_assets(
			origin: OriginFor<T>,
			dest: Box<VersionedLocation>,
			beneficiary: Box<VersionedLocation>,
			assets: Box<VersionedAssets>,
			fee_asset_item: u32,
		) -> DispatchResult {
			let dest: Location = (*dest).try_into().map_err(|()| Error::<T>::BadVersion)?;
			let beneficiary: Location =
				(*beneficiary).try_into().map_err(|()| Error::<T>::BadVersion)?;
			let assets: Assets = (*assets).try_into().map_err(|()| Error::<T>::BadVersion)?;
			// Actually teleport the assets.
			Self::do_teleport_assets(origin, dest, beneficiary, assets, fee_asset_item)
		}

		/// Transfer some assets from the local chain to the destination chain through their local,
		/// destination or remote reserve.
		#[pallet::call_index(3)]
		#[pallet::weight(Weight::default())]
		pub fn reserve_transfer_assets(
			origin: OriginFor<T>,
			dest: Box<VersionedLocation>,
			beneficiary: Box<VersionedLocation>,
			assets: Box<VersionedAssets>,
			fee_asset_item: u32,
		) -> DispatchResult {
			let dest: Location = (*dest).try_into().map_err(|()| Error::<T>::BadVersion)?;
			let beneficiary: Location =
				(*beneficiary).try_into().map_err(|()| Error::<T>::BadVersion)?;
			let assets: Assets = (*assets).try_into().map_err(|()| Error::<T>::BadVersion)?;
			// Actually reserve transfer the assets.
			Self::do_reserve_transfer_assets(origin, dest, beneficiary, assets, fee_asset_item)
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Execute an XCM locally on this chain on behalf of `origin`.
	pub fn do_execute(origin: OriginFor<T>, message: Xcm<()>) -> DispatchResult {
		let execute_origin: Location = T::ExecuteXcmOrigin::ensure_origin(origin)?;
		T::XcmExecutor::execute(execute_origin, message).map_err(|_| Error::<T>::ExecutorError)?;
		Ok(())
	}

	/// Relay an XCM `message` from a given `interior` location in this context to a given `dest`
	/// location.
	pub fn do_send(origin: OriginFor<T>, dest: Location, mut message: Xcm<()>) -> DispatchResult {
		let origin_location = T::SendXcmOrigin::ensure_origin(origin)?;
		let interior: Junctions =
			origin_location.clone().try_into().map_err(|_| Error::<T>::InvalidOrigin)?;
		if interior != Junctions::Here {
			message.0.insert(0, DescendOrigin(interior.clone()));
		}
		let (ticket, _) = T::XcmRouter::validate(&mut Some(dest), &mut Some(message))
			.map_err(|_| Error::<T>::RouterError)?;
		let _message_id = T::XcmRouter::deliver(ticket).map_err(|_| Error::<T>::RouterError)?;
		Ok(())
	}

	pub fn do_teleport_assets(
		origin: OriginFor<T>,
		dest: Location,
		beneficiary: Location,
		assets: Assets,
		// The index into `assets` of the item which should be used to pay fees.
		// We don't use this in our naive implementation.
		_fee_asset_item: u32,
	) -> DispatchResult {
		todo!("{:?} {:?} {:?}", dest, beneficiary, assets)
	}

	pub fn do_reserve_transfer_assets(
		_origin: OriginFor<T>,
		_dest: Location,
		_beneficiary: Location,
		_assets: Assets,
		_fee_asset_item: u32,
	) -> DispatchResult {
		// There are 3 different reserve transfer scenarios:
		// - A local reserve transfer: reserve-transfer `asset` to `dest`, using local chain as
		//   reserve.
		// - A destination reserve transfer: reserve-transfer `asset` to `dest`, using `dest` as
		//   reserve.
		// - A remote reserve transfer: reserve-transfer `asset` to `dest`, using remote chain
		//   `Location` as reserve.
		//
		// This is a lot to do in this workshop, but a welcome challenge for the reader to
		// implement.
		unimplemented!()
	}
}
