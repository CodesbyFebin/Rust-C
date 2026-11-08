# Contributing to Rust-C

## Quick Start
```bash
git clone https://github.com/CodesbyFebin/Rust-C.git
cd Rust-C
cargo test
cargo run -- vil examples/defensive.vil
cargo run -- commit examples/defensive.vil examples/main_rc.rs
```

## Hygiene Rules (must read)
- 100% Rust. No JavaScript, no Node, no mocks claiming proof.
- Deterministic: same input = same output. No floats, no wall-clock, no IO in execution.
- Fail-closed: if Envelope / VIL / Rust-C / TEE / zkVM / SIP / ATP / x402 / git fails, no commit. No commit, no money.
- Never claim TEE ran, zkVM proved, or payment happened unless you have real proof. In this repo `starkVerified=false`, escrow stays `HELD`.
- `settle()` is banned in VIL. Use `escrow()`.
- Keep `target/` out: `.gitignore` already covers it. Never `git add .` from `~`.

## Branch + Commit
- Branch: `feat/your-feature`, `fix/your-fix`, `docs/your-change`
- Commits: Conventional Commits — `feat:`, `fix:`, `docs:`, `test:`, `chore:`
- Example: `fix: remove moved bytes in envelope verify`

## PR Checklist
- [ ] `cargo test` passes (6 tests)
- [ ] `cargo run -- commit` produces a receipt with `escrowStatus: HELD`, `starkVerified: false`
- [ ] No `target/`, no secrets, no `.env` with real values
- [ ] README claims match code — if you add a claim, add a test or proof
