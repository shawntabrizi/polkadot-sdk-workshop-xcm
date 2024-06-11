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

use sp_std::{marker::PhantomData, prelude::*};
use xcm::latest::prelude::*;
use xcm_executor::traits::{ProcessTransaction, Properties, ShouldExecute, TransactAsset};

pub trait XcmConfig {
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
	/// Crete an initialize a new XCM Executor.
	pub fn new(origin: impl Into<Location>) -> Self {
		let origin = origin.into();
		let context =
			XcmContext { origin: Some(origin), message_id: Default::default(), topic: None };
		Self { holding: Default::default(), context, _config: PhantomData }
	}

	/// Process an entire XCM program, instruction by instruction.
	pub fn process(&mut self, xcm: Xcm<()>) -> Result<(), XcmError> {
		log::trace!(target: "xcm::process", "xcm: {:?}", xcm);

		for instruction in xcm.0.into_iter() {
			self.process_instruction(instruction)?;
		}
		Ok(())
	}

	/// Simple helper function to access the `origin` from the XCM Executor `context`.
	fn origin_ref(&self) -> Option<&Location> {
		self.context.origin.as_ref()
	}

	/// Process a single XCM instruction, mutating the state of the XCM virtual machine.
	fn process_instruction(&mut self, instr: Instruction<()>) -> Result<(), XcmError> {
		log::trace!(target: "xcm::process_instruction", "=== {:?}", instr);
		match instr {
			ClearOrigin => {
				self.context.origin = None;
				Ok(())
			},
			DescendOrigin(who) => self
				.context
				.origin
				.as_mut()
				.ok_or(XcmError::BadOrigin)?
				.append_with(who)
				.map_err(|_| XcmError::LocationFull),
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
			_ => unimplemented!(),
		}
	}
}

pub trait ExecuteXcm {
	fn execute(origin: impl Into<Location>, xcm: Xcm<()>) -> XcmResult;
}

impl<Config: XcmConfig> ExecuteXcm for XcmExecutor<Config> {
	fn execute(origin: impl Into<Location>, mut xcm: Xcm<()>) -> XcmResult {
		let origin = origin.into();
		log::trace!(target: "xcm::execute", "xcm: {:?}", xcm);
		let mut properties = Properties { weight_credit: Weight::default(), message_id: None };
		if let Err(e) = Config::Barrier::should_execute(
			&origin,
			xcm.inner_mut(),
			Weight::default(),
			&mut properties,
		) {
			log::trace!(target: "xcm::execute", "Barrier Error: {e:?}");
			return Err(XcmError::Barrier)
		};

		let mut vm = Self::new(origin);
		vm.process(xcm)
	}
}
