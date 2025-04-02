mod genesis;

use emulated_integration_tests_common::{
	impl_accounts_helpers_for_parachain, impl_foreign_assets_helpers_for_parachain,
};
use frame_support::traits::OnInitialize;
use xcm_emulator::{decl_test_parachains, Parachain};

// AssetHubWestend Parachain declaration
decl_test_parachains! {
	pub struct AssetHubWestend {
		genesis = genesis::genesis(),
		on_init = {
			asset_hub_westend_runtime::AuraExt::on_initialize(1);
		},
		runtime = asset_hub_westend_runtime,
		core = {
			XcmpMessageHandler: asset_hub_westend_runtime::XcmpQueue,
			LocationToAccountId: asset_hub_westend_runtime::xcm_config::LocationToAccountId,
			ParachainInfo: asset_hub_westend_runtime::ParachainInfo,
			MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
		},
		pallets = {
			System: asset_hub_westend_runtime::System,
			PolkadotXcm: asset_hub_westend_runtime::PolkadotXcm,
			Balances: asset_hub_westend_runtime::Balances,
			Assets: asset_hub_westend_runtime::Assets,
			ForeignAssets: asset_hub_westend_runtime::ForeignAssets,
			PoolAssets: asset_hub_westend_runtime::PoolAssets,
			AssetConversion: asset_hub_westend_runtime::AssetConversion,
		}
	},
}

impl_foreign_assets_helpers_for_parachain!(AssetHubWestend, xcm::v5::Location);
impl_accounts_helpers_for_parachain!(AssetHubWestend);
