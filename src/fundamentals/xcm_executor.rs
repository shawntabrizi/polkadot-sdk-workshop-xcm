//! # Fundamentals lesson 5: XCM Executor
//!
//! Create your own executor for XCM.

// TODO: Key Takeaways
// - Match statement to process instructions
// - executor state (origin, holding, etc...)
// - Assets in Holding, and all its behaviors
// - How XCM Config can change logic in the executor
// - Transactional
// - composability of instructions

use super::holding::*;

use frame_support::{
	dispatch::{GetDispatchInfo, PostDispatchInfo},
	Parameter,
};
use sp_runtime::traits::Dispatchable;
use sp_std::{marker::PhantomData, prelude::*};
use xcm::latest::prelude::*;
use xcm_executor::traits::{ProcessTransaction, ShouldExecute, TransactAsset};

pub trait XcmConfig {
	type RuntimeCall: Parameter + Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo;

	/// How to withdraw and deposit an asset.
	type AssetTransactor: TransactAsset;

	/// Whether we should execute the given XCM at all.
	type Barrier: ShouldExecute;

	/// Transactional processor for XCM instructions.
	type TransactionalProcessor: ProcessTransaction;
}

pub struct XcmExecutor<Config: XcmConfig> {
	pub holding: AssetsInHolding,
	pub context: XcmContext,
	_config: PhantomData<Config>,
}

// TODO: Have students implement the logic for a few basic instructions.
impl<Config: XcmConfig> XcmExecutor<Config> {
	pub fn new(origin: impl Into<Location>) -> Self {
		let origin = origin.into();
		let context =
			XcmContext { origin: Some(origin), message_id: Default::default(), topic: None };
		Self { holding: Default::default(), context, _config: PhantomData }
	}

	fn origin_ref(&self) -> Option<&Location> {
		self.context.origin.as_ref()
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
				let origin = self.origin_ref().ok_or(XcmError::BadOrigin)?;
				Config::TransactionalProcessor::process(|| {
					// Take `assets` from the origin account (on-chain)...
					for asset in assets.inner() {
						Config::AssetTransactor::withdraw_asset(
							asset,
							origin,
							Some(&self.context),
						)?;
					}
					Ok(())
				})
				.and_then(|_| {
					// ...and place into holding.
					self.holding.subsume_assets(assets.into());
					Ok(())
				})
			},
			DepositAsset { assets, beneficiary } => {
				let old_holding = self.holding.clone();
				let result = Config::TransactionalProcessor::process(|| {
					let deposited = self.holding.saturating_take(assets);
					for asset in deposited.into_assets_iter() {
						Config::AssetTransactor::deposit_asset(
							&asset,
							&beneficiary,
							Some(&self.context),
						)?;
					}
					Ok(())
				});
				if Config::TransactionalProcessor::IS_TRANSACTIONAL && result.is_err() {
					self.holding = old_holding;
				}
				result
			},
			TransferAsset { assets, beneficiary } => {
				Config::TransactionalProcessor::process(|| {
					// Take `assets` from the origin account (on-chain) and place into dest account.
					let origin = self.origin_ref().ok_or(XcmError::BadOrigin)?;
					for asset in assets.inner() {
						Config::AssetTransactor::transfer_asset(
							&asset,
							origin,
							&beneficiary,
							&self.context,
						)?;
					}
					Ok(())
				})
			},
			ClearOrigin => {
				self.context.origin = None;
				Ok(())
			},
			_ => unimplemented!(),
		}
	}
}
