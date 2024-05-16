use crate::fundamentals::location::*;
use xcm::latest::prelude::*;

#[test]
fn relative_to_polkadot_para_1000_locations() {
    use relative_to_polkadot_para_1000::*;
    assert_eq!(PolkadotPara1000::get(), Here.into());
    assert_eq!(PolkadotPara1337::get(), (Parent, Parachain(1337)).into());
    assert_eq!(PolkadotRelay::get(), Parent.into());
    assert_eq!(PolkadotPara1337Alice::get(), Location::new(1, [Parachain(1337), AliceBytes::get().into()]));
    assert_eq!(PolkadotRelayBalancesPallet::get(), Location::new(1, [PalletInstance(1)]));
    assert_eq!(PolkadotPara1000Asset1984::get(), Location::new(0, [PalletInstance(2), GeneralIndex(1984)]));
    assert_eq!(KusamaPara69::get(), Location::new(2, [GlobalConsensus(Kusama), Parachain(69)]));
}

#[test]
fn relative_to_polkadot_relay_locations() {
    use relative_to_polkadot_relay::*;
    assert_eq!(PolkadotPara1000::get(), Location::new(0, [Parachain(1000)]));
    assert_eq!(PolkadotPara1337::get(), Location::new(0, [Parachain(1337)]));
    assert_eq!(PolkadotRelay::get(), Location::new(0, []));
    assert_eq!(PolkadotPara1337Alice::get(), Location::new(0, [Parachain(1337), AliceBytes::get().into()]));
    assert_eq!(PolkadotRelayBalancesPallet::get(), Location::new(0, [PalletInstance(1)]));
    assert_eq!(PolkadotPara1000Asset1984::get(), Location::new(0, [Parachain(1000), PalletInstance(2), GeneralIndex(1984)]));
    assert_eq!(KusamaPara69::get(), Location::new(1, [GlobalConsensus(Kusama), Parachain(69)]));
}

#[test]
fn absolute_locations() {
    use absolute::*;
    assert_eq!(PolkadotPara1000::get(), Location::new(0, [GlobalConsensus(Polkadot), Parachain(1000)]));
    assert_eq!(PolkadotPara1337::get(), Location::new(0, [GlobalConsensus(Polkadot), Parachain(1337)]));
    assert_eq!(PolkadotRelay::get(), Location::new(0, [GlobalConsensus(Polkadot)]));
    assert_eq!(PolkadotPara1337Alice::get(), Location::new(0, [GlobalConsensus(Polkadot), Parachain(1337), AliceBytes::get().into()]));
    assert_eq!(PolkadotRelayBalancesPallet::get(), Location::new(0, [GlobalConsensus(Polkadot), PalletInstance(1)]));
    assert_eq!(PolkadotPara1000Asset1984::get(), Location::new(0, [GlobalConsensus(Polkadot), Parachain(1000), PalletInstance(2), GeneralIndex(1984)]));
    assert_eq!(KusamaPara69::get(), Location::new(0, [GlobalConsensus(Kusama), Parachain(69)]));
}
