//! # XCM Fundamentals
//!
//! Students will first go through, learn, and use all the fundamental building blocks for XCM:
//! 
//! 1. [Location / Topography](location)
//! 	- Learn how to construct relative and absolute locations for common objects and types used in XCM.
//! 2. [Assets and Filters](asset)
//! 	- Learn how to represent various types of assets like fungible tokens and non-fungible tokens.
//! 	- Constructing asset filters to target pools of assets.
//! 3. [Asset Holding](holding)
//! 	- Learn how we can manage multiple assets in memory using the `AssetsInHolding` abstraction.
//! 4. [Instructions](instruction)
//! 	- Construct common XCM messages through individual XCM instructions.
//! 5. [The XCM Executor](xcm_executor)
//! 	- Learn how the XCM Executor actually functions, and loosely implement a few common instructions needed to complete end to end scenarios.
//! 6. [Pallet XCM](pallet_xcm)
//! 	- Learn how Pallet XCM provides a simple to access wrapper to the underlying XCM Executor to perform common tasks like send, execute, and teleport transfers.
//!
//! NOTE: If you can't see the docs for some lesson, compile them with `--all-features`.

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

/// Constants for accounts.
mod constants;

#[cfg(test)]
mod tests;
