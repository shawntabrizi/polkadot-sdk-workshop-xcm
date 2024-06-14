//! # Fundamentals Lesson 4
//!
//! Instructions are the fundamental building block of XCM programs.
//! Let's look at the most basic ones.

use xcm::latest::prelude::*;

use crate::constants::ALICE;

/// A message containing only a simple `ClearOrigin` instruction.
/// This instruction clears the origin of the sender, meaning after this point,
/// no special privileges are granted.
pub fn clear_origin_message() -> Xcm<()> {
	let message = todo!();

	message
}

/// Put all your knowledge of assets to the test.
/// Use the `WithdrawAsset` instruction to withdraw 100 planks
/// of the relay native token, i.e. DOT.
/// The XCM program is executed on a parachain.
/// This program won't do anything with the funds.
pub fn withdraw_asset() -> Xcm<()> {
	let assets: Assets = (Parent, 100u128).into();
	let message = todo!("{:?}", assets);

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
	let message = todo!("{:?}", assets);

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
	let message = todo!("{:?}", alice_bytes);

	message
}
