use frame_support::traits::EverythingBut;
use xcm_builder::NativeAsset;

pub type TrustedReserves = (
    NativeAsset,
    EverythingBut<super::teleporter::TrustedTeleporters>,
);
