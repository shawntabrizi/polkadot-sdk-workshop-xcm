mod westend;
mod asset_hub_westend;
mod parachain;
mod network;

pub mod prelude {
    use super::*;

    pub use network::{
        WestendRelay as Westend,
        WestendRelaySender as WestendSender,
        WestendRelayReceiver as WestendReceiver,
        AssetHubWestendPara as AssetHubWestend,
        AssetHubWestendParaSender as AssetHubWestendSender,
        AssetHubWestendParaReceiver as AssetHubWestendReceiver,
        CustomPara,
        CustomParaSender,
        CustomParaReceiver,
    };

    pub use westend::WestendRelayPallet as WestendPallet;
    pub use asset_hub_westend::AssetHubWestendParaPallet as AssetHubWestendPallet;
    pub use parachain::CustomParaPallet as CustomPallet;
}
