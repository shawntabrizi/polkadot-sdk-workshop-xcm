use crate::fundamentals::asset::*;
use xcm::latest::prelude::*;

#[test]
fn test_empty_assets() {
    let assets = empty_assets();

    assert_eq!(assets, Assets::new());
}
