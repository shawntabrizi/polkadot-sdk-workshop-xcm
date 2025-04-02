/// Lesson 1
#[cfg(feature = "location")]
pub mod location;

/// Lesson 2
#[cfg(feature = "asset")]
pub mod asset;

/// Lesson 3
#[cfg(feature = "instruction")]
pub mod instruction;

/// Constants for accounts.
mod constants;

#[cfg(test)]
mod tests;
