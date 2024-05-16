//! # Fundamentals lesson 4: XCM Executor
//!
//! Create your own executor for XCM.

use frame_support::dispatch::{GetDispatchInfo, PostDispatchInfo};
use frame_support::Parameter;
use sp_runtime::traits::Dispatchable;
use sp_std::{
    collections::{btree_map::BTreeMap, btree_set::BTreeSet},
    marker::PhantomData,
    prelude::*,
};
use xcm::latest::prelude::*;
use xcm_executor::traits::{TransactAsset, ShouldExecute};

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
    pub holding: AssetsInHolding,
    pub origin: Option<Location>,
    _config: PhantomData<Config>,
}

// TODO: Have students implement the logic for a few basic instructions.
impl<Config: XcmConfig> XcmExecutor<Config> {
    pub fn new(origin: Location) -> Self {
        Self {
            holding: Default::default(),
            origin: Some(origin),
            _config: PhantomData,
        }
    }

    /// Process an entire XCM program.
    pub fn execute(&mut self, xcm: Xcm<Config::RuntimeCall>) -> Result<(), XcmError> {
        log::trace!(
            target: "xcm::execute",
            "xcm: {:?}",
            xcm
        );
        for instruction in xcm.0.into_iter() {
            self.process_instruction(instruction)?;
        }
        Ok(())
    }

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
            WithdrawAsset(assets) => {
                let origin = self.origin.as_ref().ok_or(XcmError::BadOrigin)?;
                for asset in assets.inner() {
                    Config::AssetTransactor::withdraw_asset(asset, origin, None)?;
                    match asset.fun {
                        Fungibility::Fungible(amount) => {
                            self.holding.fungible.insert(asset.id.clone(), amount);
                        },
                        Fungibility::NonFungible(instance) => {
                            self.holding.non_fungible.insert((asset.id.clone(), instance));
                        },
                    }
                }
                Ok(())
            }
            _ => todo!(),
        }
    }
}
