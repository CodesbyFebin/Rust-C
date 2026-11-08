# ShipOS Audit — Corrections (do not use ShipOS README as-is)

ShipOS scored 48/100 on commit 00aa967. Its missing-file fixes are correct.
Its language + proof claims are wrong and must NOT be merged:

| ShipOS claim | Truth |
|---|---|
| Primary language JavaScript 82% / Rust 18% | WRONG. Repo is 100% Rust. No JS, no Node. `src/*.rs` + `Cargo.toml` only. |
| Every function call emits a TEE/zkVM proof | WRONG. Hygienic: TEE proof must be supplied. No proof = no commit. `starkVerified=false` always in this repo. |
| Tests passing | Was FALSE at 00aa967 (envelope.rs E0382). Fixed in next commit: removed moved `arr` line. |
| `cargo install` to start | WRONG. Use `cargo test` / `cargo run -- vil` / `cargo run -- commit`. |

Keep the 9,806-word README.md in this repo. It is the canonical README.
Add CI + SECURITY + CONTRIBUTING + Code of Conduct from this patch — those are the real +10.5 / +3 pt fixes.
