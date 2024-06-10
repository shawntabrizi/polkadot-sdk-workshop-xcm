pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
	pub type TrustedReserves = ();
}

#[cfg(any(feature = "example", feature = "relay-token", feature = "other-parachain-tokens"))]
mod sandbox {
	use xcm_builder::NativeAsset;

	/// We only trust other chains as reserves of their own asset.
	pub type TrustedReserves = NativeAsset;
}
