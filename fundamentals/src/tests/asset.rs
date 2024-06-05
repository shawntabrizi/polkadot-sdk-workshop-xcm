use crate::asset::*;
use xcm::latest::prelude::*;

#[test]
fn fungibles() {
	assert_eq!(EmptyAssets::get(), Assets::new());
	assert_eq!(NativeToken::get(), AssetId(Location::new(0, [])));
	assert_eq!(DotToken::get(), AssetId(Location::new(1, [])));
	assert_eq!(OneHundredNative::get(), (NativeToken::get(), 100_000_000_000_000u128).into());
	assert_eq!(OneHundredDot::get(), (DotToken::get(), 1_000_000_000_000u128).into());
}

#[test]
fn nonfungibles() {
	assert_eq!(
		NftLocation::get(),
		Location::new(1, [Parachain(1000), PalletInstance(5), GeneralIndex(42)])
	);
	assert_eq!(Nft::get(), (NftLocation::get(), 69u64).into());
}

#[test]
fn filters() {
	assert_eq!(AllAssetsFilter::get(), AssetFilter::Wild(WildAsset::All));
	assert_eq!(DotFilter::get(), AssetFilter::Definite(vec![OneHundredDot::get()].into()));
	assert_eq!(NativeFilter::get(), AssetFilter::Definite(vec![OneHundredNative::get()].into()));
}
