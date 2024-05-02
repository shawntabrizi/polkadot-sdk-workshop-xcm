use frame_support::dispatch::GetDispatchInfo;
use frame_support::dispatch::PostDispatchInfo;
use frame_support::Parameter;
use sp_runtime::traits::Dispatchable;
use sp_std::{
    collections::{btree_map::BTreeMap, btree_set::BTreeSet},
    marker::PhantomData,
    prelude::*,
};
use xcm::latest::prelude::*;
use xcm_executor::traits::ShouldExecute;
use xcm_executor::traits::TransactAsset;

/// Map of non-wildcard fungible and non-fungible assets held in the holding register.
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct AssetsInHolding {
    /// The fungible assets.
    pub fungible: BTreeMap<AssetId, u128>,

    /// The non-fungible assets.
    // TODO: Consider BTreeMap<AssetId, BTreeSet<AssetInstance>>
    //   or even BTreeMap<AssetId, SortedVec<AssetInstance>>
    pub non_fungible: BTreeSet<(AssetId, AssetInstance)>,
}

pub trait XcmConfig {
    type RuntimeCall: Parameter + Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo;

    /// How to withdraw and deposit an asset.
    type AssetTransactor: TransactAsset;

    /// Whether we should execute the given XCM at all.
    type Barrier: ShouldExecute;
}

pub struct XcmExecutor<Config: XcmConfig> {
    holding: AssetsInHolding,
    _config: PhantomData<Config>,
}

// TODO: Have students implement the logic for a few basic instructions.
impl<Config: XcmConfig> XcmExecutor<Config> {
    /// Process a single XCM instruction, mutating the state of the XCM virtual machine.
    fn process_instruction(
        &mut self,
        instr: Instruction<Config::RuntimeCall>,
    ) -> Result<(), XcmError> {
        log::trace!(
            target: "xcm::process_instruction",
            "=== {:?}",
            instr
        );
        match instr {
            _ => unimplemented!(),
        }
    }
}
