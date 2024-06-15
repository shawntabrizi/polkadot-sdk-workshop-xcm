pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
	pub type TrustedTeleporters = ();
}

#[cfg(not(feature = "start"))]
mod sandbox {
	use frame_support::{parameter_types, traits::ContainsPair};
	use sp_runtime::traits::Get;
	use xcm::latest::prelude::*;
	use xcm_simulator::ParaId;

	/// Checks whether asset matches `IsForeign`.
	pub struct IsForeignConcreteAsset<IsForeign>(sp_std::marker::PhantomData<IsForeign>);
	impl<IsForeign: ContainsPair<Location, Location>> ContainsPair<Asset, Location>
		for IsForeignConcreteAsset<IsForeign>
	{
		fn contains(asset: &Asset, origin: &Location) -> bool {
			todo!()
		}
	}

	/// Checks whether a location is from a sibling parachain.
	pub struct FromSiblingParachain<SelfParaId>(sp_std::marker::PhantomData<SelfParaId>);
	impl<SelfParaId: Get<ParaId>> ContainsPair<Location, Location>
		for FromSiblingParachain<SelfParaId>
	{
		fn contains(a: &Location, b: &Location) -> bool {
			todo!()
		}
	}

	parameter_types! {
		pub SelfParaId: ParaId = 1000.into();
	}

	// We want to trust siblings as teleporters of their own native token.
	pub type TrustedTeleporters = ();
}
