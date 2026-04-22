- [ ] how to allow account on relay chain control the same account on parachain
- [ ] how to allow an account on parachain to control the same account on another parachain
- [ ] make note of `Xcm<()>` vs `Xcm<T::RuntimeCall>`

## Rethink tutorial from first principles

Core insight: XCM is a stack machine that runs across multiple consensus systems.
Most teaching failures trace to that being invisible — students read an instruction
list with no mental model of the registers (holding, origin, fees, topic) changing
underneath. Fix observability and most of the rest follows.

### Per-topic ideas

- [ ] **Locations** — build a small interactive visual tool: draw a tree of
  chains/pallets/accounts, click any node, see its `Location` from any perspective.
  Flip perspective with a dropdown. Ten minutes in that tool teaches more than an
  hour of `parameter_types!`. Current ASCII topography diagrams are the right
  instinct but static.
- [ ] **Assets** — collapse the concept: `Asset = (Location, Fungibility)`. Once
  Locations click, writing an Asset is trivial. Treat filters as a separate small
  lesson of pattern-matching puzzles, not Rust.
- [ ] **Instructions** — build an observable executor / step debugger. Feed it an
  XCM, show holding / origin / fees / error state after each instruction. Existing
  `RUST_LOG=xcm=trace` dumps it to a log; a TUI or web renderer would transform it.
  Every instruction lesson then becomes "predict the next state, step, check."
  Kills the "why is `BuyExecution` here" mystery by showing failure without it.
- [ ] **XCM programs** — teach failure-first. Hand students a *broken* program
  with a test message like "WithdrawAsset failed: Barrier". Fixing the error
  teaches the instruction. Each exercise removes one thing. Matches real XCM
  debugging and leaves students able to read error logs.
- [ ] **Config** — teach it as "trust as code." Frame each config lesson as a
  threat: "Here's an XCM a hostile sibling could send. With the current config it
  succeeds and drains your treasury. Change one type to stop it." Barriers,
  IsReserve, IsTeleporter all become concrete. End with a diff between a real
  Asset Hub config and a custom parachain: what's different and why?

### Cross-cutting

- [ ] **Real XCMs from the wild** — pull actual XCMs from Subscan / Polkassembly
  as decode exercises: "What did this do, and why did it need each instruction?"
- [ ] **Progressive instruction reveal** — introduce ~5 core instructions first
  (Withdraw/Deposit/BuyExecution/ClearOrigin/Transact). Others appear only when a
  scenario needs them.
- [ ] **Cheatsheet, not prose** — "to do X, you need instructions Y" beats a wall
  of markdown.
- [ ] **One artifact to rule them all** — if we can only build one thing, it's
  the observable-executor stepper. Everything else is content around it.

### Tradeoff

Most of this is tooling work, not content work. Minimum viable rewrite: keep
test-driven TODOs but frame every lesson as "failure → fix" instead of "fill in
the blank." Bigger investment: build the XCM stepper; it would change how anyone
learns this.

