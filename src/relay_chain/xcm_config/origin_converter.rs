use polkadot_parachain_primitives::primitives::Id as ParaId;
use polkadot_runtime_parachains::origin;
use xcm_builder::{
    ChildParachainAsNative, ChildSystemParachainAsSuperuser, SignedAccountId32AsNative,
    SovereignSignedViaLocation,
};

use super::locations::{LocationToAccountId, RelayNetwork};

pub type LocalOriginConverter<AccountId, RuntimeOrigin> = (
    SovereignSignedViaLocation<LocationToAccountId<AccountId>, RuntimeOrigin>,
    ChildParachainAsNative<origin::Origin, RuntimeOrigin>,
    SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
    ChildSystemParachainAsSuperuser<ParaId, RuntimeOrigin>,
);
