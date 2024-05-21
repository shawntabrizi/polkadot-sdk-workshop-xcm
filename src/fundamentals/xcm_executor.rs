//! # Fundamentals lesson 4: XCM Executor
//!
//! Create your own executor for XCM.

// TODO: Key Takeaways
// - Match statement to process instructions
// - executor state (origin, holding, etc...)
// - Assets in Holding, and all its behaviors
// - How XCM Config can change logic in the executor
// - Transactional
// - composability of instructions

use frame_support::{
	dispatch::{GetDispatchInfo, PostDispatchInfo},
	Parameter,
};
use sp_runtime::{traits::Dispatchable, Saturating};
use sp_std::{
	collections::{btree_map::BTreeMap, btree_set::BTreeSet},
	marker::PhantomData,
	mem,
	prelude::*,
};
use xcm::latest::prelude::*;
use xcm_executor::traits::{ProcessTransaction, ShouldExecute, TransactAsset};

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

// TODO: Rewrite all of this to be simple for students to understand and complete for scenarios
// below.
impl AssetsInHolding {
	/// New value, containing no assets.
	pub fn new() -> Self {
		Self::default()
	}

	/// Total number of distinct assets.
	pub fn len(&self) -> usize {
		self.fungible.len() + self.non_fungible.len()
	}

	/// Mutate `self` to contain all given `assets`, saturating if necessary.
	///
	/// NOTE: [`AssetsInHolding`] are always sorted, allowing us to optimize this function from
	/// `O(n^2)` to `O(n)`.
	pub fn subsume_assets(&mut self, mut assets: AssetsInHolding) {
		let mut f_iter = assets.fungible.iter_mut();
		let mut g_iter = self.fungible.iter_mut();
		if let (Some(mut f), Some(mut g)) = (f_iter.next(), g_iter.next()) {
			loop {
				if f.0 == g.0 {
					// keys are equal. in this case, we add `self`'s balance for the asset onto
					// `assets`, balance, knowing that the `append` operation which follows will
					// clobber `self`'s value and only use `assets`'s.
					(*f.1).saturating_accrue(*g.1);
				}
				if f.0 <= g.0 {
					f = match f_iter.next() {
						Some(x) => x,
						None => break,
					};
				}
				if f.0 >= g.0 {
					g = match g_iter.next() {
						Some(x) => x,
						None => break,
					};
				}
			}
		}
		self.fungible.append(&mut assets.fungible);
		self.non_fungible.append(&mut assets.non_fungible);
	}

	/// Mutate `self` to contain the given `asset`, saturating if necessary.
	///
	/// Wildcard values of `asset` do nothing.
	pub fn subsume(&mut self, asset: Asset) {
		match asset.fun {
			Fungible(amount) => {
				self.fungible
					.entry(asset.id)
					.and_modify(|e| *e = e.saturating_add(amount))
					.or_insert(amount);
			},
			NonFungible(instance) => {
				self.non_fungible.insert((asset.id, instance));
			},
		}
	}

	/// Mutates `self` to its original value less `mask` and returns `true` iff it contains at least
	/// `mask`.
	///
	/// Returns `Ok` with the non-wildcard equivalence of `mask` taken and mutates `self` to its
	/// value minus `mask` if `self` contains `asset`, and return `Err` otherwise.
	pub fn saturating_take(&mut self, asset: AssetFilter) -> AssetsInHolding {
		self.general_take(asset, true)
			.expect("general_take never results in error when saturating")
	}

	/// Mutates `self` to its original value less `mask` and returns assets that were removed.
	///
	/// If `saturate` is `true`, then `self` is considered to be masked by `mask`, thereby avoiding
	/// any attempt at reducing it by assets it does not contain. In this case, the function is
	/// infallible. If `saturate` is `false` and `mask` references a definite asset which `self`
	/// does not contain then an error is returned.
	///
	/// The number of unique assets which are removed will respect the `count` parameter in the
	/// counted wildcard variants.
	///
	/// Returns `Ok` with the definite assets token from `self` and mutates `self` to its value
	/// minus `mask`. Returns `Err` in the non-saturating case where `self` did not contain (enough
	/// of) a definite asset to be removed.
	fn general_take(&mut self, mask: AssetFilter, saturate: bool) -> Result<AssetsInHolding, ()> {
		let mut taken = AssetsInHolding::new();
		let maybe_limit = mask.limit().map(|x| x as usize);
		match mask {
			// TODO: Counted variants where we define `limit`.
			AssetFilter::Wild(All) | AssetFilter::Wild(AllCounted(_)) => {
				if maybe_limit.map_or(true, |l| self.len() <= l) {
					return Ok(self.swapped(AssetsInHolding::new()))
				} else {
					let fungible = mem::replace(&mut self.fungible, Default::default());
					fungible.into_iter().for_each(|(c, amount)| {
						if maybe_limit.map_or(true, |l| taken.len() < l) {
							taken.fungible.insert(c, amount);
						} else {
							self.fungible.insert(c, amount);
						}
					});
					let non_fungible = mem::replace(&mut self.non_fungible, Default::default());
					non_fungible.into_iter().for_each(|(c, instance)| {
						if maybe_limit.map_or(true, |l| taken.len() < l) {
							taken.non_fungible.insert((c, instance));
						} else {
							self.non_fungible.insert((c, instance));
						}
					});
				}
			},
			AssetFilter::Wild(AllOfCounted { fun: WildFungible, id, .. }) |
			AssetFilter::Wild(AllOf { fun: WildFungible, id }) =>
				if maybe_limit.map_or(true, |l| l >= 1) {
					if let Some((id, amount)) = self.fungible.remove_entry(&id) {
						taken.fungible.insert(id, amount);
					}
				},
			AssetFilter::Wild(AllOfCounted { fun: WildNonFungible, id, .. }) |
			AssetFilter::Wild(AllOf { fun: WildNonFungible, id }) => {
				let non_fungible = mem::replace(&mut self.non_fungible, Default::default());
				non_fungible.into_iter().for_each(|(c, instance)| {
					if c == id && maybe_limit.map_or(true, |l| taken.len() < l) {
						taken.non_fungible.insert((c, instance));
					} else {
						self.non_fungible.insert((c, instance));
					}
				});
			},
			AssetFilter::Definite(assets) => {
				if !saturate {
					self.ensure_contains(&assets)?;
				}
				for asset in assets.into_inner().into_iter() {
					match asset {
						Asset { fun: Fungible(amount), id } => {
							let (remove, amount) = match self.fungible.get_mut(&id) {
								Some(self_amount) => {
									let amount = amount.min(*self_amount);
									*self_amount -= amount;
									(*self_amount == 0, amount)
								},
								None => (false, 0),
							};
							if remove {
								self.fungible.remove(&id);
							}
							if amount > 0 {
								taken.subsume(Asset::from((id, amount)).into());
							}
						},
						Asset { fun: NonFungible(instance), id } => {
							let id_instance = (id, instance);
							if self.non_fungible.remove(&id_instance) {
								taken.subsume(id_instance.into())
							}
						},
					}
				}
			},
		}
		Ok(taken)
	}

	/// Swaps two mutable AssetsInHolding, without deinitializing either one.
	pub fn swapped(&mut self, mut with: AssetsInHolding) -> Self {
		mem::swap(&mut *self, &mut with);
		with
	}

	/// Returns an error unless all `assets` are contained in `self`. In the case of an error, the
	/// first asset in `assets` which is not wholly in `self` is returned.
	pub fn ensure_contains(&self, assets: &Assets) -> Result<(), ()> {
		for asset in assets.inner().iter() {
			match asset {
				Asset { fun: Fungible(amount), id } => {
					if self.fungible.get(id).map_or(true, |a| a < amount) {
						return Err(())
						//return Err(TakeError::AssetUnderflow((id.clone(), *amount).into()))
					}
				},
				Asset { fun: NonFungible(instance), id } => {
					let id_instance = (id.clone(), *instance);
					if !self.non_fungible.contains(&id_instance) {
						return Err(())
						//return Err(TakeError::AssetUnderflow(id_instance.into()))
					}
				},
			}
		}
		return Ok(())
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

pub trait XcmConfig {
	type RuntimeCall: Parameter + Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo;

	/// How to withdraw and deposit an asset.
	type AssetTransactor: TransactAsset;

	/// Whether we should execute the given XCM at all.
	type Barrier: ShouldExecute;

	/// Transactional processor for XCM instructions.
	type TransactionalProcessor: ProcessTransaction;
}

pub struct XcmExecutor<Config: XcmConfig> {
	pub holding: AssetsInHolding,
	pub context: XcmContext,
	_config: PhantomData<Config>,
}

// TODO: Have students implement the logic for a few basic instructions.
impl<Config: XcmConfig> XcmExecutor<Config> {
	pub fn new(origin: impl Into<Location>) -> Self {
		let origin = origin.into();
		let context =
			XcmContext { origin: Some(origin), message_id: Default::default(), topic: None };
		Self { holding: Default::default(), context, _config: PhantomData }
	}

	fn origin_ref(&self) -> Option<&Location> {
		self.context.origin.as_ref()
	}

	fn cloned_origin(&self) -> Option<Location> {
		self.context.origin.clone()
	}

	/// Process an entire XCM program.
	pub fn execute(&mut self, xcm: Xcm<Config::RuntimeCall>) -> Result<(), XcmError> {
		log::trace!(
			target: "xcm::execute",
			"xcm: {:?}",
			xcm
		);
		for instruction in xcm.0.into_iter() {
			self.process_instruction(instruction)?;
		}
		Ok(())
	}

	/// Process a single XCM instruction, mutating the state of the XCM virtual machine.
	fn process_instruction(
		&mut self,
		instr: Instruction<Config::RuntimeCall>,
	) -> Result<(), XcmError> {
		log::trace!(
			target: "xcm::process_instruction",
			"=== {:?}",
			instr
		);
		match instr {
			WithdrawAsset(assets) => {
				let origin = self.origin_ref().ok_or(XcmError::BadOrigin)?;
				Config::TransactionalProcessor::process(|| {
					// Take `assets` from the origin account (on-chain)...
					for asset in assets.inner() {
						Config::AssetTransactor::withdraw_asset(
							asset,
							origin,
							Some(&self.context),
						)?;
					}
					Ok(())
				})
				.and_then(|_| {
					// ...and place into holding.
					self.holding.subsume_assets(assets.into());
					Ok(())
				})
			},
			DepositAsset { assets, beneficiary } => {
				let old_holding = self.holding.clone();
				let result = Config::TransactionalProcessor::process(|| {
					let deposited = self.holding.saturating_take(assets);
					for asset in deposited.into_assets_iter() {
						Config::AssetTransactor::deposit_asset(
							&asset,
							&beneficiary,
							Some(&self.context),
						)?;
					}
					Ok(())
				});
				if Config::TransactionalProcessor::IS_TRANSACTIONAL && result.is_err() {
					self.holding = old_holding;
				}
				result
			},
			TransferAsset { assets, beneficiary } => {
				Config::TransactionalProcessor::process(|| {
					// Take `assets` from the origin account (on-chain) and place into dest account.
					let origin = self.origin_ref().ok_or(XcmError::BadOrigin)?;
					for asset in assets.inner() {
						Config::AssetTransactor::transfer_asset(
							&asset,
							origin,
							&beneficiary,
							&self.context,
						)?;
					}
					Ok(())
				})
			},
			_ => unimplemented!(),
		}
	}
}
