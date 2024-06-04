/// Lesson 1
#[cfg(feature = "location")]
pub mod location;

/// Lesson 2
#[cfg(feature = "asset")]
pub mod asset;

/// Lesson 3
#[cfg(feature = "holding")]
pub mod holding;

/// Lesson 4
#[cfg(feature = "instruction")]
pub mod instruction;

/// Lesson 5
#[cfg(feature = "xcm_executor")]
pub mod xcm_executor;

/// Lesson 6
#[cfg(feature = "pallet_xcm")]
pub mod pallet_xcm;

/// Mock network for running the `pallet_xcm` tests.
#[cfg(feature = "pallet_xcm")]
pub mod network;

/// Constants for accounts.
mod constants;

#[cfg(test)]
mod tests;
