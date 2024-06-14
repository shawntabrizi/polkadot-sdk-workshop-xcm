pub use sandbox::*;

#[cfg(not(feature = "barrier"))]
mod sandbox {
	use frame_support::traits::Everything;
	use xcm_builder::AllowUnpaidExecutionFrom;

	pub type Barrier = AllowUnpaidExecutionFrom<Everything>;
}

#[cfg(feature = "barrier")]
mod sandbox {
	use frame_support::traits::{Contains, Everything};
	use xcm::prelude::*;
	use xcm_builder::{AllowExplicitUnpaidExecutionFrom, AllowTopLevelPaidExecutionFrom};

	// We want to allow paid messages from anyone, explicitly unpaid only for relay.
	// TODO: Finish type.
	pub type Barrier = ();
}
