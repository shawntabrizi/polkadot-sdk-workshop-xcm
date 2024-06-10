pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
	pub type TrustedReserves = ();
}

#[cfg(any(feature = "example", feature = "relay-token"))]
mod sandbox {
	use crate::parachain::teleporter::TrustedTeleporters;
	use frame_support::traits::EverythingBut;
	use xcm_builder::NativeAsset;

	pub type TrustedReserves = (NativeAsset, EverythingBut<TrustedTeleporters>);
}
