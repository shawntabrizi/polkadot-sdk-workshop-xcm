//! Mock network using `xcm_executor::XcmExecutor`.

use sp_runtime::BuildStorage;
use sp_tracing;
use xcm::prelude::*;
use xcm_executor::traits::ConvertLocation;
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain, TestExt};

pub mod asset_hub;
pub mod constants;
pub mod mock_message_queue;
pub mod parachain;
pub mod relay_chain;

#[cfg(test)]
mod tests;

use constants::{ALICE, BOB, CHARLIE, INITIAL_BALANCE};

decl_test_parachain! {
	pub struct ParaA {
		Runtime = parachain::Runtime,
		XcmpMessageHandler = parachain::MessageQueue,
		DmpMessageHandler = parachain::MessageQueue,
		new_ext = para_ext(1),
	}
}

decl_test_parachain! {
	pub struct ParaB {
		Runtime = parachain::Runtime,
		XcmpMessageHandler = parachain::MessageQueue,
		DmpMessageHandler = parachain::MessageQueue,
		new_ext = para_ext(2),
	}
}

decl_test_parachain! {
	pub struct ParaC {
		Runtime = parachain::Runtime,
		XcmpMessageHandler = parachain::MessageQueue,
		DmpMessageHandler = parachain::MessageQueue,
		new_ext = para_ext(3),
	}
}

decl_test_parachain! {
	pub struct AssetHub {
		Runtime = asset_hub::Runtime,
		XcmpMessageHandler = asset_hub::MessageQueue,
		DmpMessageHandler = asset_hub::MessageQueue,
		new_ext = para_ext(1000),
	}
}

decl_test_relay_chain! {
	pub struct Relay {
		Runtime = relay_chain::Runtime,
		RuntimeCall = relay_chain::RuntimeCall,
		RuntimeEvent = relay_chain::RuntimeEvent,
		XcmConfig = relay_chain::XcmConfig,
		MessageQueue = relay_chain::MessageQueue,
		System = relay_chain::System,
		new_ext = relay_ext(),
	}
}

decl_test_network! {
	pub struct MockNet {
		relay_chain = Relay,
		parachains = vec![
			(1, ParaA),
			(2, ParaB),
			(3, ParaC),
			(1000, AssetHub),
		],
	}
}

pub fn parent_account_id() -> parachain::AccountId {
	let location = (Parent,);
	parachain::LocationConverter::convert_location(&location.into()).unwrap()
}

pub fn child_account_id(para: u32) -> relay_chain::AccountId {
	let location = (Parachain(para),);
	relay_chain::LocationConverter::convert_location(&location.into()).unwrap()
}

pub fn child_account_account_id(para: u32, who: sp_runtime::AccountId32) -> relay_chain::AccountId {
	let location = (Parachain(para), AccountId32 { network: None, id: who.into() });
	relay_chain::LocationConverter::convert_location(&location.into()).unwrap()
}

pub fn sibling_account_id(para: u32) -> parachain::AccountId {
	let location = (Parent, Parachain(para));
	parachain::LocationConverter::convert_location(&location.into()).unwrap()
}

pub fn sibling_account_account_id(para: u32, who: sp_runtime::AccountId32) -> parachain::AccountId {
	let location = (Parent, Parachain(para), AccountId32 { network: None, id: who.into() });
	parachain::LocationConverter::convert_location(&location.into()).unwrap()
}

pub fn parent_account_account_id(who: sp_runtime::AccountId32) -> parachain::AccountId {
	let location = (Parent, AccountId32 { network: None, id: who.into() });
	parachain::LocationConverter::convert_location(&location.into()).unwrap()
}

pub fn para_ext(para_id: u32) -> sp_io::TestExternalities {
	use parachain::{MessageQueue, Runtime, System};
	let mut t = frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();

	let account_with_starting_balance = match para_id {
		1 | 1000 => ALICE,
		2 => BOB,
		3 => CHARLIE,
		_ => panic!("Not a valid para_id"),
	};

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![
			(account_with_starting_balance, INITIAL_BALANCE),
			(sibling_account_id(1), INITIAL_BALANCE),
			(sibling_account_id(2), INITIAL_BALANCE),
			(sibling_account_id(3), INITIAL_BALANCE),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		sp_tracing::try_init_simple();
		System::set_block_number(1);
		MessageQueue::set_para_id(para_id.into());
		#[cfg(feature = "other-parachain-tokens")]
		force_create_foreign_asset(para_id);
	});
	ext
}

/// Will create a foreign asset on one parachain representing the asset
/// of another.
/// If para_id is 1, then it will create the asset in 1, referencing the asset in 2.
/// If para_id is 2, then it will create the asset in 2, referencing the asset in 1.
#[cfg(feature = "other-parachain-tokens")]
fn force_create_foreign_asset(para_id: u32) {
	use frame_support::assert_ok;
	use parachain::{ForeignAssets, RuntimeOrigin};
	let other_para_id = if para_id == 1 { 2 } else { 1 };
	// We mark the asset as sufficient so tests are easier.
	// Being sufficient means an account with only this asset can exist.
	// In general, we should be careful with what is sufficient, as it can become an attack vector.
	assert_ok!(ForeignAssets::force_create(
		RuntimeOrigin::root(),
		(Parent, Parachain(other_para_id)).into(),
		ALICE, // Owner. You probably don't want this to be just an account.
		true,  // Sufficient.
		1,     // Minimum balance, this is the ED.
	));
}

pub fn relay_ext() -> sp_io::TestExternalities {
	use relay_chain::{Runtime, RuntimeOrigin, System, Uniques};

	let mut t = frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();

	pallet_balances::GenesisConfig::<Runtime> { balances: vec![(ALICE, INITIAL_BALANCE)] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
		assert_eq!(Uniques::force_create(RuntimeOrigin::root(), 1, ALICE, true), Ok(()));
		assert_eq!(Uniques::mint(RuntimeOrigin::signed(ALICE), 1, 42, child_account_id(1)), Ok(()));
	});
	ext
}

pub type RelayChainPalletXcm = pallet_xcm::Pallet<relay_chain::Runtime>;
pub type ParachainPalletXcm = pallet_xcm::Pallet<parachain::Runtime>;
