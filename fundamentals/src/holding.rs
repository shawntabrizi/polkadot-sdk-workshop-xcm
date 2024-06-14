//! # Fundamentals lesson 3: Assets Holding
//!
//! Create and managing an in-memory asset holding for the XCM Executor.

use sp_runtime::Saturating;
use sp_std::{
	collections::{btree_map::BTreeMap, btree_set::BTreeSet},
	mem,
	prelude::*,
};
use xcm::latest::prelude::*;

/// Map of non-wildcard fungible and non-fungible assets held in the holding register.
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct AssetsInHolding {
	/// The fungible assets.
	pub fungible: BTreeMap<AssetId, u128>,

	/// The non-fungible assets.
	pub non_fungible: BTreeSet<(AssetId, AssetInstance)>,
}

impl From<Vec<Asset>> for AssetsInHolding {
	fn from(assets: Vec<Asset>) -> AssetsInHolding {
		let mut result = Self::default();
		for asset in assets.into_iter() {
			result.subsume(asset)
		}
		result
	}
}

impl From<Assets> for AssetsInHolding {
	fn from(assets: Assets) -> AssetsInHolding {
		assets.into_inner().into()
	}
}

impl AssetsInHolding {
	/// New value, containing no assets.
	pub fn new() -> Self {
		Self::default()
	}

	/// Mutate `self` to contain the given `asset`, saturating if necessary.
	pub fn subsume(&mut self, asset: Asset) {
		/* TODO:
			- For fungible assets:
				- Check if we already have some existing assets in the holding.
				- If we do, `saturating_add` to the existing amount.
				- If we don't, `insert` the asset into the `BTreeMap`.
			- For non-fungible assets:
				- Simply insert the `asset.id` and `instance` into the `BTreeSet`.

		*/
		todo!("{:?}", asset)
	}

	/// Mutate `self` to contain all given `assets`, saturating if necessary.
	/// NOTE: This function can be written more optimally given the fact that assets in the holding
	/// are always in order.
	pub fn subsume_assets(&mut self, mut assets: AssetsInHolding) {
		/* TODO:
			- For each `(key, value)` in `assets.fungible`.
				- If the asset already exists `self.fungible`, `saturating_accrue` the new `value`.
				- If it is new, `insert` the new asset.
			- For non-fungible assets, take `self.non_fungible` and `append` the `assets.non_fungible`.
			- Then clear `assets.fungible` and `assets.non_fungible`.
		*/
		todo!("{:?}", assets)
	}

	/// Mutates `self` to its original value less `mask` and returns `true` iff it contains at least
	/// `mask`.
	///
	/// Returns `Ok` with the non-wildcard equivalence of `mask` taken and mutates `self` to its
	/// value minus `mask` if `self` contains `asset`, and return `Err` otherwise.
	pub fn saturating_take(&mut self, asset: AssetFilter) -> AssetsInHolding {
		let mut taken = AssetsInHolding::new();

		match asset {
			AssetFilter::Wild(All) => {
				/* TODO: Match on and return all assets.
					- Create a new `AssetsInHolding` called `new_holding`
					- use `mem::swap` to swap this with `self`
					- return the `new_holding`
				*/
				todo!()
			},
			AssetFilter::Definite(assets) =>
				for asset in assets.into_inner() {
					match asset {
						Asset { fun: Fungible(amount), id } => {
							/* TODO:
								- Check if there is any of this asset in the holding.
								- Get the `min` amount between the filter and what is in holding.
								- Reduce the holding by that amount.
								- Check if we should `remove` the asset from the holding when it's value goes to zero.
								- Finally, `subsume` these assets into `taken`.
							*/
							todo!("{:?} {:?}", amount, id)
						},
						Asset { fun: NonFungible(instance), id } => {
							/* TODO:
								- Try to `remove` the asset from `self.non_fungible`
								- If the asset was there, move it to `taken`
							*/
							todo!("{:?} {:?}", instance, id)
						},
					}
				},
				_ => unimplemented!("Handling other asset filters is not included to simplify the scope of the project.")
		}

		taken
	}

	/// A consuming iterator over all assets.
	pub fn into_assets_iter(self) -> impl Iterator<Item = Asset> {
		self.fungible
			.into_iter()
			.map(|(id, amount)| Asset { fun: Fungible(amount), id })
			.chain(
				self.non_fungible
					.into_iter()
					.map(|(id, instance)| Asset { fun: NonFungible(instance), id }),
			)
	}
}
