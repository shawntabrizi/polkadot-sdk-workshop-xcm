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

use codec::{Encode, Decode};
use frame_support::{
	dispatch::{DispatchResult, GetDispatchInfo, PostDispatchInfo},
	Parameter,
};
use scale_info::TypeInfo;
use sp_runtime::traits::Dispatchable;
use sp_std::{marker::PhantomData, prelude::*};
use xcm::latest::prelude::*;
use xcm_executor::traits::{ProcessTransaction, Properties, ShouldExecute, TransactAsset};

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
	pub fn process(&mut self, xcm: Xcm<Config::RuntimeCall>) -> Result<(), XcmError> {
		log::trace!(target: "xcm::process", "xcm: {:?}", xcm);

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
		log::trace!(target: "xcm::process_instruction", "=== {:?}", instr);
		match instr {
			ClearOrigin => {
				self.context.origin = None;
				Ok(())
			},
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
			BurnAsset(assets) => {
				self.holding.saturating_take(assets.into());
				Ok(())
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
			ReceiveTeleportedAsset(assets) => {
				Config::TransactionalProcessor::process(|| {
					let origin = self.origin_ref().ok_or(XcmError::BadOrigin)?;
					// check whether we trust origin to teleport this asset to us via config trait.
					for asset in assets.inner() {
						// We should check that the asset can actually be teleported in (for this to
						// be in error, there would need to be an accounting violation by one of the
						// trusted chains, so it's unlikely, but we don't want to punish a possibly
						// innocent chain/user).
						println!("{:?}", asset);
						Config::AssetTransactor::can_check_in(origin, asset, &self.context)?;
						Config::AssetTransactor::check_in(origin, asset, &self.context);
					}
					Ok(())
				})
				.and_then(|_| {
					self.holding.subsume_assets(assets.into());
					Ok(())
				})
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
			DescendOrigin(who) => self
				.context
				.origin
				.as_mut()
				.ok_or(XcmError::BadOrigin)?
				.append_with(who)
				.map_err(|_| XcmError::LocationFull),
			_ => unimplemented!(),
		}
	}
}

pub trait ExecuteXcm<RuntimeCall> {
	fn execute(origin: impl Into<Location>, xcm: Xcm<RuntimeCall>) -> Outcome;
}

/// Outcome of an XCM execution.
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
pub enum Outcome {
	/// Execution completed successfully; given weight was used.
	Complete { used: Weight },
	/// Execution started, but did not complete successfully due to the given error; given weight
	/// was used.
	Incomplete { used: Weight, error: XcmError },
	/// Execution did not start due to the given error.
	Error { error: XcmError },
}

impl Outcome {
	pub fn ensure_complete(self) -> Result<(), XcmError> {
		match self {
			Outcome::Complete { .. } => Ok(()),
			Outcome::Incomplete { error, .. } => Err(error),
			Outcome::Error { error, .. } => Err(error),
		}
	}
	pub fn ensure_execution(self) -> Result<Weight, XcmError> {
		match self {
			Outcome::Complete { used, .. } => Ok(used),
			Outcome::Incomplete { used, .. } => Ok(used),
			Outcome::Error { error, .. } => Err(error),
		}
	}
	/// How much weight was used by the XCM execution attempt.
	pub fn weight_used(&self) -> Weight {
		match self {
			Outcome::Complete { used, .. } => *used,
			Outcome::Incomplete { used, .. } => *used,
			Outcome::Error { .. } => Weight::zero(),
		}
	}
}

impl<Config: XcmConfig> ExecuteXcm<Config::RuntimeCall> for XcmExecutor<Config> {
	fn execute(origin: impl Into<Location>, mut xcm: Xcm<Config::RuntimeCall>) -> Outcome {
		let origin = origin.into();
		log::trace!(target: "xcm::execute", "xcm: {:?}", xcm);
		let mut properties = Properties { weight_credit: Weight::default(), message_id: None };
		if let Err(error) = Config::Barrier::should_execute(
			&origin,
			xcm.inner_mut(),
			Weight::default(),
			&mut properties,
		) {
			return Outcome::Error { error: XcmError::Barrier }
		};

		let mut vm = Self::new(origin);
		match vm.process(xcm) {
			Err(error) => {
				log::trace!(target: "xcm::execute", "xcm_executor error: {:?}", error);
				Outcome::Error { error }
			},
			Ok(()) => {
				Outcome::Complete { used: Weight::zero() }
			}
		}
	}
}
