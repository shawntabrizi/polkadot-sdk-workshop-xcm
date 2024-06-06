use codec::{Decode, FullCodec, MaxEncodedLen};
use frame_support::{
	traits::{ProcessMessage, ProcessMessageError},
	LOG_TARGET,
};
use scale_info::TypeInfo;
use sp_std::{fmt::Debug, marker::PhantomData};
use sp_weights::WeightMeter;
use xcm::prelude::*;

use crate::xcm_executor::ExecuteXcm;

/// A message processor that delegates execution to an `XcmExecutor`.
pub struct ProcessXcmMessage<MessageOrigin, XcmExecutor, Call>(
	PhantomData<(MessageOrigin, XcmExecutor, Call)>,
);
impl<
		MessageOrigin: Into<Location> + FullCodec + MaxEncodedLen + Clone + Eq + PartialEq + TypeInfo + Debug,
		XcmExecutor: ExecuteXcm<Call>,
		Call,
	> ProcessMessage for ProcessXcmMessage<MessageOrigin, XcmExecutor, Call>
{
	type Origin = MessageOrigin;

	/// Process the given message, using no more than the remaining `weight` to do so.
	fn process_message(
		message: &[u8],
		origin: Self::Origin,
		_meter: &mut WeightMeter,
		_id: &mut XcmHash,
	) -> Result<bool, ProcessMessageError> {
		let versioned_message = VersionedXcm::<Call>::decode(&mut &message[..]).map_err(|e| {
			log::trace!(
				target: LOG_TARGET,
				"`VersionedXcm` failed to decode: {e:?}",
			);

			ProcessMessageError::Corrupt
		})?;
		let message = Xcm::<Call>::try_from(versioned_message).map_err(|_| {
			log::trace!(
				target: LOG_TARGET,
				"Failed to convert `VersionedXcm` into `XcmV3`.",
			);

			ProcessMessageError::Unsupported
		})?;

		let result = match XcmExecutor::execute(origin.into(), message) {
			Ok(()) => {
				log::trace!(
					target: LOG_TARGET,
					"XCM message execution complete",
				);
				Ok(true)
			},
			// In the error-case we assume the worst case and consume all possible weight.
			Err(error) => {
				log::trace!(
					target: LOG_TARGET,
					"XCM message execution error: {error:?}",
				);
				let error = match error {
					xcm::latest::Error::ExceedsStackLimit => ProcessMessageError::Unsupported, /* TODO: Use correct error. */
					_ => ProcessMessageError::Unsupported,
				};

				Err(error)
			},
		};
		result
	}
}
