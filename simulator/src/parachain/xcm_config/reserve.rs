pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
	pub type TrustedReserves = ();
}

#[cfg(not(feature = "start"))]
mod sandbox {
	use xcm_builder::NativeAsset;

	/// We only trust other chains as reserves of their own asset.
	pub type TrustedReserves = NativeAsset;
}
