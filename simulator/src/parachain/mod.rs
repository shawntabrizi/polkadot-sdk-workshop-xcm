// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Parachain runtime mock.

mod xcm_config;
pub use xcm_config::*;

use core::marker::PhantomData;
use frame_support::{
	derive_impl, parameter_types,
	traits::{
		AsEnsureOriginWithArg, ConstU128, ContainsPair, EnsureOrigin, EnsureOriginWithArg,
		Everything, Nothing,
	},
	weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight},
};
use frame_system::{EnsureRoot, EnsureSigned};
use sp_core::ConstU32;
use sp_runtime::{
	traits::{Get, IdentityLookup},
	AccountId32,
};
use sp_std::prelude::*;
use xcm::latest::prelude::*;
use xcm_builder::{EnsureXcmOrigin, SignedToAccountId32};
use xcm_executor::{traits::ConvertLocation, XcmExecutor};

use super::mock_message_queue;

pub type AccountId = AccountId32;
pub type Balance = u128;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Runtime {
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
}

// TODO: Put reasonable values.
parameter_types! {
	pub const AssetDeposit: Balance = 1;
	pub const ApprovalDeposit: Balance = 1;
	pub const AssetAccountDeposit: Balance = 1;
	pub const MetadataDepositBase: Balance = 1;
	pub const MetadataDepositPerByte: Balance = 1;
}

#[derive_impl(pallet_assets::config_preludes::TestDefaultConfig)]
impl pallet_assets::Config<pallet_assets::Instance1> for Runtime {
	type Currency = Balances;
	type Balance = Balance;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type ApprovalDeposit = ApprovalDeposit;
	type AssetAccountDeposit = AssetAccountDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type Freezer = ();
}

#[cfg(not(feature = "register-assets"))]
pub type ForeignCreators = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;

#[cfg(feature = "register-assets")]
pub struct ForeignCreators;

// `EnsureOriginWithArg` impl for `CreateOrigin` which allows only XCM origins
// which are locations containing the class location.
#[cfg(feature = "register-assets")]
impl EnsureOriginWithArg<RuntimeOrigin, Location> for ForeignCreators {
	type Success = AccountId;

	fn try_origin(
		o: RuntimeOrigin,
		a: &Location,
	) -> sp_std::result::Result<Self::Success, RuntimeOrigin> {
		let origin_location = pallet_xcm::EnsureXcm::<Everything>::try_origin(o.clone())?;
		if !a.starts_with(&origin_location) {
			return Err(o);
		}
		xcm_config::location_converter::LocationConverter::convert_location(&origin_location)
			.ok_or(o)
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin(a: &Location) -> Result<RuntimeOrigin, ()> {
		Ok(pallet_xcm::Origin::Xcm(a.clone()).into())
	}
}

#[derive_impl(pallet_assets::config_preludes::TestDefaultConfig)]
impl pallet_assets::Config<pallet_assets::Instance2> for Runtime {
	type AssetId = xcm::v4::Location;
	type AssetIdParameter = xcm::v4::Location;
	type Currency = Balances;
	type Balance = Balance;
	// In tests, we already create foreign assets for other parachains.
	// TODO: In reality, we would want them to create their own assets with their XCM origin.
	type CreateOrigin = ForeignCreators;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type ApprovalDeposit = ApprovalDeposit;
	type AssetAccountDeposit = AssetAccountDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type Freezer = ();
}

#[cfg(feature = "runtime-benchmarks")]
pub struct UniquesHelper;
#[cfg(feature = "runtime-benchmarks")]
impl pallet_uniques::BenchmarkHelper<Location, AssetInstance> for UniquesHelper {
	fn collection(i: u16) -> Location {
		GeneralIndex(i as u128).into()
	}
	fn item(i: u16) -> AssetInstance {
		AssetInstance::Index(i as u128)
	}
}

impl pallet_uniques::Config<pallet_uniques::Instance1> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32; // To identify collections.
	type ItemId = u32; // To identify individual NFTs.
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type CollectionDeposit = ConstU128<1_000>;
	type ItemDeposit = ConstU128<1_000>;
	type MetadataDepositBase = ConstU128<1_000>;
	type AttributeDepositBase = ConstU128<1_000>;
	type DepositPerByte = ConstU128<1>;
	type StringLimit = ConstU32<64>;
	type KeyLimit = ConstU32<64>;
	type ValueLimit = ConstU32<128>;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = UniquesHelper;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type Locker = ();
}

impl pallet_uniques::Config<pallet_uniques::Instance2> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = Location;
	type ItemId = AssetInstance;
	type Currency = Balances;
	type CreateOrigin = ForeignCreators;
	type ForceOrigin = EnsureRoot<AccountId>;
	type CollectionDeposit = ConstU128<1_000>;
	type ItemDeposit = ConstU128<1_000>;
	type MetadataDepositBase = ConstU128<1_000>;
	type AttributeDepositBase = ConstU128<1_000>;
	type DepositPerByte = ConstU128<1>;
	type StringLimit = ConstU32<64>;
	type KeyLimit = ConstU32<64>;
	type ValueLimit = ConstU32<128>;
	type Locker = ();
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = UniquesHelper;
}

parameter_types! {
	pub const ReservedXcmpWeight: Weight = Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_div(4), 0);
	pub const ReservedDmpWeight: Weight = Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_div(4), 0);
}

impl mock_message_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

pub type LocalOriginToLocation =
	SignedToAccountId32<RuntimeOrigin, AccountId, constants::RelayNetwork>;

pub struct TrustedLockerCase<T>(PhantomData<T>);
impl<T: Get<(Location, AssetFilter)>> ContainsPair<Location, Asset> for TrustedLockerCase<T> {
	fn contains(origin: &Location, asset: &Asset) -> bool {
		let (o, a) = T::get();
		a.matches(asset) && &o == origin
	}
}

parameter_types! {
	pub RelayTokenForRelay: (Location, AssetFilter) = (Parent.into(), Wild(AllOf { id: AssetId(Parent.into()), fun: WildFungible }));
}

pub type TrustedLockers = TrustedLockerCase<RelayTokenForRelay>;

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Everything;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Nothing;
	type XcmReserveTransferFilter = Everything;
	type Weigher = weigher::Weigher;
	type UniversalLocation = constants::UniversalLocation;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type TrustedLockers = TrustedLockers;
	type SovereignAccountOf = location_converter::LocationConverter;
	type MaxLockers = ConstU32<8>;
	type MaxRemoteLockConsumers = ConstU32<0>;
	type RemoteLockConsumerIdentifier = ();
	type WeightInfo = pallet_xcm::TestWeightInfo;
	type AdminOrigin = EnsureRoot<AccountId>;
}

type Block = frame_system::mocking::MockBlock<Runtime>;

#[frame_support::runtime]
mod runtime {
	use frame_support::{Instance1, Instance2};
	
	#[runtime::runtime]
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeSlashReason,
		RuntimeLockId,
		RuntimeTask
	)]
	pub struct Runtime;

	#[runtime::pallet_index(0)]
	pub type System = frame_system;

	#[runtime::pallet_index(1)]
	pub type Balances = pallet_balances;

	#[runtime::pallet_index(2)]
	pub type Assets = pallet_assets<Instance1>;

	#[runtime::pallet_index(3)]
	pub type MessageQueue = mock_message_queue;

	#[runtime::pallet_index(4)]
	pub type PolkadotXcm = pallet_xcm;

	#[runtime::pallet_index(5)]
	pub type Uniques = pallet_uniques<Instance1>;

	#[runtime::pallet_index(6)]
	pub type ForeignUniques = pallet_uniques<Instance2>;

	#[runtime::pallet_index(7)]
	pub type ForeignAssets = pallet_assets<Instance2>;
}
