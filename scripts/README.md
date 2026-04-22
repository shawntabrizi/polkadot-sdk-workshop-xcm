# scripts

One script today, potentially more over time. Everything here is meant to be runnable from the repo root.

## `check-solutions.sh`

Verifies that every file under [`../solutions/`](../solutions) still compiles and passes tests against the currently-pinned `polkadot-sdk`. Used by CI and recommended before any SDK-pin bump.

### What it does

1. Enumerates every file under `solutions/`.
2. For each, backs up the matching in-place file (e.g. `solutions/fundamentals/src/location.rs` → backup of `fundamentals/src/location.rs`) to a temp dir.
3. Overwrites the in-place file with the solution version.
4. Runs `cargo test --workspace` (additional `cargo test` args can be passed through: `./scripts/check-solutions.sh --no-run` etc).
5. Restores in-place files from the backup on exit — passes, failures, and Ctrl+C all trigger the restore via a bash `trap`.

### When to run it

- **Before bumping the SDK pin.** If a solution breaks on a newer `polkadot-sdk` rev, this catches it before you merge the bump.
- **After adding a new solution file.** Confirms it matches the starter's structure and passes the corresponding test.
- **Debugging CI red.** Reproduces the exact CI invocation locally.

### Requirements

- Bash (works on bash 3.x / macOS default — no `mapfile` dependency).
- Access to the standard cargo toolchain as declared in `rust-toolchain.toml`.
- Clean working tree is *not* required — the backup is in-memory of the script's lifetime, so local uncommitted changes to lesson files are preserved.

### Exit codes

- `0` — all tests pass with solutions swapped in.
- non-zero — a test failed, or `solutions/` is out of sync with the in-place layout.

## CI integration

[`.github/workflows/ci.yml`](../.github/workflows/ci.yml) runs this script in the `check-solutions` job on every push to `master` and every PR.
