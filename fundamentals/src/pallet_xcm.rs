//! # Fundamentals lesson 6: Pallet XCM
//!
//! Implement the core functionality of Pallet XCM

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
// TODO: Teach about Versioned Types
use xcm::{prelude::*, VersionedAssets, VersionedLocation, VersionedXcm};

use crate::xcm_executor::ExecuteXcm;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		BadVersion,
		// TODO: Add XCM Error suberror
		ExecutorError,
		InvalidOrigin,
		RouterError,
		CannotReanchor,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
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

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::default())]
		pub fn execute(
			origin: OriginFor<T>,
			message: Box<VersionedXcm<()>>,
			_max_weight: Weight,
		) -> DispatchResult {
			let message = (*message).try_into().map_err(|()| Error::<T>::BadVersion)?;

			Self::do_execute(origin, message)
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::default())]
		pub fn send(
			origin: OriginFor<T>,
			dest: Box<VersionedLocation>,
			message: Box<VersionedXcm<()>>,
		) -> DispatchResult {
			let dest = Location::try_from(*dest).map_err(|()| Error::<T>::BadVersion)?;
			let message: Xcm<()> = (*message).try_into().map_err(|()| Error::<T>::BadVersion)?;

			Self::do_send(origin, dest, message)
		}

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

			Self::do_teleport_assets(origin, dest, beneficiary, assets, fee_asset_item)
		}

		#[pallet::call_index(3)]
		#[pallet::weight(Weight::default())]
		pub fn reserve_transfer_assets(
			origin: OriginFor<T>,
			dest: Box<VersionedLocation>,
			beneficiary: Box<VersionedLocation>,
			assets: Box<VersionedAssets>,
			fee_asset_item: u32,
		) -> DispatchResult {
			Self::do_reserve_transfer_assets(origin, dest, beneficiary, assets, fee_asset_item)
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn do_execute(origin: OriginFor<T>, message: Xcm<()>) -> DispatchResult {
		let execute_origin: Location = T::ExecuteXcmOrigin::ensure_origin(origin)?;
		T::XcmExecutor::execute(execute_origin, message).map_err(|_| Error::<T>::ExecutorError)?;
		Ok(())
	}

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
		_fee_asset_item: u32,
	) -> DispatchResult {
		let context = T::UniversalLocation::get();
		let mut reanchored_assets = assets.clone();
		reanchored_assets
			.reanchor(&dest, &context)
			.map_err(|_| Error::<T>::CannotReanchor)?;

		// XCM instructions to be executed on local chain
		let local_execute_xcm: Xcm<()> = Xcm(vec![
			// withdraw assets to be teleported
			WithdrawAsset(assets.clone()),
			// burn assets on local chain
			BurnAsset(assets),
		]);

		// XCM instructions to be executed on destination chain
		let xcm_on_dest: Xcm<()> = Xcm(vec![
			// teleport `assets` in from origin chain
			ReceiveTeleportedAsset(reanchored_assets),
			// following instructions are not exec'ed on behalf of origin chain anymore
			ClearOrigin,
			// deposit all remaining assets in holding to `beneficiary` location
			DepositAsset { assets: Wild(All), beneficiary },
		]);

		// Execute the local XCM instructions.
		Self::do_execute(origin.clone(), local_execute_xcm)?;
		// Send the destination XCM instructions.
		Self::do_send(origin, dest, xcm_on_dest)?;

		Ok(())
	}

	pub fn do_reserve_transfer_assets(
		_origin: OriginFor<T>,
		_dest: Box<VersionedLocation>,
		_beneficiary: Box<VersionedLocation>,
		_assets: Box<VersionedAssets>,
		_fee_asset_item: u32,
	) -> DispatchResult {
		unimplemented!()
	}
}
