use crate::parachain::constants::sandbox::*;
use xcm::latest::prelude::*;

#[test]
fn sibling_1337_location() {
    assert_eq!(Sib1337Location::get(), Location::new(0, [Parachain(1337)]));
}

#[test]
fn dot_location() {
    println!("{:?}", DotLocation::get());
    assert_eq!(
        DotLocation::get(),
        Location::new(0, [GlobalConsensus(Polkadot)])
    );
}
