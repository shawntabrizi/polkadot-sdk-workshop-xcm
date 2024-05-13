use crate::fundamentals::xcm_executor::*;
use crate::parachain;
use crate::ParaA;
use xcm_simulator::TestExt;
use frame_support::assert_ok;
use xcm::latest::prelude::*;

struct Config;
impl XcmConfig for Config {
    type RuntimeCall = parachain::RuntimeCall;
    type AssetTransactor = parachain::asset_transactor::AssetTransactor;
    type Barrier = ();
}

#[test]
fn clear_origin_works() {
    // TODO
}

#[test]
fn withdraw_works() {
    ParaA::execute_with(|| {
        let message = Xcm::<parachain::RuntimeCall>::builder_unsafe()
            .withdraw_asset((Parent, 100u128))
            .build();
        let origin = AccountId32 { id: crate::ALICE.into(), network: None }.into();
        let mut executor = XcmExecutor::<Config>::new(origin);
        assert_ok!(executor.execute(message));
        assert_eq!(executor.holding.fungible.get(&Parent.into()), Some(&100u128));
    });
}

#[test]
fn buy_execution_works() {
    // TODO
}

#[test]
fn deposit_asset_works() {
    // TODO
}

#[test]
fn transact_works() {
    // TODO
}
