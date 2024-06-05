use codec::Encode;
use xcm::latest::prelude::*;

use crate::constants;
use crate::instruction::*;

#[test]
fn clear_origin_message_correct() {
	let message = clear_origin_message();

	assert_eq!(message, Xcm(vec![ClearOrigin]));
}

#[test]
fn withdraw_asset_correct() {
	let message = withdraw_asset();

	assert_eq!(message, Xcm(vec![WithdrawAsset((Parent, 100u128).into())]));
}

#[test]
fn withdraw_and_deposit_correct() {
	let message = withdraw_and_deposit();

	assert_eq!(
		message,
		Xcm(vec![
			WithdrawAsset((Parent, 100u128).into()),
			DepositAsset {
				assets: All.into(),
				beneficiary: AccountId32 { id: constants::ALICE.into(), network: None }.into()
			}
		])
	);
}

#[test]
fn withdraw_and_deposit_paying_fees_correct() {
	let message = withdraw_and_deposit_paying_fees();

	assert_eq!(
		message,
		Xcm(vec![
			WithdrawAsset((Parent, 100u128).into()),
			BuyExecution { fees: (Parent, 10u128).into(), weight_limit: Unlimited.into() },
			DepositAsset {
				assets: All.into(),
				beneficiary: AccountId32 { id: constants::ALICE.into(), network: None }.into()
			}
		])
	);
}
