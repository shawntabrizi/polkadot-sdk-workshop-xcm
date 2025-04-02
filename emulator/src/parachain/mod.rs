mod genesis;

use emulated_integration_tests_common::impl_foreign_assets_helpers_for_parachain;
use xcm_emulator::decl_test_parachains;

decl_test_parachains! {
	pub struct Custom {
		genesis = genesis::genesis(),
		on_init = {},
		runtime = parachain_runtime,
		core = {
			XcmpMessageHandler: parachain_runtime::XcmpQueue,
			LocationToAccountId: parachain_runtime::configs::xcm_config::LocationToAccountId,
			ParachainInfo: parachain_runtime::ParachainInfo,
			MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
		},
		pallets = {
			Balances: parachain_runtime::Balances,
			ForeignAssets: parachain_runtime::ForeignAssets,
			PolkadotXcm: parachain_runtime::PolkadotXcm,
		}
	}
}

impl_foreign_assets_helpers_for_parachain!(Custom, xcm::v5::Location);
