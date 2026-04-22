//! # Fundamentals Lesson 3
//!
//! Instructions are the fundamental building block of XCM programs.
//! Let's look at the most basic ones.

#![allow(unused_imports, unused_variables)]

use xcm::latest::prelude::*;

use crate::constants::ALICE;

/// ✅ Worked example — a message containing only a single `ClearOrigin` instruction.
/// `ClearOrigin` strips the sender's privileges for the rest of the program.
/// An `Xcm<Call>` wraps a `Vec<Instruction<Call>>`.
pub fn clear_origin_message() -> Xcm<()> {
	Xcm(vec![ClearOrigin])
}

/// Put your asset knowledge to the test.
/// Return an XCM that withdraws 100 planks of the relay's native token (DOT).
/// The program is assumed to execute on a parachain; it doesn't need to do anything
/// with the funds yet.
///
/// Hint: there's an instruction for taking assets from the sender and putting them
/// into the holding register.
pub fn withdraw_asset() -> Xcm<()> {
	todo!()
}

/// Let's do something with the withdrawn funds.
/// Return an XCM that withdraws the same 100 planks, then deposits everything currently
/// in the holding register to `ALICE`.
///
/// Hints:
///   - A wildcard can refer to "all the assets we just withdrew" without restating them.
///   - `ALICE` is a 32-byte array; the beneficiary needs to be a `Location`.
pub fn withdraw_and_deposit() -> Xcm<()> {
	todo!()
}

/// Real XCMs have to pay for their execution. Return an XCM that withdraws 100 planks,
/// pays up to 10% of that for execution, and deposits the rest to `ALICE`.
///
/// Try writing this one with the builder API: `Xcm::builder().withdraw_asset(...).<...>.build()`.
///
/// Hint: three instructions — withdraw, pay for execution, deposit. There is a dedicated
/// builder method for each one.
pub fn withdraw_and_deposit_paying_fees() -> Xcm<()> {
	todo!()
}
