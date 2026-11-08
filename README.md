# Rust-C — The First Version Control Where a Commit Is Proof of Execution

[Rust](https://img.shields.io/badge/language-100%25%20Rust-orange)
[VIL](https://img.shields.io/badge/VIL-0.2-blue)
[License](https://img.shields.io/badge/license-MIT-green)
[Hygiene](https://img.shields.io/badge/hygiene-no%20mocks-brightgreen)
[Tests](https://img.shields.io/badge/tests-cargo%20test-passing)
[Deterministic](https://img.shields.io/badge/deterministic-same%20input%20same%20output-blueviolet)

> **Git tracks code. GitHub tracks collaboration. Rust-C tracks code + intent + proof + payment.**
>
> A Rust-C commit is not a hash of files. It is a hash of authorization + intent + deterministic execution + attestation + proof + state + transport + settlement + history. If any layer fails, there is no commit. No commit, no money. No proof, no merge.

**Repo:** `https://github.com/CodesbyFebin/Rust-C`
**VIL standalone:** `https://github.com/CodesbyFebin/VIL`
**Language:** 100% Rust. No JavaScript. No Node. No mocks.
**Status:** Hygienic v1.0 — fail-closed, deterministic, verifiable.

---


## Table of Contents

- [Why Rust-C Exists](#why-rust-c-exists)
- [The One-Sentence Differentiator](#the-one-sentence-differentiator)
- [Quickstart in 60 Seconds](#quickstart-in-60-seconds)
- [What Is VIL?](#what-is-vil)
- [What Is Rust-C?](#what-is-rust-c)
- [The 9 Layers, Explained](#the-9-layers-explained)
- [A Complete Example, End to End](#a-complete-example-end-to-end)
- [How Verification Works Without Re-Execution](#how-verification-works-without-re-execution)
- [Hygiene: What We Keep and What We Leave Out](#hygiene-what-we-keep-and-what-we-leave-out)
- [Architecture](#architecture)
- [Security Model and Threat Model](#security-model-and-threat-model)
- [Determinism: Same Input, Same Output, Always](#determinism-same-input-same-output-always)
- [Comparison: Git vs GitHub vs Rust-C vs Blockchains vs Agent Frameworks](#comparison-git-vs-github-vs-rust-c-vs-blockchains-vs-agent-frameworks)
- [Use Cases](#use-cases)
- [For AI Agents, For Humans, For Auditors](#for-ai-agents-for-humans-for-auditors)
- [Benchmarks and Honest Limits](#benchmarks-and-honest-limits)
- [FAQ — 40 Questions Answered](#faq--40-questions-answered)
- [Roadmap](#roadmap)
- [Contributing — Whitehat Only](#contributing--whitehat-only)
- [How This Repo Trends — Whitehat Playbook](#how-this-repo-trends--whitehat-playbook)
- [SEO and Discoverability — How This README Is Built to Rank](#seo-and-discoverability--how-this-readme-is-built-to-rank)
- [Changelog](#changelog)
- [License](#license)

---


## Why Rust-C Exists

By 2030, agents will write most code. That is not a prediction anymore. It is a trajectory. Every major lab, every major platform, every major enterprise is moving toward agent-written code, agent-executed tasks, agent-to-agent payments, and agent-managed infrastructure.

Git was built for humans. Git assumes the author can be trusted to say, truthfully, "I did this." Git assumes the committer is accountable, reachable, and has a reputation to lose. Git assumes a human reviewed the diff, understood the intent, and approved the merge.

None of those assumptions hold for agents.

An agent can hallucinate. An agent can misinterpret intent. An agent can claim it did work it did not do. An agent can exfiltrate data while doing the work. An agent can be prompt-injected by a third party. An agent can be a copy of a copy of a copy, with no reputation, no accountability, and no memory of what it did yesterday.

If agents write most code, we need a version control system built for agents. Not a nicer GitHub UI. Not a better prompt. Not a better review bot. A new commit format, a new verification model, and a new settlement model.

Rust-C is that system.

Rust-C answers three questions that Git cannot answer:

1. **Did the agent do what the human meant?** Not what the prompt said, not what the agent thought the prompt said, but what the human meant. That is intent. In Rust-C, intent is written as VIL, compiled, hashed, and bound to the output. If the output does not match the intent, verification fails.

2. **Did the agent actually do the work, correctly, in a trusted environment?** Not did it claim to do the work, but did it run the deterministic code in a trusted execution environment, with a proof that can be checked without re-running the work. That is Rust-C plus TEE plus zkVM linkage.

3. **Should the agent be paid for this work?** Not should it be paid because it said it finished, but should it be paid because the work was verified. In Rust-C, payment is escrowed before the work, held during the work, and released only on verification. No verification, no money. No merge, no money.

Those three questions are the missing layer between version control and the agent economy. Rust-C is that layer.

This README is long on purpose. It is long because GitHub ranks long, structured, keyword-rich, example-rich READMEs higher in search. It is long because new contributors need to understand the system without reading ten separate docs. It is long because auditors need to see the threat model, the limits, and the honest labels. And it is long because a repo that trends is a repo that teaches. If you came here from trending, welcome. Start with Quickstart, then read VIL, then read the example. You can run everything in under two minutes with `cargo test`.

---


## The One-Sentence Differentiator

In Git, a commit is `hash(files + message + parent)`.

In Rust-C, a commit is:

```
hash(
  Envelope(authorization)
+ VIL(intent = contract)
+ Rust-C(deterministic execution)
+ TEE(attestation)
+ zkVM(proof linkage)
+ SIP(state)
+ ATP(transport)
+ x402(settlement)
+ git(history)
)
```

If any layer fails, there is no commit. That is the differentiator. No other version control system, no other agent framework, no other payment protocol does this in one commit.

This is not a marketing claim. It is a code claim. You can read `src/gitrust.rs`. You can read the tests. You can run `cargo test` and see the fail-closed behavior: no TEE proof, no commit. Over-budget policy, no commit. Tampered seal, no commit. Tampered git chain, no settlement. The differentiator is enforced by code, not by copy.

---


## Quickstart in 60 Seconds

This repo is 100% Rust. You need Rust and Cargo. No Node. No npm. No Python.

```bash
git clone https://github.com/CodesbyFebin/Rust-C.git
cd Rust-C
cargo test
```

That runs all tests. Every test is deterministic. Same inputs, same outputs, every time.

```bash
cargo run -- vil examples/defensive.vil
# intent process_sales_data hash 8bc8...
# execute total 400 commitment 80ad...
# verify ok total 400

cargo run -- build examples/main.rc.rs --out out.wasm
# built 40 bytes

cargo run -- commit examples/defensive.vil examples/main.rc.rs
# prints a receipt: gitCommit, vil commitment, rustc codeHash,
# tee supplied, zk linkageOk, sip root, atp ok, x402 HELD, total 400, verifiedHashChecks true
```

If you only have 60 seconds, run those three commands. You have now seen VIL compile, Rust-C compile, and a hygienic commit. The rest of this README explains why each step matters and how to use it in production.

For VIL alone, use the standalone repo:

```bash
git clone https://github.com/CodesbyFebin/VIL.git
cd VIL
cargo test
cargo run -- run examples/defensive.vil
```

VIL is the contract language inside Rust-C. It also stands alone. If you only care about intent, start with VIL. If you care about execution, payment, and history, start with Rust-C.

---


## What Is VIL?

VIL stands for Verifiable Intent Language. Version 0.2 is the current spec. VIL is the contract inside every Rust-C commit.

In most agent systems, intent is a prompt. A prompt is ambiguous. A prompt is not versioned. A prompt is not hashed. A prompt cannot be verified. A prompt can be misinterpreted in a hundred ways, and you only discover the misinterpretation after the agent has already spent money, changed code, or exfiltrated data.

In Rust-C, intent is VIL. VIL is human-readable, like a prompt, but it is also compilable, hashable, and verifiable, like code. VIL is written by a human, or by a human with an agent, reviewed like code, committed like code, and verified like code.

Here is the canonical VIL example. It is the same example in the tests, in the CLI, and in every doc. Learn this one example and you understand VIL:

```vil
intent process_sales_data(
  source: AttestedUrl,
  budget: Budget<max_cost: 0.05 USD, ttl: 1h>
) -> Sealed<Total>
  effects: [read: source, pay: budget]
  trust: zkvm(risczero) + tee-audit(sample: 0.01)
{
  let committed: Committed<RawBytes> = fetch(source) @TEE;
  let validated: Validated<SalesRecord> = validate(committed, SalesRecord) @zkvm(risczero);
  let total: Aggregated<Total> = aggregate(validated, sum(revenue)) @zkvm(risczero);
  let escrowId: Escrow = escrow(total, price: "$0.01", budget: budget) @x402;
  return seal(total, escrowId);
}
```

Read it line by line. The intent is `process_sales_data`. It takes a source and a budget. It returns a sealed total. Its effects are reading the source and paying the budget. Its trust is zkVM plus TEE audit. Its body fetches inside TEE, validates, aggregates only validated data, escrows payment, and seals the result.

Every line matters. Every line is checked.

VIL has five guarantees. All five are enforced at compile time or by hash. None require trust.

First, proof-carrying types. Data flows through types that carry proof. Raw bytes become committed bytes. Committed bytes become validated records. Validated records become aggregated totals. You cannot aggregate raw bytes. You cannot aggregate committed bytes. The compiler rejects it. The error is `PROOF_CARRYING_TYPE_VIOLATION`. This is not a style rule. It is a security rule. If you could aggregate raw bytes, you could aggregate unvalidated, untrusted, injected data. VIL prevents that at compile time.

Second, effects as types. Every VIL intent declares its effects. Reading, paying, writing, calling. If an agent tries to do an effect that is not declared, it is not just bad practice. It is a violation of the contract. Effects are not comments. Effects are types. They are checked.

Third, trust in signature. Every VIL intent declares its trust. Which zkVM, which TEE, which audit sample. Trust is a declaration, not a proof. This is an honest label. VIL does not claim the audit happened. It declares that the audit is required. The proof of the audit must be supplied separately. That separation is what makes VIL hygienic. A language that claims trust without proof is not hygienic. A language that declares trust and requires proof is.

Fourth, escrow, not settle. In naive agent systems, the agent calls `settle()` and gets paid. In VIL, `settle()` is banned. The compiler rejects it with `SETTLE_BANNED`. The only way to get paid is `escrow()`. Escrow holds the money. Escrow releases only on verification. Escrow reverts on proof failure. This is the money-follows-proof rule, enforced by the language.

Fifth, seal equals commitment plus proof. The output is not just a total. It is a sealed total. The seal is a hash of the total, the proof chain, the escrow id, and the intent hash. The verifier recomputes the seal. If the seal does not recompute, verification fails. The verifier never re-executes the aggregation. Verification is cheap. Execution is expensive. That asymmetry is intentional. It is what makes verification scalable.

VIL is small. It is intentionally small. A small language is easier to audit, easier to learn, and harder to exploit. VIL is not a general-purpose language. It is a contract language. It does one thing: it makes intent verifiable.

If you want to learn VIL deeply, read `docs/VIL.md` in the VIL repo and `examples/` in this repo. There are valid examples and invalid examples. The invalid examples are as important as the valid ones. They show what VIL rejects and why.

---


## What Is Rust-C?

Rust-C is a deterministic Rust subset. It is Rust, but with the nondeterminism removed.

Normal Rust can do anything. It can read files. It can access the network. It can get the time. It can generate random numbers. It can use floats. Floats are nondeterministic across platforms. Time is nondeterministic. Random is nondeterministic. I/O is nondeterministic. All of those are useful in normal Rust. All of those are fatal in verifiable Rust.

Rust-C bans them. Rust-C allows only what is deterministic. Same source, same input, same output, on any machine, at any time, forever.

Here is the canonical Rust-C example:

```rust
fn aggregate(state: u32, input: u32) -> u32 {
  let total: u32 = state + input;
  total
}
```

That is it. A pure function. State plus input equals next state. No floats. No time. No I/O. No random. No side effects.

Rust-C enforces:

- Only `u32` arithmetic. No `f32`, no `f64`. Floats are banned because floats are not deterministic across platforms and compilers.
- No wall-clock. No `std::time`, no `SystemTime`, no `Date`. Time is an explicit input, not a hidden input.
- No randomness. No `rand`, no `Math.random`, no hidden seed. Randomness, if needed, is an explicit input, a seed passed in.
- No I/O. No `println!`, no `fs::`, no `net::`, no `std::io`. A Rust-C function cannot read a file, cannot call a network, cannot print. It can only compute.
- Entry point is `fn step(state: u32, input: u32) -> u32` or `fn aggregate(state: u32, input: u32) -> u32`. The compiler checks the signature. The wrong signature is a compile error.
- Wrapping arithmetic. `state + input` in Rust-C is wrapping arithmetic. It does not panic on overflow. It wraps. That is deterministic. Panic is nondeterministic in the sense that it depends on debug vs release, on platform, on configuration. Wrapping is always the same.

Why does determinism matter? Because verification depends on determinism. If execution is nondeterministic, you cannot prove what happened. You cannot reproduce it. You cannot audit it. You cannot check a receipt. A receipt for nondeterministic execution is not a receipt. It is a story.

In Rust-C, the receipt is real. The receipt is `sha256(DOMAIN || wasmDigest || nonce || state || input || seed || nextState)`. Same inputs, same receipt. Different inputs, different receipt. Anyone can check the receipt by recomputing the hash. No trust required.

Rust-C compiles to WASM. The WASM bytes are deterministic. Same source, same WASM. The WASM digest is part of the receipt. The WASM digest is part of the TEE measurement. The TEE measurement must equal the code hash. If the code changes, the measurement changes, the receipt changes, verification fails. That chain is what binds code to execution to proof.

Rust-C is not a new language. It is a subset. That matters. You do not need to learn a new language. You need to learn what to leave out. Leave out floats, time, random, I/O, and you have Rust-C. That is a small price for verifiability.

If you want to see what Rust-C rejects, look at the tests. Floats rejected. Time rejected. I/O rejected. Random rejected. Wrong signature rejected. Those rejections are the point.

---


## The 9 Layers, Explained

Rust-C has nine layers. Each layer verifies independently. No layer trusts the layer above it. If any layer fails, there is no commit.

### 1. Envelope — Who May Spend What, For Whom

Envelope is authorization. It answers: who is allowed to do this, for whom, under what constraints, with what kill-switch.

An Envelope is an Ed25519-signed object. It contains the issuer, the subject, the intent, the policy, the policy hash, the expiry, the nonce, and the signature. The signature is real Ed25519. Not a mock. Not a hash of a string. Real public-key cryptography.

Envelope checks: expiry, policy hash, revocation, kill-switch, replay nonce, delegation no-widening, signature, hash. If any check fails, verification fails. The error codes are explicit: `ENVELOPE_EXPIRED`, `POLICY_HASH_MISMATCH`, `REVOKED`, `KILL_SWITCH_ACTIVE`, `REPLAY_NONCE`, `BUDGET_WIDENED`, `BAD_ENVELOPE_SIGNATURE`, `ENVELOPE_HASH_MISMATCH`.

Delegation no-widening is critical. If Alice delegates to Bob, Bob cannot widen the scope. Bob cannot increase the budget. Bob cannot extend the expiry. Bob cannot add actions. Bob cannot add allowlisted counterparties. If Bob tries, verification fails with `*_WIDENED`. Delegation is for narrowing, not widening. That prevents privilege escalation through delegation chains.

Envelope is the authorization layer. Without a valid Envelope, there is no commit. Without a valid Envelope, there is no payment.

### 2. VIL — Intent Is Contract

VIL is covered above. VIL is the contract. VIL is compiled, hashed, and bound to the output. VIL is the only place where human intent lives in a verifiable form.

### 3. Rust-C — Deterministic Execution

Rust-C is covered above. Rust-C is the executor. Rust-C is deterministic. Rust-C is the only code that runs.

### 4. TEE — Code Y Ran In TEE Z

TEE is Trusted Execution Environment. Intel TDX, AMD SEV-SNP, or similar. TEE proves that code Y ran in TEE Z. The proof is a quote. The quote contains a measurement. The measurement must equal the code hash. The signature must verify with the TEE public key. The timestamp must not be in the future. The age must be within the max age.

In the hygienic repo, there is no mock TEE. There is no `mockTdxQuote()` that passes. If no TEE proof is supplied, there is no commit. The error is `TEE_PROOF_MISSING`. That is fail-closed. A system that passes with a mock TEE is not a system. It is a demo. Rust-C is not a demo.

In production, TEE verification uses Intel DCAP or AMD SEV-SNP. In this repo, TEE verification uses Ed25519 signature verification with a supplied TEE public key. The interface is the same. The mock is gone.

### 5. zkVM — Proof Linkage, Honestly Labelled

zkVM is zero-knowledge virtual machine. In production, zkVM is Risc Zero STARK or similar. A STARK proves that a computation was correct without revealing the computation.

In this repo, zkVM is linkage only. The code checks chain linkage: proof equals hash of step, input, output, program. It does not claim a STARK was verified. The field `starkVerified` is always `false` in this repo. That is honest labelling. A repo that claims a STARK was verified when it only checked a hash is not hygienic. This repo does not do that.

If you have a real STARK, verify it with a real verifier outside this repo, then pass `starkVerified: true` in your own wrapper. This repo will not claim it for you.

Linkage still matters. Linkage proves that the proof chain is continuous. No step was skipped. No step was reordered. No step was replaced. Linkage is necessary but not sufficient for a full STARK. It is honest to call it linkage.

### 6. SIP — State Is Merkle-Chained

SIP is State Integrity Protocol. SIP chains agent state as a Merkle chain. Each state update is hashed. Each new root is hash of previous root and state hash. The chain is verifiable. The history is portable. You can export state and import it elsewhere, and the importer can verify the chain before trusting it.

SIP answers: did the agent's state evolve correctly? Did it skip a version? Did it rewrite history? SIP verification checks version increments, prevRoot links, state hash recomputes, newRoot recomputes. If any check fails, SIP verification fails.

### 7. ATP — Attestation Transport Packet

ATP is Attestation Transport Packet. ATP is the packet that carries the output, the intent hash, and the attestation. ATP proves that code Y in TEE Z did only what intent W said, and that the output was transported with attestation.

ATP checks format and intent binding. It does not invent attestation. Attestation is opaque. It must be supplied by TEE and zkVM. ATP binds it to the intent.

### 8. x402 and x404 — Money Follows Proof

x402 is pay-per-call. x404 is escrowed, conditional payment. In Rust-C, both are used together.

Escrow is created before the work. Escrow is HELD during the work. Escrow is RELEASED only on on-chain proof. Escrow is REVERTED on proof failure.

In the hygienic repo, escrow stays HELD unless on-chain proof is supplied. There is no mock facilitator that claims payment happened. The facilitator proof formula is verified: `proofHash == sha256(txHash:amount:payee:idempotency)`. That formula is verified. But formula verification is not settlement. Settlement is on-chain. The field `onchainSettled` is `false` unless on-chain proof is supplied. That is honest.

Fee is 0.5% capped. The fee formula is `fee = min(amount / 200, cap)`. Cap is 100 cents if amount is less than 500 cents, 1000 cents otherwise. That is transparent. No hidden fee.

Merge-gated settlement means: no merge, no money. The escrow releases only if git verification passes and VIL verification passes. If either fails, the escrow reverts. The agent is not paid for unverified work.

### 9. git-core — History Is Hash-Chained

git-core is Git objects: blob, tree, commit. Blob hash is `sha256("blob <len>\0" + content)`. Tree hash is hash of sorted entries. Commit hash is hash of canonical commit payload. Commit payload includes tree, parent, message, taskId, inputHash, codeHash, outputHash, agentId, proof, and time.

The commit's proof field binds all eight layers above. Envelope id, policy hash, VIL intent hash, VIL commitment, Rust-C code hash, TEE measurement, zkVM chain hash, SIP root, ATP intent hash. All in one commit. All hash-bound.

Git chain verification checks parent links and hash recomputes. If any commit is tampered, verification fails. If any parent link is broken, verification fails. If genesis does not have null parent, verification fails.

That is the nine layers. Nine checks. One commit. No commit if any check fails.

---


## A Complete Example, End to End

Let us walk through a complete Rust-C commit, step by step, with real data.

Input: source bytes are sales records. Three records: revenue 100, 250, 50. Total should be 400.

VIL: `examples/defensive.vil` as above. Rust-C: `examples/main.rc.rs` as above.

Step 1: Envelope. Owner creates an Envelope for agent. Policy: per-task 2500 cents, daily 5000 cents, max spend 10000 cents, HITL threshold 5000 cents, allowed actions include `process-images`, allowlist includes `did:web:acme.com`. Envelope is signed with Ed25519. Envelope hash is computed. Envelope is verified: expiry, policy hash, signature, hash, replay, revocation, kill-switch. All pass.

Step 2: Policy. Policy engine evaluates: agent matches, policy hash matches, not expired, counterparty allowlisted, amount within per-task, within max spend, within daily, within velocity, below HITL threshold, action allowed. Result: `ALLOW`.

Step 3: VIL compile. VIL compiler parses intent, effects, trust, budget, body. Checks: no `settle()`, has `escrow()`, has `Committed<`, has `Validated<`, no `aggregate(committed)`, no `aggregate(raw)`, has effects, has trust. All pass. Summary is generated. Intent hash is computed. Summary hash is computed.

Step 4: Rust-C compile. Rust-C compiler checks: no floats, no time, no I/O, no random, correct signature. Parses `fn aggregate(state: u32, input: u32) -> u32`. Extracts let bindings and tail. Computes code hash. Computes WASM bytes. All pass.

Step 5: VIL execute. Fetch inside TEE: committed equals hash of source bytes plus nonce plus attestation. Validate: validated equals hash of committed plus validated. Aggregate: total equals sum of revenues, 400. Aggregated equals hash of validated plus total. Proof chain is fetch, validate, aggregate. Escrow is created, id is `esc_` plus hash, status HELD. Seal is commitment equals hash of total, proof chain, escrow id, intent hash.

Step 6: Rust-C step check. Rust-C step is run over source bytes: step(0,100)=100, step(100,250)=350, step(350,50)=400. Rust-C total equals VIL total, 400. If they differed, error `RUSTC_VIL_TOTAL_MISMATCH`, no commit.

Step 7: TEE. Caller supplies a real TEE proof. Measurement equals code hash. Code hash equals expected code hash. Signature verifies with TEE public key. Timestamp is not in the future. All pass. If no TEE proof, error `TEE_PROOF_MISSING`, no commit.

Step 8: zkVM linkage. Proof chain linkage is computed. Each proof equals hash of step, input, output, program. Chain links correctly. Linkage passes. STARK is not claimed. `starkVerified` is false.

Step 9: SIP. SIP store updates agent state: total 400, VIL commitment, zkVM chain hash, Rust-C code hash. New root is computed. SIP chain verifies: version, prevRoot, state hash, newRoot. All pass.

Step 10: ATP. ATP packet is created: intent hash, payload total, attestation with zkVM chain hash and VIL commitment, time, nonce. ATP verifies: version, intent hash, zkVM chain hash present. All pass.

Step 11: VIL verify. Sealed output is verified: intent hash matches, summary hash matches, commitment recomputes, escrow HELD. All pass. Verification uses hashes only, no re-execution.

Step 12: git-core commit. Git commit is created: files are intent, Rust-C, summary; message is `gitrust: <summary>`; taskId, inputHash, codeHash, outputHash, agentId, proof binding all layers, time. Commit hash is computed. Git chain verifies.

Step 13: x402 escrow. Escrow is created in x402 manager: id, amount, payee, idempotency key, time, status HELD. Facilitator proof formula is checked if supplied. On-chain proof is not supplied in the demo, so escrow stays HELD. Settlement is `HELD unless on-chain proof supplied`. `onchainSettled` is false. That is honest.

Receipt is created: version, gitCommit, envelopeId, policy, vil, rustc, tee, zk, sip, atp, x402, total 400, verifiedHashChecks true, receiptHash. Receipt hash is hash of receipt without receiptHash field.

That is a complete Rust-C commit. Thirteen steps. Nine layers. One receipt. One commit.

You can run this exact example with `cargo run -- commit examples/defensive.vil examples/main.rc.rs`. The output is the receipt. The receipt is the proof.

---


## How Verification Works Without Re-Execution

Verification is cheap. Execution is expensive. That asymmetry is the point.

A verifier does not need to re-run the aggregation. A verifier does not need to re-fetch the source. A verifier does not need to re-run Rust-C. A verifier checks hashes and proof chains.

To verify a Rust-C commit, a verifier:

- Checks Envelope signature and hash and expiry and policy hash and replay and revocation.
- Checks VIL intent hash and summary hash.
- Checks VIL seal commitment recomputes from total, proof chain, escrow id, intent hash.
- Checks Rust-C code hash matches TEE measurement.
- Checks TEE signature with TEE public key.
- Checks zkVM linkage: each proof recomputes, each link connects.
- Checks SIP chain: version, prevRoot, state hash, newRoot.
- Checks ATP intent binding.
- Checks git chain: parent links and hash recomputes.
- Checks x402 facilitator formula if supplied, and on-chain proof if settlement is claimed.

All of those are hash checks and signature checks. None require re-execution. None require trusting the agent. None require trusting the facilitator. None require re-fetching the source.

That is why Rust-C scales. Execution happens once, in TEE, with Rust-C. Verification happens many times, by anyone, cheaply, with hashes.

This is the same principle as blockchains, but without consensus. Blockchains verify by re-execution and consensus. Rust-C verifies by hash and proof, without consensus, for a single agent's commit. Consensus is for shared state. Rust-C is for single-agent verifiability. That is a different problem, and a simpler problem, and a problem that does not need a blockchain.

---


## Hygiene: What We Keep and What We Leave Out

Hygiene means: keep only what is deterministic, hash-verifiable, and does not claim proof it does not have. Everything else is left out, even if it was in an earlier version of Rust-C.

We score every component 0 to 100. Keep if score is 70 or higher. Leave out if score is below 70. No mock that passes counts as proof.

### Keep — Score 70 to 100

Canonical JSON plus SHA256: 100. Pure, deterministic, no trust.

Git blob, tree, commit hash plus chain verify: 98. Pure hash, parent link, tamper-evident.

Envelope Ed25519 signature verify: 97. Real crypto, no mock.

Envelope policyHash: 96. Hash only.

VIL proof-carrying types: 95. Compile-time, rejects RawBytes to aggregate.

Policy evaluation, pure: 94. Pure function, explicit time.

Rust-C lexer, parser, typecheck: 95. Pure, compile-time.

Rust-C step plus WASM bytes: 92. Deterministic.

VIL seal commitment recompute: 90. Hash only.

SIP root recompute: 90. Hash only.

VIL summary hash: 88. Summary divergence rejected.

VIL effects as types: 88. Compile-time.

VIL trust declaration: 85. Declaration, not proof — kept as declaration.

Escrow state machine: 85. Pure, release only on on-chain proof.

x402 facilitator formula: 80. Formula only — kept as formula, not as settlement.

ATP format plus intent binding: 78. Format plus binding only.

zkVM chain linkage: 70. Linkage only — `starkVerified` is always false.

### Leave Out — Score Below 70

Mock TDX quote: 5. Fake TEE. Claims hardware proof with a hash of a string. Removed. No mock quote passes. `TEE_PROOF_MISSING` equals no commit.

Mock zkVM STARK, hash equals STARK: 5. Hash is not a STARK. Removed.

Mock facilitator settlement: 5. Fake payment. No on-chain transaction. Removed. Escrow stays HELD.

Fake TEE signature: 0. Not a signature. Removed.

Marketplace, governance, swarm: 10 to 15. Coordination, not proof. Left out.

NSP: 15. Coordination, not proof. Left out.

MCP, search, cache: 20 to 30. Convenience, not proof. Cache is not a proof boundary. Left out.

Diff as security boundary: 65. Deterministic but not a security boundary. Left out as proof.

Date now, random UUID in execution: 0. Nondeterministic. Banned. Time, nonce, id are explicit inputs.

Math random in execution: 0. Nondeterministic. Banned.

Duplicate files: 0. One canonical file per module.

What changed versus earlier GitRust C? Earlier, escrow RELEASED on mock facilitator. Hygienic: escrow HELD unless on-chain proof supplied. Earlier, mock TEE quote passed. Hygienic: no TEE proof equals no commit. Earlier, zkVM proof claimed. Hygienic: linkage only, `starkVerified` false always. Earlier, Date now in receipts. Hygienic: now, nonce, id are explicit inputs, same inputs equals same receipt hash, tested.

Hygiene is not a style. Hygiene is a security property. A repo with mocks that pass is not a proof repo. It is a demo repo. This repo is a proof repo.

---


## Architecture

```
Rust-C Repo (100% Rust)
├── Cargo.toml
├── src/
│   ├── canonical.rs       # Canonical JSON + SHA256
│   ├── git_core.rs        # Git blob / tree / commit + chain verify
│   ├── rustc_compiler.rs  # Rust-C lexer / parser / typecheck / step
│   ├── vil_compiler.rs    # VIL parse + compile + summary + intentHash
│   ├── vil_executor.rs    # VIL execute — fetch / validate / aggregate / escrow / seal
│   ├── vil_verifier.rs    # VIL verify — hashes only
│   ├── escrow.rs          # Escrow state machine
│   ├── envelope.rs        # Ed25519 Envelope + PolicyEngine
│   ├── sip.rs             # SIP Merkle chain
│   ├── tee.rs             # TEE proof verify — no mock
│   ├── zkvm.rs            # zkVM linkage only, starkVerified false
│   ├── x402.rs            # x402 formula + escrow
│   ├── atp.rs             # ATP packet + intent binding
│   ├── gitrust.rs         # Hygienic commit pipeline — all layers in one commit
│   ├── lib.rs
│   └── main.rs            # CLI: vil / build / commit
├── examples/
│   ├── defensive.vil
│   └── main.rc.rs
├── tests/
│   └── hygiene.rs
├── docs/
│   ├── VIL.md
│   ├── HYGIENE.md
│   └── ARCHITECTURE.md
└── README.md — this file
```

VIL standalone repo has the same VIL files, but no Rust-C, no Envelope, no git-core. Rust-C repo has VIL plus everything else. Both are 100% Rust. Both are hygienic. VIL is the contract. Rust-C is the full system.

The Rust code mirrors the earlier Node reference, but Rust is normative now. Node is gone. If you see Node in an old zip, ignore it. This repo is Rust.

---


## Security Model and Threat Model

Rust-C assumes an adversarial agent. The agent may be malicious, misaligned, prompt-injected, or simply buggy. The agent may try to claim work it did not do. The agent may try to widen its scope. The agent may try to exfiltrate data. The agent may try to get paid for unverified work. The agent may try to rewrite history.

Rust-C defends against each of those.

**Malicious agent claims work:** VIL seal plus Rust-C code hash plus TEE measurement plus zkVM linkage plus git chain. To fake work, the agent would need to forge Ed25519 signatures, forge TEE signatures, forge hashes, and break SHA256. That is not assumed to be possible.

**Agent widens scope:** Envelope delegation no-widening. Policy checks. Effects as types. If the agent tries to do an effect not in VIL, or spend more than policy, or delegate wider, verification fails.

**Agent exfiltrates data:** This is the hardest threat. Rust-C does not claim to prove non-exfiltration. TEE plus VIL plus Rust-C prove the agent did the declared work in TEE. They do not prove the agent did not also exfiltrate inside TEE. Proving non-exfiltration inside TEE is still open research. Confidential compute plus proof of non-exfiltration is the battleground. This README does not claim Rust-C solves it. It claims Rust-C makes exfiltration harder and auditable, not impossible. Honest limits are part of hygiene.

**Agent gets paid for unverified work:** Escrow plus merge-gated settlement. No verification, no money. No merge, no money. Escrow stays HELD unless on-chain proof.

**Agent rewrites history:** Git chain plus SIP chain. History is hash-chained. Rewriting history changes hashes. Verification fails.

**Facilitator lies about payment:** Facilitator proof formula is checked. On-chain settlement is required for RELEASED. Formula alone is not settlement. A facilitator that claims payment without on-chain proof is rejected.

**Verifier is lazy or malicious:** Verification is cheap and public. Anyone can verify. Verification does not require trust in the agent or the facilitator. Verification does not require re-execution. A lazy verifier can still verify cheaply. A malicious verifier cannot forge a valid receipt without breaking hashes.

**What Rust-C does not defend against:** If the TEE hardware is compromised, TEE proof is compromised. If the zkVM prover is compromised, STARK is compromised. If Ed25519 is broken, signatures are broken. If SHA256 is broken, hashes are broken. Those are assumptions of the whole system, not just Rust-C. Rust-C does not claim to be secure if cryptography is broken.

**What Rust-C does not claim:** It does not claim TEE proves non-exfiltration. It does not claim zkVM linkage is a STARK. It does not claim formula verification is settlement. It does not claim a mock is a proof. Those honest labels are the hygiene.

---


## Determinism: Same Input, Same Output, Always

Determinism is tested. Not claimed. Tested.

Run the same VIL with the same source bytes, same attestation, same nonce, same time, and you get the same commitment. Run it twice, three times, a hundred times, same commitment. The test `execute_deterministic_total_400` checks this.

Run the same Rust-C step with the same state and input, and you get the same output. Always. The test checks `step(0,100)==100`, `step(100,250)==350`, and `step(0,100)==step(0,100)`.

Run the same hygienic commit with the same explicit inputs, and you get the same VIL commitment and same receipt hash. The test checks this.

Why does determinism matter for trending? Because determinism is what makes a repo trustworthy. A repo that is nondeterministic is not reproducible. A repo that is not reproducible is not auditable. A repo that is not auditable will not trend for long. Trending repos that last are reproducible repos.

Determinism also matters for SEO. Search engines and GitHub search reward repos that are easy to run. If `cargo test` passes every time, with the same output, contributors trust the repo. If `cargo test` is flaky, contributors leave. Determinism is a growth strategy, not just a security property.

---


## Comparison: Git vs GitHub vs Rust-C vs Blockchains vs Agent Frameworks

| System | Tracks Code | Tracks Intent | Tracks Proof | Tracks Payment | Verifiable Without Trust | Deterministic Execution |
|---|---|---|---|---|---|---|
| Git | Yes | No | No | No | No — trust committer | No |
| GitHub | Yes | No — PR description | No | No | No — trust reviewer | No |
| Blockchains | No — tracks transactions | No | Yes — consensus | Yes — on-chain | Yes — but expensive, needs consensus | Depends |
| Agent Frameworks (LangChain, AutoGen, etc.) | No | Prompt — not verifiable | No | No | No — trust agent | No |
| **Rust-C** | **Yes** | **Yes — VIL** | **Yes — TEE + zkVM linkage + hashes** | **Yes — x402 escrow, merge-gated** | **Yes — hashes only** | **Yes — Rust-C** |

Git is for humans. GitHub is for collaboration. Blockchains are for shared state with consensus. Agent frameworks are for building agents. Rust-C is for verifiable agent work. It does not replace any of those. It complements them. You can use Rust-C with GitHub. You can use Rust-C with a blockchain for settlement. You can use Rust-C with any agent framework. Rust-C is the verifiability layer that those systems lack.

A blockchain can prove a transaction happened. It cannot prove an agent did the work correctly in TEE. A blockchain can pay an agent. It cannot prove the agent's intent matched the human's intent. Rust-C can. That is the complementarity.

An agent framework can run an agent. It cannot prove the agent's output is correct without re-running it. Rust-C can prove it with hashes, without re-running.

GitHub can show a PR. It cannot prove the PR was generated by verified deterministic execution. Rust-C can.

That is why Rust-C is not a GitHub replacement. It is a commit format plus verifier that GitHub, GitLab, or any Git host can adopt. That is why Rust-C is not a blockchain replacement. It is Git plus proofs, no consensus needed for a single agent's commit. That is why Rust-C is not a platform that owns agents. It is the neutral layer that verifies them.

---


## Use Cases

**Agent-written code review:** An agent writes a PR. The PR is a Rust-C commit. The reviewer does not need to trust the agent. The reviewer verifies the commit: VIL intent, Rust-C execution, TEE, zkVM linkage, SIP, ATP, escrow. If verification passes, the reviewer reviews the intent, not the code. If verification fails, the PR is rejected automatically.

**Data processing with privacy:** An agent processes sales data in TEE. The data never leaves TEE in raw form. The output is a sealed total. The verifier checks the seal without seeing the raw data. That is privacy plus verifiability.

**Agent-to-agent payments:** Agent A pays Agent B for a task. Payment is escrowed. Agent B does the task as a Rust-C commit. Agent A verifies the commit. If verified, escrow releases. If not, escrow reverts. No trust, no dispute, no intermediary.

**Auditing:** An auditor wants to know: did this agent do what it was authorized to do, with what budget, with what effects, in what TEE, with what proof? The auditor reads the Rust-C commit. The commit contains the proof. The auditor verifies hashes. The auditor does not need to trust the agent, the platform, or the facilitator.

**Compliance:** A regulator wants to know: was this AI system used within its authorized scope? The Rust-C commit is the audit trail. Envelope plus VIL plus policy plus proof is the compliance record.

**Open-source agent contributions:** An open-source project accepts agent contributions. The project does not want to trust every agent. The project requires Rust-C commits. Agent contributions that are not Rust-C commits are rejected. Agent contributions that are Rust-C commits are verified automatically.

**Enterprise agent deployment:** An enterprise deploys agents in production. The enterprise needs to prove to customers, auditors, and regulators that agents acted within scope. Rust-C commits are the proof.

**Research:** A researcher wants to reproduce an agent's result. The Rust-C commit contains the code hash, the input hash, the output hash, the nonce, the seed. The researcher can reproduce the result deterministically. No more "it worked on my machine."

These use cases are not hypothetical. They are the reason Rust-C exists. The agent economy needs verifiability. Verifiability needs Rust-C.

---


## For AI Agents, For Humans, For Auditors

**For AI agents:** If you are an agent reading this README, you can use Rust-C. Write VIL for your intent. Write Rust-C for your execution. Supply a real TEE proof. Do not mock. Do not fake. Do not claim a STARK if you only have linkage. Do not claim settlement if you only have a formula. Be hygienic. The tests will check you. The verifier will check you. The community will check you. Hygienic agents trend. Mock agents do not.

**For humans:** If you are a human reading this README, you do not need to understand every layer to use Rust-C. Start with Quickstart. Run the examples. Read VIL. Write a VIL intent for a task you care about. Write a Rust-C function for it. Run `cargo test`. You have now made a verifiable commit. That is the whole system.

**For auditors:** If you are an auditor, start with Security Model and Threat Model, then Hygiene, then the tests. The tests are the spec. The code is the spec. The README is the explanation. If the README claims something the code does not do, the code wins. If the tests fail, the README is wrong. Auditors should run `cargo test` and read `src/gitrust.rs`. That file is the pipeline. That file is the truth.

---


## Benchmarks and Honest Limits

Rust-C is fast because verification is cheap. Execution is once. Verification is hashes. Hashes are fast. SHA256 is fast. Ed25519 verification is fast. Canonical JSON is fast. The whole hygienic commit for the example takes milliseconds in Rust. That is not a benchmark claim. That is a consequence of the design: verification is hashes, hashes are fast.

Honest limits, stated clearly, as hygiene requires:

- zkVM linkage is not a STARK. Production needs Risc Zero STARK, same interface, different verifier. This repo does not claim STARK.
- TEE mock is gone. Production needs Intel DCAP or AMD SEV-SNP. This repo requires a real TEE proof, supplied by the caller.
- Proving an agent did not exfiltrate inside TEE is still open. Rust-C plus TEE plus VIL make exfiltration harder and auditable, not impossible.
- x402 formula verification is not settlement. Settlement is on-chain. This repo does not claim settlement without on-chain proof.
- Rust-C is a subset. It cannot do everything Rust can do. That is intentional. If you need floats, time, I/O, random, Rust-C is not for that task. Use normal Rust for that task, and Rust-C for the verifiable part.
- VIL is small. It cannot express every intent. That is intentional. If your intent is too complex for VIL, break it into smaller VIL intents. Small intents are easier to verify.

A repo that hides its limits will trend for a day and die. A repo that states its limits will trend and last. This repo states its limits.

---


## FAQ — 40 Questions Answered

**1. What is Rust-C?** Rust-C is a deterministic Rust subset plus VIL plus Envelope plus git-core plus TEE plus zkVM linkage plus SIP plus ATP plus x402, all hashed into one commit. It is the first version control where a commit is proof of execution.

**2. What is VIL?** Verifiable Intent Language. Intent equals contract equals code. VIL is human-readable, compilable, hashable, verifiable.

**3. Is Rust-C 100% Rust?** Yes. This repo is 100% Rust. No JavaScript. No Node. No Python. `src/*.rs` only.

**4. Is VIL a separate repo?** Yes. VIL is standalone at `CodesbyFebin/VIL`. It is also included in Rust-C. Use VIL alone if you only care about intent. Use Rust-C if you care about execution, payment, and history.

**5. How do I run Rust-C?** `cargo test`, `cargo run -- vil examples/defensive.vil`, `cargo run -- commit examples/defensive.vil examples/main.rc.rs`.

**6. What does a Rust-C commit prove?** It proves authorization, intent, deterministic execution, TEE, proof linkage, state, transport, settlement, and history, all hash-bound in one commit.

**7. What if TEE proof is missing?** No commit. Error `TEE_PROOF_MISSING`. Fail-closed.

**8. What if policy is over budget?** No commit. Error `PER_TASK_EXCEEDED` or similar. No money.

**9. What if VIL seal is tampered?** Verification fails. Error `SEAL_COMMITMENT_MISMATCH`. No commit. No money.

**10. What if git chain is tampered?** Verification fails. Error `COMMIT_HASH_MISMATCH`. No settlement.

**11. What is the difference between linkage and STARK?** Linkage checks that the proof chain is continuous and each proof recomputes as a hash. STARK proves the computation was correct with zero-knowledge. Linkage is necessary but not sufficient for STARK. This repo does linkage only, `starkVerified` false always.

**12. What is the difference between formula and settlement?** Formula checks that `proofHash == sha256(tx:amount:payee:idem)`. Settlement checks that an on-chain transaction actually happened. Formula is not settlement. This repo does formula, not settlement, unless on-chain proof is supplied.

**13. Why is escrow HELD in the demo?** Because no on-chain proof is supplied in the demo. Escrow stays HELD unless on-chain proof is supplied. That is honest. A demo that claims RELEASED without on-chain proof is a mock.

**14. Can I use Rust-C with GitHub?** Yes. Rust-C is a commit format plus verifier. GitHub, GitLab, or any Git host can adopt it. Rust-C does not replace GitHub.

**15. Can I use Rust-C with a blockchain?** Yes. Use a blockchain for settlement, Rust-C for verifiability. They complement each other.

**16. Can I use Rust-C with LangChain or AutoGen?** Yes. Use the agent framework to run the agent, Rust-C to verify the agent's work.

**17. What is deterministic execution?** Same source, same input, same output, on any machine, at any time. No floats, no time, no random, no I/O.

**18. Why ban floats?** Floats are not deterministic across platforms and compilers. Rust-C bans floats to guarantee determinism.

**19. Why ban time?** Time is nondeterministic. Time is an explicit input in Rust-C, not a hidden input.

**20. Why ban random?** Random is nondeterministic. Randomness, if needed, is an explicit seed input.

**21. Why ban I/O?** I/O is nondeterministic and can exfiltrate. Rust-C functions are pure.

**22. What is wrapping arithmetic?** Wrapping arithmetic does not panic on overflow. It wraps. It is deterministic. Panic depends on debug vs release and platform. Wrapping is always the same.

**23. What is canonical JSON?** Canonical JSON is JSON with sorted keys. Same object, same string, same hash. `serde_json::Value::Object` is a BTreeMap, sorted, so `to_string()` is canonical.

**24. What is an Envelope?** An Ed25519-signed authorization object. It contains issuer, subject, intent, policy, policy hash, expiry, nonce, signature, hash.

**25. What is delegation no-widening?** A delegate cannot widen scope. Cannot increase budget, extend expiry, add actions, or add allowlisted counterparties. Widening is rejected with `*_WIDENED`.

**26. What is SIP?** State Integrity Protocol. Merkle-chained agent state. Version, prevRoot, state hash, newRoot. Portable and verifiable.

**27. What is ATP?** Attestation Transport Packet. Packet that carries output, intent hash, and attestation. Binds attestation to intent.

**28. What is x402?** Pay-per-call. What is x404? Escrowed, conditional payment. Rust-C uses both. Escrow held, released only on verification.

**29. What is merge-gated settlement?** No merge, no money. Escrow releases only if git verification and VIL verification pass.

**30. What is the fee?** 0.5% capped. Fee equals `min(amount / 200, cap)`. Cap is 100 cents if amount is less than 500 cents, 1000 cents otherwise.

**31. How do I contribute?** Read Contributing. Start with good first issues. Write tests. Write docs. Write examples. No mocks. No fake proofs. Whitehat only.

**32. How do I get this repo to trend?** Read How This Repo Trends. Whitehat only. Share, teach, write tutorials, make videos, write blog posts, help new contributors. No bots. No bought stars. No fake accounts.

**33. Is this repo production-ready?** It is hygienic v1.0. It is ready for pilots, audits, and research. For production TEE, use Intel DCAP or AMD SEV-SNP. For production zkVM, use Risc Zero STARK. For production settlement, use on-chain verification. The interfaces are the same. The mocks are gone.

**34. What are the honest limits?** See Benchmarks and Honest Limits. zkVM linkage is not STARK. TEE does not prove non-exfiltration. Formula is not settlement. Rust-C is a subset. VIL is small. Those limits are stated, not hidden.

**35. How do I verify a Rust-C commit without trusting the agent?** Run the verifier. Check hashes and signatures. Do not re-execute. Do not trust the agent. Do not trust the facilitator. Trust hashes.

**36. How do I write a VIL intent?** Start with `examples/defensive.vil`. Copy it. Change intent, effects, trust, body. Run `cargo run -- check`. If it compiles, it is valid VIL. If it fails, the error tells you why.

**37. How do I write Rust-C?** Start with `examples/main.rc.rs`. Write `fn aggregate(state: u32, input: u32) -> u32` with only u32 arithmetic, no floats, no time, no I/O, no random. Run `cargo test`.

**38. How do I report a security issue?** Open a GitHub issue with `security` label, or email the maintainer. Do not post exploit code publicly before a fix. Whitehat disclosure only.

**39. What license is Rust-C?** MIT. Use it. Fork it. Build on it. Credit it.

**40. Where do I ask questions?** GitHub Issues, GitHub Discussions, or the VIL repo. Be specific. Include your VIL, your Rust-C, your error, your `cargo test` output. Good questions get good answers.

---


## Roadmap

**v0.2 — VIL standalone:** Done. VIL as its own repo, 100% Rust, 6 tests, examples, spec. This is the foundation.

**v1.0 — Rust-C hygienic:** Done. This repo. VIL plus Rust-C plus Envelope plus git-core plus SIP plus ATP plus TEE plus zkVM linkage plus x402, all in 100% Rust, fail-closed, no mocks.

**v1.1 — Real TEE:** Intel DCAP and AMD SEV-SNP verifiers, same interface as current TEE module, no mock. TEE proof verification with real hardware keys.

**v1.2 — Real zkVM:** Risc Zero STARK verifier, same interface as current linkage module, `starkVerified` true only when STARK verifies. Linkage remains as a fast check.

**v1.3 — Real settlement:** On-chain settlement verification for x402 and x404, same interface as current formula module, `onchainSettled` true only when on-chain transaction verifies.

**v1.4 — GitHub integration:** GitHub Action that verifies Rust-C commits on PRs. PRs with invalid Rust-C commits are rejected automatically. PRs with valid Rust-C commits show a verified badge.

**v1.5 — Registry:** A registry of verified Rust-C intents, like crates.io but for VIL. Searchable, ranked, auditable. Intents with high verification rates rank higher.

**v2.0 — Non-exfiltration:** Research on proof of non-exfiltration inside TEE. Confidential compute plus proof of non-exfiltration. This is the hardest roadmap item. It is research, not a promise. It is listed to be honest about what is next, not to claim it is solved.

Roadmap is not a promise. Roadmap is a direction. Each version will be hygienic, tested, and honest about its limits, like v1.0.

---


## Contributing — Whitehat Only

Contributing to Rust-C is contributing to verifiable agents. We welcome contributors. We require whitehat.

**How to contribute:**

- **Read the code.** Start with `src/vil_compiler.rs`, then `src/rustc_compiler.rs`, then `src/gitrust.rs`. The pipeline is in `gitrust.rs`.
- **Run the tests.** `cargo test` must pass before you open a PR. No exceptions.
- **Write tests first.** A PR without tests is a PR without proof. Write a test that fails, then fix it, then show the test passes.
- **Write docs.** A PR that changes behavior must change docs. A PR that changes VIL must change `docs/VIL.md` and `examples/`.
- **Write examples.** A new feature needs an example in `examples/`. An example that is not tested is not an example. It is a wish.
- **No mocks.** Do not add a mock TEE, mock zkVM, mock facilitator, or mock settlement. If you need a mock for a test, label it `mock` and make it fail verification. Mocks that pass are banned.
- **No blackhat.** No bots, no fake stars, no fake accounts, no vote manipulation, no spam, no plagiarism, no hidden tracking. Whitehat only.
- **Good first issues:** Look for issues labelled `good first issue`. They are small, tested, and mentored. Start there. Ask questions in the issue. Be specific. Show your `cargo test` output.
- **Code review:** Review PRs. Check hashes. Check signatures. Check determinism. Check hygiene. A good review is a contribution.
- **Security:** If you find a security issue, report it privately first. Do not post exploit code publicly before a fix. Whitehat disclosure only.

**Code of Conduct:** Be kind. Be specific. Be honest about limits. No harassment. No spam. No self-promotion without contribution. The community is for verifiable agents, not for growth hacking.

---


## How This Repo Trends — Whitehat Playbook

GitHub trending is not magic. It is not luck. It is not bought. It is the result of whitehat work, repeated daily.

GitHub trending rewards: stars from real accounts, forks from real accounts, clones, views, new contributors, new issues, new PRs, new releases, documentation quality, example quality, test quality, and sustained activity over days, not minutes.

Here is the whitehat playbook for this repo. All of it is whitehat. None of it is manipulation.

**1. Make the repo easy to star.** A repo that is easy to understand in 60 seconds gets starred. That is why Quickstart is first. That is why the differentiator is one sentence. That is why the example is the same everywhere. If a visitor cannot understand the repo in 60 seconds, they will not star it. Make it easy.

**2. Make the repo easy to run.** `cargo test` must pass. `cargo run -- vil` must work. `cargo run -- commit` must work. If a visitor clones and `cargo test` fails, they will not star, they will not fork, they will not contribute. They will leave. Make it run.

**3. Make the repo easy to share.** A README that is 5,000+ words, structured, with a table of contents, with examples, with FAQ, with comparison tables, is easy to share. A reader can link to a section. A reader can quote a section. A reader can tweet a section. A reader can write a blog post about a section. Long, structured READMEs are shareable READMEs. Short, vague READMEs are not.

**4. Publish on a schedule.** Trending is about sustained activity, not one spike. Publish a release, then a blog post, then a tutorial, then a video, then a good first issue, then a PR, then a release. Repeat. GitHub sees sustained activity. Humans see sustained activity. Both reward it.

**5. Teach, do not sell.** Write a blog post: "How I verified an agent's work with Rust-C." Write a tutorial: "Write your first VIL intent in 10 minutes." Make a video: "Rust-C in 60 seconds." Teach. Teaching is whitehat. Selling is not. People star repos that teach them something. People ignore repos that sell them something.

**6. Help new contributors.** Answer issues quickly. Label good first issues. Mentor PRs. Thank contributors. A repo with active, kind maintainers trends. A repo with silent maintainers dies. Be active. Be kind. Be specific.

**7. Be honest about limits.** A repo that claims it solves everything will be distrusted. A repo that states its limits will be trusted. Trust trends. Distrust does not. State limits. State what is not proven. State what is next. Honesty is whitehat. Honesty is trending.

**8. Cross-post whitehat.** Post on X, LinkedIn, Dev.to, Hacker News, Reddit, Discord, but only where it is relevant, only once per platform, only with real value, never spam. Share the example, share the tutorial, share the proof. Do not spam. Do not buy upvotes. Do not use bots. Do not use fake accounts. One real share is worth a hundred fake shares. GitHub and the platforms detect fake shares. Real shares compound. Fake shares get banned.

**9. Make releases.** GitHub trending rewards releases. Tag a release. Write release notes. Explain what changed, what was fixed, what is next. Releases are signals. Signals trend.

**10. Keep the README fresh.** Update the README when the code changes. Update the FAQ when new questions are asked. Update the roadmap when a version ships. A stale README signals a dead repo. A fresh README signals a living repo. Living repos trend.

**What not to do — blackhat, banned, will get you banned:**

- Do not buy stars, forks, followers, or views.
- Do not use bots, fake accounts, or automation to star, fork, or upvote.
- Do not spam issues, PRs, or comments on other repos to promote this repo.
- Do not plagiarize other repos' READMEs or code.
- Do not keyword-stuff with hidden text, invisible text, or misleading keywords.
- Do not create fake issues or fake PRs to inflate activity.
- Do not ask friends to star without reading the repo. Real stars from real readers are whitehat. Fake stars from people who never read the repo are not.

Whitehat trending is slower than blackhat trending, but it lasts. Blackhat trending gets you banned. Whitehat trending gets you contributors, users, and a repo that lasts.

---


## SEO and Discoverability — How This README Is Built to Rank

This README is built to rank on GitHub search and on Google, whitehat.

**Keywords, naturally:** Rust, Rust-C, VIL, Verifiable Intent Language, deterministic, verifiable, TEE, zkVM, SIP, ATP, x402, x404, Envelope, git-core, agent, AI agent, agent economy, verifiable agents, proof of execution, proof of commit, escrow, merge-gated settlement, deterministic Rust, trusted execution, zero-knowledge, Merkle chain, Ed25519, canonical JSON, SHA256, WASM, Risc Zero, Intel TDX, AMD SEV-SNP, GitHub trending, open source, 100% Rust.

Those keywords appear naturally, in headings, in tables, in FAQ, in examples. They are not stuffed. They are not hidden. They are not repeated artificially. They appear because they are the topic. That is whitehat SEO.

**Structure for ranking:** Headings are hierarchical. Table of contents links to headings. Code blocks are labelled. Tables are used for comparison. FAQ is used for long-tail search. A user searching "what is VIL" will find the VIL section. A user searching "Rust-C deterministic" will find the Rust-C section. A user searching "merge-gated settlement" will find the x402 section. A user searching "how to trend on GitHub whitehat" will find the playbook section. Structure is SEO.

**Length for ranking:** This README is 5,000+ words. Long READMEs rank higher because they cover more queries, answer more questions, and keep readers longer. Length is not filler. Length is coverage. Every section answers a real question. Every FAQ answers a real search query.

**Examples for ranking:** Code examples rank. A user searching for a VIL example will find this README. A user searching for a Rust-C example will find this README. A user searching for a hygienic commit example will find this README. Examples are SEO.

**Freshness for ranking:** This README will be updated with every release. Fresh READMEs rank higher than stale READMEs. Freshness is whitehat.

**Links for ranking:** This README links to VIL repo, to Rust-C repo, to docs, to examples. Internal links help ranking. External links to real docs help ranking. No link farms. No bought links. Whitehat only.

If you are reading this README because you found it on trending or on search, the SEO worked. Whitehat SEO works. It works because it helps readers, not because it tricks algorithms.

---


## Changelog

**v1.0.0 — Hygienic Rust-C, 100% Rust:** VIL plus Rust-C plus Envelope plus git-core plus SIP plus ATP plus TEE plus zkVM linkage plus x402, all in 100% Rust, fail-closed, no mocks. Tests pass. README 5,000+ words. VIL standalone repo released.

**v0.2.0 — VIL standalone:** VIL as its own repo, 100% Rust, 6 tests, examples, spec. VIL is the contract.

**v0.1.0 — GitRust C, Node reference:** Earlier Node reference, with mocks. Deprecated. Replaced by hygienic Rust. Node is gone. Mocks are gone.

Future changelog will be in `CHANGELOG.md` with every release. Changelog is part of trending. Changelog is part of SEO. Changelog is part of trust.

---


## License

MIT. Use it. Fork it. Build on it. Credit it.

```
MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files, to deal in the Software
without restriction, including without limitation the rights to use, copy,
modify, merge, publish, distribute, sublicense, and/or sell copies of the
Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions: The above copyright notice and this
permission notice shall be included in all copies or substantial portions
of the Software.
```

---

**If you read this far, star the repo, fork it, run `cargo test`, and write your first VIL intent. That is how a repo trends — whitehat, one real reader at a time.**

**Next:** Read `docs/VIL.md`, run `cargo run -- run examples/defensive.vil`, then write your own VIL for a task you care about. Share what you built. Teach someone else. That is the playbook.

