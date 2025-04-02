use emulated_integration_tests_common::build_genesis_storage;
use sp_runtime::Storage;

pub fn genesis() -> Storage {
	let genesis_config = parachain_runtime::RuntimeGenesisConfig {
		parachain_info: parachain_info::GenesisConfig {
			parachain_id: 2000.into(),
			..Default::default()
		},
		..Default::default()
	};

	build_genesis_storage(
		&genesis_config,
		parachain_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
	)
}
