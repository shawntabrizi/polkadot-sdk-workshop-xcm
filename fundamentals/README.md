# fundamentals

Lessons 1–3. This crate is pure Rust — no emulator, no runtime — so it compiles fast and isolates the XCM primitives (`Location`, `Asset`, `Instruction`) from everything else.

## What's here

| File | Lesson | Role |
|---|---|---|
| [`src/location.rs`](src/location.rs) | 1 — Locations | **Edit this.** Fill in the `parameter_types!` blocks so each named location matches its description from the right perspective. |
| [`src/asset.rs`](src/asset.rs) | 2 — Assets | **Edit this.** Construct `AssetId`, `Asset`, and `AssetFilter` values. |
| [`src/instruction.rs`](src/instruction.rs) | 3 — Instructions | **Edit this.** Write small XCM programs out of primitive instructions. |
| `src/tests/{location,asset,instruction}.rs` | — | **Read-only.** Tests that gate each lesson. Don't edit unless you find a bug. |
| `src/constants.rs` | — | **Read-only.** Defines `ALICE`/`BOB` 32-byte accounts used throughout. |
| `src/lib.rs` | — | **Read-only.** Feature-gated module wiring. |

## Running lessons

Each lesson compiles and tests independently via a cargo feature so you can isolate failures:

```sh
cargo test -p fundamentals --no-default-features --features location
cargo test -p fundamentals --no-default-features --features asset
cargo test -p fundamentals --no-default-features --features instruction
```

All three at once (sequential, first-fail-stop):

```sh
../test_fundamentals.sh   # convenience wrapper
```

## How the tests behave in starter state

Lesson files use `todo!()` as placeholders inside `parameter_types!` and `pub fn` bodies. `cargo check` is green; `cargo test` panics with `not yet implemented` on the first unresolved TODO. Each panic names the exact value you still need to define.

## Reference solutions

Mirror path: [`../solutions/fundamentals/src/`](../solutions/fundamentals/src/). Try to resist peeking — the hints in each file's comments are designed to get you unstuck without the full answer.
