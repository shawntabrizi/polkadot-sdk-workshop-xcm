pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
    pub type TrustedReserves = ();
}

#[cfg(feature = "example")]
mod sandbox {
    use frame_support::traits::EverythingBut;
    use xcm_builder::NativeAsset;

    pub type TrustedReserves = (
        NativeAsset,
        EverythingBut<super::super::teleporter::TrustedTeleporters>,
    );
}
