use crate::fundamentals::holding::*;
use xcm_executor::AssetsInHolding as OfficialAssetsInHolding;

use xcm::latest::prelude::*;
#[allow(non_snake_case)]
/// Concrete fungible constructor
fn CF(amount: u128) -> Asset {
	(Here, amount).into()
}
#[allow(non_snake_case)]
/// Concrete non-fungible constructor
fn CNF(instance_id: u8) -> Asset {
	(Here, [instance_id; 4]).into()
}

fn test_assets() -> AssetsInHolding {
	let mut assets = AssetsInHolding::new();
	assets.subsume(CF(300));
	assets.subsume(CNF(40));
	assets
}

fn official_test_assets() -> OfficialAssetsInHolding {
	let mut assets = OfficialAssetsInHolding::new();
	assets.subsume(CF(300));
	assets.subsume(CNF(40));
	assets
}

fn are_holdings_eq(local: AssetsInHolding, official: OfficialAssetsInHolding) -> bool {
	let fungible_eq = local.fungible == official.fungible;
	if !fungible_eq {
		println!("Fungible assets are not the same.")
	}
	let non_fungible_eq = local.non_fungible == official.non_fungible;
	if !fungible_eq {
		println!("Non-fungible assets are not the same.")
	}

	return fungible_eq && non_fungible_eq
}

#[test]
fn subsume_assets_works() {
	let mut t1 = test_assets();
	let mut t2 = AssetsInHolding::new();
	t2.subsume(CF(300));
	t2.subsume(CNF(50));
	t1.subsume_assets(t2.clone());

	let mut o1 = official_test_assets();
	let mut o2 = OfficialAssetsInHolding::new();
	o2.subsume(CF(300));
	o2.subsume(CNF(50));
	o1.subsume_assets(o2.clone());

	assert!(are_holdings_eq(t1, o1));
}

#[test]
fn into_assets_iter_works() {
	let assets = test_assets();
	let mut iter = assets.into_assets_iter();
	// Order defined by implementation: CF, CNF
	assert_eq!(Some(CF(300)), iter.next());
	assert_eq!(Some(CNF(40)), iter.next());
	assert_eq!(None, iter.next());
}

#[test]
fn assets_into_works() {
	let mut assets_vec: Vec<Asset> = Vec::new();
	assets_vec.push(CF(300));
	assets_vec.push(CNF(40));
	// Push same group of tokens again
	assets_vec.push(CF(300));
	assets_vec.push(CNF(40));

	let assets: AssetsInHolding = assets_vec.into();
	let mut iter = assets.into_assets_iter();
	// Fungibles add
	assert_eq!(Some(CF(600)), iter.next());
	// Non-fungibles collapse
	assert_eq!(Some(CNF(40)), iter.next());
	assert_eq!(None, iter.next());
}

#[test]
fn saturating_take_all_and_none_works() {
	let mut assets = test_assets();

	let taken_none = assets.saturating_take(vec![].into());
	assert_eq!(None, taken_none.into_assets_iter().next());
	let taken_all = assets.saturating_take(All.into());
	// Everything taken
	assert_eq!(None, assets.into_assets_iter().next());
	let all_iter = taken_all.into_assets_iter();
	assert!(all_iter.eq(test_assets().into_assets_iter()));
}

#[test]
fn saturating_take_basic_works() {
	let mut assets1 = test_assets();

	let mut assets2 = AssetsInHolding::new();
	// This is more then 300, so it takes everything
	assets2.subsume(CF(600));
	// This asset should be taken
	assets2.subsume(CNF(40));
	let assets2: Assets = assets2.into_assets_iter().collect::<Vec<Asset>>().into();

	let taken = assets1.saturating_take(assets2.into());
	let taken = taken.into_assets_iter().collect::<Vec<_>>();
	assert_eq!(taken, vec![CF(300), CNF(40)]);
}
