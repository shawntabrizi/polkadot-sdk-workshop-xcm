mod genesis;

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
        pallets = {}
    }
}
