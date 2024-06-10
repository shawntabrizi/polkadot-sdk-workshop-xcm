pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
	pub type Barrier = ();
}

#[cfg(any(feature = "example", feature = "relay-token"))]
mod sandbox {
	use frame_support::traits::Everything;
	use xcm_builder::AllowUnpaidExecutionFrom;

	pub type Barrier = AllowUnpaidExecutionFrom<Everything>;
}
