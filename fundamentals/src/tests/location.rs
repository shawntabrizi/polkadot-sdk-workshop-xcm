use crate::location::*;
use xcm::latest::prelude::*;

#[test]
fn relative_to_polkadot_para_1000_locations() {
	use relative_to_polkadot_para_1000::*;
	assert_eq!(PolkadotPara1000::get(), Here.into());
	assert_eq!(PolkadotPara2004::get(), (Parent, Parachain(2004)).into());
	assert_eq!(PolkadotRelay::get(), Parent.into());
	assert_eq!(
		PolkadotPara1000Alice::get(),
		Location::new(1, [Parachain(1000), AliceBytes::get().into()])
	);
	assert_eq!(PolkadotRelayBalancesPallet::get(), Location::new(1, [PalletInstance(5)]));
	assert_eq!(
		PolkadotPara1000Asset1984::get(),
		Location::new(0, [PalletInstance(50), GeneralIndex(1984)])
	);
	assert_eq!(KusamaPara1000::get(), Location::new(2, [GlobalConsensus(Kusama), Parachain(1000)]));
}

#[test]
fn relative_to_polkadot_relay_locations() {
	use relative_to_polkadot_relay::*;
	assert_eq!(PolkadotPara1000::get(), Location::new(0, [Parachain(1000)]));
	assert_eq!(PolkadotPara2004::get(), Location::new(0, [Parachain(2004)]));
	assert_eq!(PolkadotRelay::get(), Location::new(0, []));
	assert_eq!(
		PolkadotPara1000Alice::get(),
		Location::new(0, [Parachain(1000), AliceBytes::get().into()])
	);
	assert_eq!(PolkadotRelayBalancesPallet::get(), Location::new(0, [PalletInstance(5)]));
	assert_eq!(
		PolkadotPara1000Asset1984::get(),
		Location::new(0, [Parachain(1000), PalletInstance(50), GeneralIndex(1984)])
	);
	assert_eq!(KusamaPara1000::get(), Location::new(1, [GlobalConsensus(Kusama), Parachain(1000)]));
}

#[test]
fn absolute_locations() {
	use absolute::*;
	assert_eq!(
		PolkadotPara1000::get(),
		Location::new(0, [GlobalConsensus(Polkadot), Parachain(1000)])
	);
	assert_eq!(
		PolkadotPara2004::get(),
		Location::new(0, [GlobalConsensus(Polkadot), Parachain(2004)])
	);
	assert_eq!(PolkadotRelay::get(), Location::new(0, [GlobalConsensus(Polkadot)]));
	assert_eq!(
		PolkadotPara1000Alice::get(),
		Location::new(0, [GlobalConsensus(Polkadot), Parachain(1000), AliceBytes::get().into()])
	);
	assert_eq!(
		PolkadotRelayBalancesPallet::get(),
		Location::new(0, [GlobalConsensus(Polkadot), PalletInstance(5)])
	);
	assert_eq!(
		PolkadotPara1000Asset1984::get(),
		Location::new(
			0,
			[GlobalConsensus(Polkadot), Parachain(1000), PalletInstance(50), GeneralIndex(1984)]
		)
	);
	assert_eq!(KusamaPara1000::get(), Location::new(0, [GlobalConsensus(Kusama), Parachain(1000)]));
}

use sp_runtime::AccountId32;
const ALICE_BYTES: [u8; 32] = [1u8; 32];
const ALICE_ACCOUNT: AccountId32 = AccountId32::new(ALICE_BYTES);

#[test]
fn extract_last_account_id_works() {
	use manipulation::extract_last_account_id;

	let tests: Vec<(Location, Option<AccountId32>)> = vec![
		(Here.into(), None),
		(
			Location::new(0, [GlobalConsensus(Polkadot), Parachain(1000), ALICE_BYTES.into()]),
			Some(ALICE_ACCOUNT),
		),
	];

	for (loc, res) in tests {
		assert_eq!(extract_last_account_id(loc), res);
	}
}

#[test]
fn check_sibling_parachains_works() {
	use manipulation::check_sibling_parachains;

	let tests: Vec<(Location, Option<u32>)> = vec![
		(Here.into(), None),
		(Location::new(0, [Parachain(1000)]), None),
		(Location::new(0, [Parachain(1000)]), None),
		(Location::new(1, [Parachain(1000)]), Some(1000)),
		(Location::new(0, [Parachain(1000), ALICE_BYTES.into()]), None),
	];

	for (loc, res) in tests {
		assert_eq!(check_sibling_parachains(loc), res);
	}
}
