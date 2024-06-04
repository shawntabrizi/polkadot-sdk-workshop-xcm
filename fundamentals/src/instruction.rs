//! # Fundamentals Lesson 4
//!
//! Instructions are the fundamental building block of XCM programs.
//! Let's look at the most basic ones.

#![allow(dead_code)]

use chains::parachain;
use xcm::latest::prelude::*;

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
/// Deposit all the assets to `crate::constants::ALICE`.
/// Remember how to use wildcards.
pub fn withdraw_and_deposit() -> Xcm<()> {
	let assets: Assets = (Parent, 100u128).into();
	let message = Xcm(vec![
		WithdrawAsset(assets),
		DepositAsset {
			assets: All.into(),
			beneficiary: AccountId32 { id: crate::constants::ALICE.into(), network: None }.into(),
		},
	]);

	message
}

/// Normally, we charge fees for execution on the Blockchain.
/// XCM programs specify paying this fee with the `BuyExecution` instruction.
/// We're missing paying execution fees in the previous example.
/// Use up to 10% of the assets to pay for execution.
/// You're going to have to first convert `crate::constants::ALICE` into bytes.
/// Bonus points: Use the builder pattern.
pub fn withdraw_and_deposit_paying_fees() -> Xcm<()> {
	let alice_bytes: [u8; 32] = crate::constants::ALICE.into();
	let message = Xcm::builder()
		.withdraw_asset((Parent, 100u128))
		.buy_execution((Parent, 10u128), Unlimited)
		.deposit_asset(All, alice_bytes)
		.build();

	message
}

/// The `Transact` instruction lets us execute any call on the receiving system.
/// Try executing `System::remark` on the receiving system.
/// Remark "Hello, world!".
/// Assume the receiver is another parachain.
/// Use `Weight::MAX` as `require_weight_at_most` just for testing.
/// Remember to pay for fees.
/// You're going to need to import `codec::Encode` to encode the call.
pub fn transact() -> Xcm<parachain::RuntimeCall> {
	use codec::Encode;
	let call = parachain::RuntimeCall::System(frame_system::Call::<
		parachain::Runtime,
	>::remark {
		remark: b"Hello, world!".to_vec(),
	});
	let message = Xcm::builder()
		.withdraw_asset((Parent, 10u128))
		.buy_execution((Parent, 10u128), Unlimited)
		.transact(OriginKind::SovereignAccount, Weight::MAX, call.encode())
		.build();
	message
}
