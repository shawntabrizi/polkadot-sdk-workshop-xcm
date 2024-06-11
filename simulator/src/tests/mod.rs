//! We're going to build asset hub, step-by-step.

#[cfg(feature = "relay-token")]
mod relay_token;

#[cfg(feature = "other-parachain-tokens")]
mod other_parachain_tokens;

#[cfg(feature = "register-assets")]
mod register_assets;
