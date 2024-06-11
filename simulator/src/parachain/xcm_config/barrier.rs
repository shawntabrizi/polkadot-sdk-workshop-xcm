pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
	pub type Barrier = ();
}

#[cfg(not(feature = "start"))]
mod sandbox {
	use frame_support::traits::Everything;
	use xcm_builder::AllowUnpaidExecutionFrom;

	pub type Barrier = AllowUnpaidExecutionFrom<Everything>;
}
