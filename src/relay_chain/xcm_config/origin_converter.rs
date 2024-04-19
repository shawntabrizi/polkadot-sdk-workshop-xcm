use polkadot_parachain_primitives::primitives::Id as ParaId;
use polkadot_runtime_parachains::origin;
use xcm_builder::{
    ChildParachainAsNative, ChildSystemParachainAsSuperuser, SignedAccountId32AsNative,
    SovereignSignedViaLocation,
};

use crate::relay_chain::locations::{LocationToAccountId, RelayNetwork};
use crate::relay_chain::RuntimeOrigin;

pub type LocalOriginConverter = (
    SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
    ChildParachainAsNative<origin::Origin, RuntimeOrigin>,
    SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
    ChildSystemParachainAsSuperuser<ParaId, RuntimeOrigin>,
);
