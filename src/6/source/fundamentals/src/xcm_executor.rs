//! # Fundamentals lesson 5: XCM Executor
//!
//! Create your own executor for XCM.

use super::holding::*;

use sp_std::{marker::PhantomData, prelude::*};
use xcm::latest::prelude::*;
use xcm_executor::traits::{ProcessTransaction, TransactAsset};

pub trait XcmConfig {
	/// How to withdraw and deposit an asset.
	type AssetTransactor: TransactAsset;
	/// Transactional processor for XCM instructions.
	type TransactionalProcessor: ProcessTransaction;
}

/// The heart of the XCM Virtual Machine.
pub struct XcmExecutor<Config: XcmConfig> {
	/// The asset holding registrar, where we keep track of assets being processed by the XCM
	/// Executor.
	pub holding: AssetsInHolding,
	/// Contextual data pertaining to a specific list of XCM instructions. Most relevant the
	/// `origin` of the XCM Message.
	pub context: XcmContext,
	/// Just a placeholder to allow Rust to let us keep `Config`.
	_config: PhantomData<Config>,
}

/// The implementation of the XCM Executor and how it processes XCM.
impl<Config: XcmConfig> XcmExecutor<Config> {
	/// Crete an initialize a new XCM Executor.
	pub fn new(origin: impl Into<Location>) -> Self {
		let origin = origin.into();
		// In our version of the XCM Executor, we ignore `message_id` and `topic`.
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
	pub fn origin_ref(&self) -> Option<&Location> {
		self.context.origin.as_ref()
	}

	/// Process a single XCM instruction, mutating the state of the XCM virtual machine.
	fn process_instruction(&mut self, instr: Instruction<()>) -> Result<(), XcmError> {
		log::trace!(target: "xcm::process_instruction", "=== {:?}", instr);
		match instr {
			// Clear the origin.
			//
			// This may be used by the XCM author to ensure that later instructions cannot command
			// the authority of the origin (e.g. if they are being relayed from an untrusted
			// source, as often the case with `ReserveAssetDeposited`).
			ClearOrigin => {
				todo!()
			},
			// Appends `who` to the current XCM Executor `origin` location.
			DescendOrigin(who) => {
				todo!("{:?}", who)
			},
			// Withdraw asset(s) (`assets`) from the ownership of `origin` and place equivalent
			// assets under the ownership of `beneficiary`.
			//
			// - `assets`: The asset(s) to be withdrawn.
			// - `beneficiary`: The new owner for the assets.
			TransferAsset { assets, beneficiary } => {
				todo!("{:?} {:?}", assets, beneficiary)
			},
			// Withdraw asset(s) (`assets`) from the ownership of `origin` and place them into the
			// Holding Register.
			//
			// - `assets`: The asset(s) to be withdrawn into holding.
			WithdrawAsset(assets) => {
				todo!("{:?}", assets)
			},
			// Reduce Holding by up to the given assets.
			//
			// Holding is reduced by as much as possible up to the assets in the parameter. It is
			// not an error if the Holding does not contain the assets (to make this an error, use
			// `ExpectAsset` prior).
			BurnAsset(assets) => {
				todo!("{:?}", assets)
			},
			// Remove the asset(s) (`assets`) from the Holding Register and place equivalent assets
			// under the ownership of `beneficiary` within this consensus system.
			//
			// - `assets`: The asset(s) to remove from holding.
			// - `beneficiary`: The new owner for the assets.
			DepositAsset { assets, beneficiary } => {
				todo!("{:?} {:?}", assets, beneficiary)
			},
			// Asset(s) (`assets`) have been destroyed on the `origin` system and equivalent assets
			// should be created and placed into the Holding Register.
			//
			// - `assets`: The asset(s) that are minted into the Holding Register.
			ReceiveTeleportedAsset(assets) => {
				todo!("{:?}", assets)
			},
			// In this workshop, we won't be implementing every instruction, just the ones above...
			// Our executor will simply panic if you try to execute other instructions.
			_ => unimplemented!(),
		}
	}
}

/// A public trait allowing other systems to access and use the `XcmExecutor`.
pub trait ExecuteXcm {
	/// Execute an XCM from a given `origin`.
	fn execute(origin: impl Into<Location>, xcm: Xcm<()>) -> XcmResult;
}

impl<Config: XcmConfig> ExecuteXcm for XcmExecutor<Config> {
	/// Execute an XCM from a given `origin`.
	fn execute(origin: impl Into<Location>, xcm: Xcm<()>) -> XcmResult {
		log::trace!(target: "xcm::execute", "xcm: {:?}", xcm);
		todo!("{:?}", origin.into())
	}
}
