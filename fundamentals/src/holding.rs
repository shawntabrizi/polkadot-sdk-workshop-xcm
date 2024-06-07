//! # Fundamentals lesson 3: Assets Holding
//!
//! Now that we have learned how to construct various kinds of assets and their locations, we can look more deeply how we would actually interact with multiple assets within the XCM system.
//!
//! ## Holding Registrar
//! 
//! To manage multiple assets within XCM, we use an abstraction called the "holding registrar" and a structure representing this abstraction called `AssetsInHolding`.
//! 
//! ```rust
//! /// Map of non-wildcard fungible and non-fungible assets held in the holding register.
//! #[derive(Default, Clone, Debug, Eq, PartialEq)]
//! pub struct AssetsInHolding {
//! 	/// The fungible assets.
//! 	pub fungible: BTreeMap<AssetId, u128>,
//! 
//! 	/// The non-fungible assets.
//! 	pub non_fungible: BTreeSet<(AssetId, AssetInstance)>,
//! }
//! ```
//! 
//! This structure keeps track of all assets which are currently being processed by XCM.
//! 
//! As you can see, the `AssetsInHolding` uses a `BTreeMap` and `BTreeSet` to manage fungible and non-fungible assets respectively. The holding registrar should be treated as a single pool of assets, and there should only be a single instance of any asset in the holding. If we want to include some assets into the holding, we should check if the asset already exists, and increase that value if so. Otherwise, we place that asset into the holding for the first time.
//! 
//! ## Cross-Chain Uses
//! 
//! The holding registrar is a key part of enabling end-to-end scenarios between consensus systems. Let's learn why.
//! 
//! ### Passing the Holding Registrar
//! 
//! The holding registrar keeps track of all assets in the current XCM state. So after executing all XCM messages on a single consensus system, the holding registrar may have some assets inside of it
//! 
//! ### In Memory
//! 
//! The holding registrar is a completely in-memory abstractions. Changes happening here do not necessarily reflect changes to the underlying blockchain state. If you include a new asset into the holding, with the intention of moving or transferring that asset into
//! 
//! ### Trust Assumptions
//! 
//! ## Other XCM State
//! 
//! The holding registrar is not the only state managed by the XCM executor.
//! 
//! It also has information like:
//! 
//! - `context`: Contextual information about where the message is coming from
//! -

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
	// TODO: Consider BTreeMap<AssetId, BTreeSet<AssetInstance>>
	//   or even BTreeMap<AssetId, SortedVec<AssetInstance>>
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
								taken.subsume(Asset::from((id, amount)).into());
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
