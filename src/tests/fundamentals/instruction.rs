use crate::fundamentals::instruction::*;
use codec::Encode;
use xcm::latest::prelude::*;

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
				beneficiary: AccountId32 { id: crate::ALICE.into(), network: None }.into()
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
				beneficiary: AccountId32 { id: crate::ALICE.into(), network: None }.into()
			}
		])
	);
}

#[test]
fn transact_correct() {
	let call = crate::parachain::RuntimeCall::System(frame_system::Call::<
		crate::parachain::Runtime,
	>::remark {
		remark: b"Hello, world!".to_vec(),
	});
	let message = transact();

	assert_eq!(
		message,
		Xcm(vec![
			WithdrawAsset((Parent, 10u128).into()),
			BuyExecution { fees: (Parent, 10u128).into(), weight_limit: Unlimited.into() },
			Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: Weight::MAX,
				call: call.encode().into()
			},
		])
	);
}
