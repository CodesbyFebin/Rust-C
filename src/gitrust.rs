use crate::canonical::{canonical_hash, sha256_hex};
use crate::compiler::{compile_vil, Compiled};
use crate::envelope::{create_envelope, verify_envelope, Agent, PolicyEngine};
use crate::executor::{execute_ir, Escrow as VilEscrow};
use crate::git_core::GitCore;
use crate::rustc_compiler::{compile_rustc, CompiledRustC};
use crate::sip::SipStore;
use crate::tee::{verify_tee_proof, TeeProof};
use crate::verifier::verify_sealed;
use crate::x402::{fee_cents, X402Escrow, Escrow as XEscrow};
use crate::atp::{make_atp_packet, verify_atp_packet};
use crate::zkvm::{prove_chain_linkage, verify_chain_linkage};
use serde_json::{json, Value};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Receipt { pub value: Value }
pub struct GitrustOutput { pub ok: bool, pub reason: String, pub stage: String, pub receipt: Option<Value> }

#[allow(clippy::too_many_arguments)]
pub fn gitrust_commit_hygienic(vil_source: &str, rustc_source: &str, source_bytes: &[Value], owner: &Agent, agent: &Agent, policy: &Value, payee: &str, price_cents: u64, now: &str, nonce: &str, envelope_id: &str, tee_proof: Option<TeeProof>) -> GitrustOutput {
    if now.is_empty() || nonce.is_empty() || envelope_id.is_empty() {
        return GitrustOutput { ok: false, reason: "EXPLICIT_INPUTS_REQUIRED".to_string(), stage: "INPUTS".to_string(), receipt: None };
    }
    // 1 Envelope
    let envelope = create_envelope(owner, Some(agent.did.clone()), json!({"goal":"process_sales_data","action":"process-images","resource":"images","budgetCents":price_cents,"counterpartyDid":payee}), policy.clone(), "2027-01-01T00:00:00.000Z", now, nonce, envelope_id);
    let mut seen = HashSet::new();
    if let Err(e) = verify_envelope(&envelope, owner, &mut seen, now) {
        return GitrustOutput { ok: false, reason: e, stage: "ENVELOPE".to_string(), receipt: None };
    }
    // 2 Policy
    let engine = PolicyEngine::new();
    if let Err(e) = engine.evaluate(policy, &envelope, &agent.did, price_cents, payee, now) {
        return GitrustOutput { ok: false, reason: e, stage: "POLICY".to_string(), receipt: None };
    }
    // 3 VIL compile
    let vil_compiled: Compiled = match compile_vil(vil_source) { Ok(c)=>c, Err(e)=> return GitrustOutput { ok:false, reason: e.code, stage:"VIL_COMPILE".to_string(), receipt:None } };
    // 4 Rust-C compile
    let rustc: CompiledRustC = match compile_rustc(rustc_source) { Ok(c)=>c, Err(e)=> return GitrustOutput { ok:false, reason:e, stage:"RUSTC".to_string(), receipt:None } };
    // 5 VIL execute
    let vil_exec = match execute_ir(&vil_compiled, source_bytes, "attested-url-tls:example.com/sales", nonce, now) { Ok(e)=>e, Err(e)=> return GitrustOutput { ok:false, reason:e, stage:"VIL_EXEC".to_string(), receipt:None } };
    let mut rustc_total: u32 = 0;
    for v in source_bytes { if let Some(r)=v.get("revenue").and_then(|x| x.as_u64()) { rustc_total = rustc.step(rustc_total, r as u32); } }
    if rustc_total as u64 != vil_exec.total { return GitrustOutput { ok:false, reason:"RUSTC_VIL_TOTAL_MISMATCH".to_string(), stage:"RUSTC_VIL".to_string(), receipt:None } };
    // 6 TEE fail-closed
    let tp = match tee_proof { None => return GitrustOutput { ok:false, reason:"TEE_PROOF_MISSING".to_string(), stage:"TEE".to_string(), receipt:None }, Some(p)=>p };
    if let Err(e) = verify_tee_proof(&tp, &rustc.code_hash, now) { return GitrustOutput { ok:false, reason:e, stage:"TEE".to_string(), receipt:None } };
    // 7 zkVM linkage only
    let steps: Vec<(String,String)> = vil_exec.proof_chain.iter().map(|s| (s.step_id.clone(), s.output_commitment.clone())).collect();
    let (zk_proofs, zk_chain_hash) = prove_chain_linkage(&steps, &rustc.code_hash);
    if let Err(e) = verify_chain_linkage(&zk_proofs) { return GitrustOutput { ok:false, reason:e, stage:"ZKVM".to_string(), receipt:None } };
    // 8 SIP
    let mut sip = SipStore::new();
    let sip_snap = sip.update(&agent.did, json!({"total": vil_exec.total, "vilCommitment": vil_exec.commitment, "zkChainHash": zk_chain_hash, "rustcCodeHash": rustc.code_hash}), now);
    let sip_root = match sip.verify_chain(&agent.did) { Ok(r)=>r, Err(e)=> return GitrustOutput { ok:false, reason:e, stage:"SIP".to_string(), receipt:None } };
    // 9 ATP
    let atp = make_atp_packet(&vil_compiled.intent_hash, json!({"total": vil_exec.total}), &zk_chain_hash, &vil_exec.commitment, now, nonce);
    if let Err(e) = verify_atp_packet(&atp, &vil_compiled.intent_hash) { return GitrustOutput { ok:false, reason:e, stage:"ATP".to_string(), receipt:None } };
    // 10 VIL verify
    if let Err(e) = verify_sealed(&vil_compiled, &vil_exec.sealed, &vil_exec.escrow) { return GitrustOutput { ok:false, reason:e, stage:"VIL_VERIFY".to_string(), receipt:None } };
    // 11 git-core
    let mut git = GitCore::new();
    let proof = json!({"envelopeId": envelope.id, "policyHash": envelope.policy_hash, "vilIntentHash": vil_compiled.intent_hash, "vilCommitment": vil_exec.commitment, "rustcCodeHash": rustc.code_hash, "teeMeasurement": tp.measurement, "zkChainHash": zk_chain_hash, "sipRoot": sip_root, "atpIntentHash": atp.intent_hash, "starkVerified": false, "onchainSettled": false});
    let commit = match git.commit(&[("intent.vil".to_string(), vil_source.to_string()), ("main.rc.rs".to_string(), rustc_source.to_string())], &format!("gitrust: {}", vil_compiled.summary), None, Some("task:demo".to_string()), Some(sha256_hex(serde_json::to_string(&Value::Array(source_bytes.to_vec())).unwrap().as_bytes())), Some(rustc.code_hash.clone()), Some(vil_exec.commitment.clone()), Some(agent.did.clone()), Some(proof), now) { Ok(c)=>c, Err(e)=> return GitrustOutput { ok:false, reason:e, stage:"GIT".to_string(), receipt:None } };
    if let Err(e) = git.verify_chain() { return GitrustOutput { ok:false, reason:e, stage:"GIT_VERIFY".to_string(), receipt:None } };
    // 12 x402 escrow HELD
    let mut x = X402Escrow::new();
    let escrow_id = vil_exec.escrow.id.clone();
    let _ = VilEscrow { id: escrow_id.clone(), amount_cents: price_cents, status: "HELD".to_string(), created_at: now.to_string() };
    let xesc = XEscrow { id: escrow_id.clone(), amount_cents: price_cents, payee: payee.to_string(), idempotency_key: format!("poc:{}:{}:{}", commit.hash, payee, price_cents), status: "HELD".to_string(), created_at: now.to_string() };
    let _ = x.create(xesc);
    let _ = fee_cents(price_cents);
    let _ = sip_snap;
    let receipt = json!({"version":"gitrust-c-hygienic/1.0","gitCommit": commit.hash, "envelopeId": envelope.id, "policy":{"allowed":true}, "vil":{"intentHash": vil_compiled.intent_hash, "commitment": vil_exec.commitment, "summary": vil_compiled.summary}, "rustc":{"codeHash": rustc.code_hash, "wasmBytes": rustc.wasm.len()}, "tee":{"supplied":true,"measurement": tp.measurement}, "zk":{"linkageOk":true,"starkVerified":false,"chainHash": zk_chain_hash}, "sip":{"root": sip_root, "ok":true}, "atp":{"ok":true,"intentHash": atp.intent_hash}, "x402":{"escrowStatus":"HELD","onchainSettled":false}, "total": vil_exec.total, "verifiedHashChecks": true, "settlementClaim":"HELD unless on-chain proof supplied"});
    let receipt_hash = canonical_hash(&receipt);
    let mut r = receipt.clone(); if let Value::Object(ref mut m) = r { m.insert("receiptHash".to_string(), Value::String(receipt_hash)); }
    GitrustOutput { ok: true, reason: "OK".to_string(), stage: "COMMIT".to_string(), receipt: Some(r) }
}
