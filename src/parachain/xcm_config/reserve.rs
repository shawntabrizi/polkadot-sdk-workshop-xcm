pub use workspace::*;

#[cfg(feature = "start")]
mod workspace {
    pub type TrustedReserves = ();
}

#[cfg(feature = "example")]
mod workspace {
    use frame_support::traits::EverythingBut;
    use xcm_builder::NativeAsset;

    pub type TrustedReserves = (
        NativeAsset,
        EverythingBut<super::teleporter::TrustedTeleporters>,
    );
}
