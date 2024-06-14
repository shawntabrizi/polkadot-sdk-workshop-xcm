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
		match asset.fun {
			Fungible(amount) =>
				if let Some(existing_amount) = self.fungible.get_mut(&asset.id) {
					*existing_amount = existing_amount.saturating_add(amount);
				} else {
					self.fungible.insert(asset.id, amount);
				},
			NonFungible(instance) => {
				self.non_fungible.insert((asset.id, instance));
			},
		}
	}

	/// Mutate `self` to contain all given `assets`, saturating if necessary.
	/// NOTE: This function can be written more optimally given the fact that assets in the holding
	/// are always in order.
	pub fn subsume_assets(&mut self, mut assets: AssetsInHolding) {
		// Loop through all fungible assets in `assets` and add them to `self`
		for (key, value) in assets.fungible.iter_mut() {
			if let Some(existing_value) = self.fungible.get_mut(key) {
				// If the asset already exists in `self`, add the values, saturating if necessary
				existing_value.saturating_accrue(*value);
			} else {
				// Otherwise, insert the new asset into `self`
				self.fungible.insert(key.clone(), *value);
			}
		}

		// Append all non-fungible assets from `assets` to `self`
		self.non_fungible.append(&mut assets.non_fungible);

		// Clear the original `assets` collections as they are now part of `self`
		assets.fungible.clear();
		assets.non_fungible.clear();
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
				let mut new_holding = AssetsInHolding::new();
				mem::swap(&mut *self, &mut new_holding);
				return new_holding
			},
			AssetFilter::Definite(assets) =>
				for asset in assets.into_inner() {
					match asset {
						Asset { fun: Fungible(amount), id } => {
							let amount = if let Some(self_amount) = self.fungible.get_mut(&id) {
								let amount = amount.min(*self_amount);
								*self_amount -= amount;
								if *self_amount == 0 {
									self.fungible.remove(&id);
								}
								amount
							} else {
								0
							};
							if amount > 0 {
								taken.subsume(Asset::from((id, amount)));
							}
						},
						Asset { fun: NonFungible(instance), id } => {
							let id_instance = (id, instance);
							if self.non_fungible.remove(&id_instance) {
								taken.subsume(id_instance.into());
							}
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
