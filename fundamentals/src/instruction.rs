//! # Fundamentals Lesson 4: Instructions
//! 
//! We are familiar at this point that Cross-Consensus Messages are used to allow different consensus systems to communicate and interoperate with one another.
//! 
//! So now let's take a look at how those XCM messages are composed.
//! 
//! ## Composition of an XCM
//! 
//! Let's start by looking at the definition of an XCM:
//! 
//! ```rust
//! pub struct Xcm<Call>(pub Vec<Instruction<Call>>);
//! ```
//! 
//! So an XCM is just a wrapper for a vector of `Instructions`. It is generic over `Call` which is the callable functions exposed by your consensus system. For example, on a Substrate chain, these would be the Pallet Calls.
//! 
//! We will take a high level look at this generic `Call` type and how it is used later.
//! 
//! ## XCM Instructions
//! 
//! So XCM messages are just a vector of instructions... but what is an `Instruction`.
//! 
//! At the time of writing, there are nearly 50 different instructions exposed by the XCM format.
//! 
//! At a high level, instructions are specific actions used to manipulate either or both of the underlying blockchain state and the XCM in-memory state.
//! 
//! Let's take a look at just a few examples of instructions to get an idea for what they do:
//! 
//! - `WithdrawAsset`: Moves assets from the ownership of `origin` to the holding registrar.
//! - `DepositAsset`: Moves assets from the holding registrar to a `beneficiary`.
//! - `BurnAsset`: Reduce assets in the holding by up to some amount.
//! - `ClearOrigin`: An instruction to reset the state of `origin` in the XCM Executor state.
//! - `ReceiveTeleportedAsset`: Add assets that have been destroyed on the `origin` system into the current system's holding registrar.
//! - `TransferAsset`: Withdraw assets from the ownership of `origin` and deposit them into the ownership of a `beneficiary`.
//! 
//! To see the full list of available instructions, check ou the [XCM Format repo](https://github.com/paritytech/xcm-format).
//! 
//! ### RFCs
//! 
//! You might be interested to know how new instructions get added to XCM. For this we have the [XCM Format repo](https://github.com/paritytech/xcm-format) and an [RFC process](https://github.com/paritytech/xcm-format/blob/master/proposals/0000-template.md).
//! 
//! We won't go into details about this in this guide, but feel free to browse around there.
//! 
//! ## Composition
//! 
//! Composing a new XCM from `Instruction`s is really simple. Let's look for a single instruction:
//! 
//! ```rust
//! use xcm::latest::prelude::*;
//! 
//! let my_message: Xcm<()> = Xcm(vec![ClearOrigin]);
//! ```
//! 
//! And for multiple instructions, you just increase add more items to the vector:
//! 
//! ```rust
//! use xcm::latest::prelude::*;
//! let assets: Assets = (Parent, 100u128).into();
//! 
//! let message = Xcm(vec![
//! 	WithdrawAsset(assets.clone()),
//! 	BurnAsset(assets),
//! ]);
//! ```
//! 
//! While you can really put whatever instructions you want into a message, not every combination of instructions will be sensible or even valid. The XCM Executor will return an error when the instruction set does not make sense.
//! 
//! ### Builder Pattern

use xcm::latest::prelude::*;

use crate::constants::ALICE;

/// A message containing only a simple `ClearOrigin` instruction.
/// This instruction clears the origin of the sender, meaning after this point,
/// no special privileges are granted.
pub fn clear_origin_message() -> Xcm<()> {
	let message = Xcm(vec![ClearOrigin]);

	message
}

/// Put all your knowledge of assets to the test.
/// Use the `WithdrawAsset` instruction to withdraw 100 planks
/// of the relay native token, i.e. DOT.
/// The XCM program is executed on a parachain.
/// This program won't do anything with the funds.
pub fn withdraw_asset() -> Xcm<()> {
	let assets: Assets = (Parent, 100u128).into();
	let message = Xcm(vec![WithdrawAsset(assets)]);

	message
}

/// Let's do something with the funds.
/// This time, incorporate the `DepositAsset` instruction right after
/// withdrawing the assets.
/// Use the same assets.
/// Deposit all the assets to `ALICE`.
/// Remember how to use wildcards.
pub fn withdraw_and_deposit() -> Xcm<()> {
	let assets: Assets = (Parent, 100u128).into();
	let message = Xcm(vec![
		WithdrawAsset(assets),
		DepositAsset {
			assets: All.into(),
			beneficiary: AccountId32 { id: ALICE.into(), network: None }.into(),
		},
	]);

	message
}

/// Normally, we charge fees for execution on the Blockchain.
/// XCM programs specify paying this fee with the `BuyExecution` instruction.
/// We're missing paying execution fees in the previous example.
/// Use up to 10% of the assets to pay for execution.
/// You're going to have to first convert `ALICE` into bytes.
/// Bonus points: Use the builder pattern.
pub fn withdraw_and_deposit_paying_fees() -> Xcm<()> {
	let alice_bytes: [u8; 32] = ALICE.into();
	let message = Xcm::builder()
		.withdraw_asset((Parent, 100u128))
		.buy_execution((Parent, 10u128), Unlimited)
		.deposit_asset(All, alice_bytes)
		.build();

	message
}
