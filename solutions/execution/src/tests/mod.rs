//! The tests.

// Tests with the full parachain configuration.
// They showcase various features of XCM.
mod full;

// Tests that show how configuring different asset transactors change the behaviour
// of XCM.
mod asset_transactor;
// Tests for configuring different barriers.
mod barrier;
// Tests for configuring different reserves and teleporters.
mod reserves_and_teleports;

// Common helpers used throughout the tests.
mod common;
