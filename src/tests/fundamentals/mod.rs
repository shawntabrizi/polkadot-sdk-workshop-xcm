// Lesson 1
#[cfg(feature = "location")]
pub mod location;

// Lesson 2
#[cfg(feature = "asset")]
pub mod asset;

// Lesson 3
#[cfg(feature = "instruction")]
pub mod instruction;

// Lesson 4
#[cfg(all(feature = "holding", not(feature = "xcm_executor"), not(feature = "pallet_xcm")))]
pub mod holding;

// Lesson 5
#[cfg(all(feature = "xcm_executor", not(feature = "pallet_xcm")))]
pub mod xcm_executor;

// Lesson 6
#[cfg(feature = "pallet_xcm")]
pub mod pallet_xcm;
