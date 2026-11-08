use serde_json::json;
use rustc_hygienic::{compile_vil, execute_ir, verify_sealed, compile_rustc, GitCore, Agent, gitrust_commit_hygienic, TeeProof};
use ed25519_dalek::{SigningKey, Signer};
use rand::rngs::OsRng;
fn defensive() -> String { std::fs::read_to_string("examples/defensive.vil").unwrap() }
fn main_rs() -> String { std::fs::read_to_string("examples/main.rc.rs").unwrap() }
#[test]
fn vil_compile_and_execute_400() {
    let c = compile_vil(&defensive()).unwrap();
    assert_eq!(c.intent, "process_sales_data");
    let ex = execute_ir(&c, &[json!({"revenue":100}), json!({"revenue":250}), json!({"revenue":50})], "attested-url-tls:example.com/sales", "nonce-123", "2026-11-08T04:00:00.000Z").unwrap();
    assert_eq!(ex.total, 400);
    assert_eq!(verify_sealed(&c, &ex.sealed, &ex.escrow).unwrap(), 400);
}
#[test]
fn vil_banned_settle() {
    let bad = defensive().replace("escrow(total", "settle(total");
    assert_eq!(compile_vil(&bad).unwrap_err().code, "SETTLE_BANNED");
}
#[test]
fn rustc_deterministic() {
    let c = compile_rustc(&main_rs()).unwrap();
    assert_eq!(c.step(0,100), 100);
    assert_eq!(c.step(100,250), 350);
}
#[test]
fn git_chain_tamper_breaks() {
    let mut g = GitCore::new();
    let c1 = g.commit(&[("a.txt".to_string(),"hello".to_string())], "c1", None, None, None, None, None, Some("a".to_string()), None, "2026-11-08T04:00:00.000Z").unwrap();
    let _ = g.commit(&[("b.txt".to_string(),"world".to_string())], "c2", Some(c1.hash), None, None, None, None, Some("a".to_string()), None, "2026-11-08T04:00:00.000Z").unwrap();
    assert!(g.verify_chain().is_ok());
    g.commits[1].message = "tampered".to_string();
    assert!(g.verify_chain().is_err());
}
#[test]
fn hygienic_no_tee_no_commit() {
    let owner = Agent::new("owner"); let agent = Agent::new("agent");
    let policy = json!({"version":1,"agentDid":agent.did,"perTaskCents":2500,"dailyCents":5000,"maxSpendCents":10000,"hitlThresholdCents":5000,"allowedActions":["process-images"],"allowlist":["did:web:acme.com"],"blocklist":[]});
    let r = gitrust_commit_hygienic(&defensive(), &main_rs(), &[json!({"revenue":100})], &owner, &agent, &policy, "did:web:acme.com", 1, "2026-11-08T04:00:00.000Z", "nonce-123", "env_001", None);
    assert!(!r.ok); assert_eq!(r.stage, "TEE");
}
#[test]
fn hygienic_with_tee_commit_held() {
    let owner = Agent::new("owner"); let agent = Agent::new("agent");
    let policy = json!({"version":1,"agentDid":agent.did,"perTaskCents":2500,"dailyCents":5000,"maxSpendCents":10000,"hitlThresholdCents":5000,"allowedActions":["process-images"],"allowlist":["did:web:acme.com"],"blocklist":[]});
    let rc = compile_rustc(&main_rs()).unwrap();
    let tee_sk = SigningKey::generate(&mut OsRng); let tee_vk = tee_sk.verifying_key();
    let msg = rustc_hygienic::canonical::canonical_stringify(&json!({"measurement": rc.code_hash, "codeHash": rc.code_hash}));
    let sig = tee_sk.sign(msg.as_bytes());
    let tp = TeeProof { measurement: rc.code_hash.clone(), code_hash: rc.code_hash.clone(), signature_hex: hex::encode(sig.to_bytes()), verifying_key_hex: hex::encode(tee_vk.to_bytes()), ref_: Some("tee:demo".to_string()), timestamp: "2026-11-08T04:00:00.000Z".to_string() };
    let r = gitrust_commit_hygienic(&defensive(), &main_rs(), &[json!({"revenue":100}), json!({"revenue":250}), json!({"revenue":50})], &owner, &agent, &policy, "did:web:acme.com", 1, "2026-11-08T04:00:00.000Z", "nonce-123", "env_001", Some(tp));
    assert!(r.ok);
    let rec = r.receipt.unwrap();
    assert_eq!(rec["total"], 400);
    assert_eq!(rec["zk"]["starkVerified"], false);
    assert_eq!(rec["x402"]["escrowStatus"], "HELD");
}
