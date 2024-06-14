use crate::asset::*;
use xcm::latest::prelude::*;

#[test]
fn fungibles() {
	assert_eq!(EmptyAssets::get(), Assets::new());
	assert_eq!(Usdt::get(), AssetId(Location::new(0, [PalletInstance(50), GeneralIndex(1984)])));
	assert_eq!(DotToken::get(), AssetId(Location::new(1, [])));
	assert_eq!(OneHundredUsdt::get(), (Usdt::get(), 100_000_000u128).into());
	assert_eq!(OneHundredDot::get(), (DotToken::get(), 1_000_000_000_000u128).into());
}

#[test]
fn nonfungibles() {
	assert_eq!(
		NftLocation::get(),
		Location::new(0, [PalletInstance(52), GeneralIndex(3)])
	);
	assert_eq!(Nft::get(), (NftLocation::get(), 69u64).into());
}

#[test]
fn filters() {
	assert_eq!(AllAssetsFilter::get(), AssetFilter::Wild(WildAsset::All));
	assert_eq!(DotFilter::get(), AssetFilter::Definite(vec![OneHundredDot::get()].into()));
	assert_eq!(UsdtFilter::get(), AssetFilter::Definite(vec![OneHundredUsdt::get()].into()));
}
