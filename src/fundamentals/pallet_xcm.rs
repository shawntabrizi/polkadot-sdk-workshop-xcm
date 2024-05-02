use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
// TODO: Teach about Versioned Types
use xcm::{VersionedAssets, VersionedLocation, VersionedXcm};

// TODO: Have students implement these functions (close as makes sense), that are also implemented in Pallet XCM.
// TODO: Potentially use XCM Executor implementation.
struct Sandbox<T: pallet_xcm::Config>(PhantomData<T>);
impl<T: pallet_xcm::Config> Sandbox<T> {
    fn send(
        _origin: OriginFor<T>,
        _dest: Box<VersionedLocation>,
        _message: Box<VersionedXcm<()>>,
    ) -> DispatchResult {
        unimplemented!();
    }

    fn teleport_asset(
        _origin: OriginFor<T>,
        _dest: Box<VersionedLocation>,
        _beneficiary: Box<VersionedLocation>,
        _assets: Box<VersionedAssets>,
        _fee_asset_item: u32,
    ) -> DispatchResult {
        unimplemented!();
    }

    fn reserve_transfer_assets(
        _origin: OriginFor<T>,
        _dest: Box<VersionedLocation>,
        _beneficiary: Box<VersionedLocation>,
        _assets: Box<VersionedAssets>,
        _fee_asset_item: u32,
    ) -> DispatchResult {
        unimplemented!();
    }

    fn execute(
        _origin: OriginFor<T>,
        _message: Box<VersionedXcm<<T as pallet_xcm::Config>::RuntimeCall>>,
        _max_weight: Weight,
    ) -> DispatchResult {
        unimplemented!();
    }
}
