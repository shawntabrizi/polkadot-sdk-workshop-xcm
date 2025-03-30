mod genesis;

// Cumulus
use xcm_emulator::decl_test_relay_chains;

// Westend declaration
decl_test_relay_chains! {
	#[api_version(13)]
	pub struct Westend {
		genesis = genesis::genesis(),
		on_init = (),
		runtime = westend_runtime,
		core = {
			SovereignAccountOf: westend_runtime::xcm_config::LocationConverter,
		},
		pallets = {
			XcmPallet: westend_runtime::XcmPallet,
			Sudo: westend_runtime::Sudo,
			Balances: westend_runtime::Balances,
			Treasury: westend_runtime::Treasury,
			AssetRate: westend_runtime::AssetRate,
			Hrmp: westend_runtime::Hrmp,
			Identity: westend_runtime::Identity,
			IdentityMigrator: westend_runtime::IdentityMigrator,
		}
	},
}
