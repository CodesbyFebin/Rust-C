use std::{env, fs, process};
use serde_json::{json, Value};
use rustc_hygienic::{compile_vil, execute_ir, verify_sealed, compile_rustc, gitrust_commit_hygienic, Agent, TeeProof};
use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand::rngs::OsRng;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { eprintln!("Usage: rustc-hygienic <vil|build|commit> ..."); process::exit(2); }
    match args[1].as_str() {
        "vil" => {
            let src = fs::read_to_string(&args[2]).expect("read .vil");
            let c = compile_vil(&src).unwrap_or_else(|e| { eprintln!("VIL error: {}", e); process::exit(1); });
            println!("intent {} hash {}", c.intent, c.intent_hash);
            println!("summary {}", c.summary);
            let ex = execute_ir(&c, &[json!({"revenue":100}), json!({"revenue":250}), json!({"revenue":50})], "attested-url-tls:example.com/sales", "nonce-123", "2026-11-08T04:00:00.000Z").unwrap();
            println!("execute total {} commitment {}", ex.total, ex.commitment);
            println!("verify {:?}", verify_sealed(&c, &ex.sealed, &ex.escrow));
        }
        "build" => {
            let src = fs::read_to_string(&args[2]).expect("read .rs");
            let c = compile_rustc(&src).unwrap();
            println!("built {} wasm {} bytes codeHash {}", c.fn_name, c.wasm.len(), c.code_hash);
        }
        "commit" => {
            let vil_src = fs::read_to_string(&args[2]).expect("read vil");
            let rustc_src = fs::read_to_string(&args[3]).expect("read rs");
            // Real TEE proof would come from hardware. For demo CLI, we create a TEE keypair and sign measurement — same interface as production, signature verified in gitrust.rs.
            let tee_sk = SigningKey::generate(&mut OsRng);
            let tee_vk: VerifyingKey = tee_sk.verifying_key();
            let rc = compile_rustc(&rustc_src).unwrap();
            let msg = rustc_hygienic::canonical::canonical_stringify(&json!({"measurement": rc.code_hash, "codeHash": rc.code_hash}));
            let sig = tee_sk.sign(msg.as_bytes());
            let tee_proof = TeeProof { measurement: rc.code_hash.clone(), code_hash: rc.code_hash.clone(), signature_hex: hex::encode(sig.to_bytes()), verifying_key_hex: hex::encode(tee_vk.to_bytes()), ref_: Some("tee:demo-key:quote-123".to_string()), timestamp: "2026-11-08T04:00:00.000Z".to_string() };
            let owner = Agent::new("owner");
            let agent = Agent::new("agent");
            let policy = json!({"version":1,"agentDid":agent.did,"perTaskCents":2500,"dailyCents":5000,"maxSpendCents":10000,"hitlThresholdCents":5000,"allowedActions":["process-images"],"allowlist":["did:web:acme.com"],"blocklist":[]});
            let res = gitrust_commit_hygienic(&vil_src, &rustc_src, &[json!({"revenue":100}), json!({"revenue":250}), json!({"revenue":50})], &owner, &agent, &policy, "did:web:acme.com", 1, "2026-11-08T04:00:00.000Z", "nonce-123", "env_001", Some(tee_proof));
            println!("{}", serde_json::to_string_pretty(&res.receipt.unwrap_or(Value::Null)).unwrap());
            if !res.ok { process::exit(1); }
        }
        _ => { eprintln!("Usage: rustc-hygienic <vil|build|commit>"); process::exit(2); }
    }
}
