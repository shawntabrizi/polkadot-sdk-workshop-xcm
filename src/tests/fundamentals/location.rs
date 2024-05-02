use crate::fundamentals::location::*;
use xcm::latest::prelude::*;

#[test]
fn sibling_1337_location() {
    assert_eq!(Sib1337Location::get(), Location::new(0, [Parachain(1337)]));
}

#[test]
fn dot_location() {
    assert_eq!(
        DotLocation::get(),
        Location::new(0, [GlobalConsensus(Polkadot)])
    );
}

#[test]
fn parachain_69_location() {
    assert_eq!(
        Para69Location::get(),
        Location::new(0, [GlobalConsensus(Polkadot), Parachain(69)])
    );
}

#[test]
fn alice_1337_location() {
    let alice_bytes: [u8; 32] = crate::ALICE.into();
    assert_eq!(
        Alice1337Location::get(),
        Location::new(0, [Parachain(1337), alice_bytes.into()])
    );
}

// #[test]
// fn relay_chain_balances_pallet_location() {
//     assert_eq!(
//         RelayChainBalancesLocation::get(),
//         Location::new(0, [PalletInstance()])
//     );
// }
