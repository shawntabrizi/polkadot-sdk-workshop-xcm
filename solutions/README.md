# solutions

Reference answers for every lesson. The directory structure mirrors the in-place lesson-file paths exactly:

```
solutions/
├── fundamentals/src/
│   ├── location.rs              ↔ ../../fundamentals/src/location.rs
│   ├── asset.rs                 ↔ ../../fundamentals/src/asset.rs
│   └── instruction.rs           ↔ ../../fundamentals/src/instruction.rs
├── execution/src/tests/         ↔ ../../execution/src/tests/
│   ├── full.rs                  (all 5 tests fully written, incl. transfer_n_times)
│   ├── asset_transactor.rs      (identical to in-place — it's a gate, not a lesson)
│   ├── barrier.rs               (identical to in-place)
│   ├── reserves_and_teleports.rs(identical to in-place)
│   ├── common.rs, mod.rs, weigher.rs
└── parachain/src/configs/xcm/   ↔ ../../parachain/src/configs/xcm/
    ├── asset_transactor.rs
    ├── barrier.rs
    └── reserves_and_teleports.rs
```

## Try to resist peeking

Every lesson file has hints written into its header comments. The hints are designed to unblock you without handing over the answer. Looking here should be a last resort.

If you do peek, read the solution once, close the file, then write your own — don't copy-paste. The value of the workshop is in the act of reaching the solution.

## How this is used mechanically

[`scripts/check-solutions.sh`](../scripts/check-solutions.sh) is the one tool that reads this directory. It:

1. Backs up every in-place lesson file to a temp dir.
2. Overwrites each in-place lesson file with its `solutions/` counterpart.
3. Runs `cargo test --workspace`.
4. On exit (pass, fail, or Ctrl+C), restores from the backup.

CI runs this on every PR — if any solution breaks against the current SDK pin, the CI job surfaces which lesson is affected.

## Maintaining solutions

Whenever you change a lesson file's surrounding context (helper types, imports, test harness) such that the existing solution no longer compiles or passes, update the matching file here in the same commit. Otherwise the `check-solutions` CI job will go red on the next push.

Solution files are fully standalone — they don't include scaffolding comments, hints, or TODO markers. They should compile cleanly on their own when copied over the starter.
