use crate::{
    westend::Westend,
    asset_hub_westend::AssetHubWestend,
};

use emulated_integration_tests_common::accounts::{ALICE, BOB};
use xcm_emulator::{decl_test_networks, decl_test_sender_receiver_accounts_parameter_types};

decl_test_networks! {
    pub struct WestendNetwork {
        relay_chain = Westend,
        parachains = vec![
            AssetHubWestend,
        ],
        bridge = ()
    }
}

decl_test_sender_receiver_accounts_parameter_types! {
    WestendRelay { sender: ALICE, receiver: BOB },
    AssetHubWestendPara { sender: ALICE, receiver: BOB }
}
