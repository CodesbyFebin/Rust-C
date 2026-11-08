use crate::canonical::{canonical_hash, canonical_stringify, sha256_hex};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Agent { pub did: String, pub signing_key: SigningKey, pub verifying_key: VerifyingKey, pub name: String }
impl Agent {
    pub fn new(name: &str) -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        let did = format!("did:key:z{}", hex::encode(verifying_key.to_bytes()));
        Self { did, signing_key, verifying_key, name: name.to_string() }
    }
    pub fn sign(&self, msg: &[u8]) -> String { hex::encode(self.signing_key.sign(msg).to_bytes()) }
    pub fn verify(&self, msg: &[u8], sig_hex: &str) -> bool {
        let bytes = match hex::decode(sig_hex) { Ok(b)=>b, Err(_)=>return false };
        let arr: [u8;64] = match bytes.try_into() { Ok(a)=>a, Err(_)=>return false };
        let sig = match Signature::from_slice(&bytes) { Ok(s)=>s, Err(_)=>return false };
        self.verifying_key.verify(msg, &sig).is_ok()
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Envelope { pub version: String, pub id: String, pub issuer: String, pub subject: Option<String>, pub intent: Value, pub policy: Value, pub policy_hash: String, pub expires_at: String, pub created_at: String, pub nonce: String, pub status: String, pub signature: String, pub hash: String }
pub fn create_envelope(issuer: &Agent, subject_did: Option<String>, intent: Value, policy: Value, expires_at: &str, created_at: &str, nonce: &str, id: &str) -> Envelope {
    let policy_hash = canonical_hash(&policy);
    let unsigned = json!({"version":"envelope-spec/0.1","id":id,"issuer":issuer.did,"subject":subject_did,"intent":intent,"policy":policy,"policyHash":policy_hash,"expiresAt":expires_at,"createdAt":created_at,"nonce":nonce,"status":"CREATED"});
    let sig = issuer.sign(canonical_stringify(&unsigned).as_bytes());
    let with_sig = json!({"version":"envelope-spec/0.1","id":id,"issuer":issuer.did,"subject":subject_did,"intent":intent,"policy":policy,"policyHash":policy_hash,"expiresAt":expires_at,"createdAt":created_at,"nonce":nonce,"status":"CREATED","signature":sig});
    let hash = canonical_hash(&with_sig);
    Envelope { version:"envelope-spec/0.1".to_string(), id:id.to_string(), issuer:issuer.did.clone(), subject:subject_did, intent, policy, policy_hash, expires_at:expires_at.to_string(), created_at:created_at.to_string(), nonce:nonce.to_string(), status:"CREATED".to_string(), signature:sig, hash }
}
pub fn verify_envelope(env: &Envelope, issuer: &Agent, seen_nonces: &mut std::collections::HashSet<String>, now: &str) -> Result<(), String> {
    if env.expires_at < now.to_string() { return Err("ENVELOPE_EXPIRED".to_string()); }
    if canonical_hash(&env.policy) != env.policy_hash { return Err("POLICY_HASH_MISMATCH".to_string()); }
    if seen_nonces.contains(&env.nonce) { return Err("REPLAY_NONCE".to_string()); }
    let unsigned = json!({"version":env.version,"id":env.id,"issuer":env.issuer,"subject":env.subject,"intent":env.intent,"policy":env.policy,"policyHash":env.policy_hash,"expiresAt":env.expires_at,"createdAt":env.created_at,"nonce":env.nonce,"status":env.status});
    if !issuer.verify(canonical_stringify(&unsigned).as_bytes(), &env.signature) { return Err("BAD_ENVELOPE_SIGNATURE".to_string()); }
    let with_sig = json!({"version":env.version,"id":env.id,"issuer":env.issuer,"subject":env.subject,"intent":env.intent,"policy":env.policy,"policyHash":env.policy_hash,"expiresAt":env.expires_at,"createdAt":env.created_at,"nonce":env.nonce,"status":env.status,"signature":env.signature});
    if canonical_hash(&with_sig) != env.hash { return Err("ENVELOPE_HASH_MISMATCH".to_string()); }
    let _ = sha256_hex(b"x");
    Ok(())
}
#[derive(Clone)]
pub struct PolicyEngine { pub spent: HashMap<String,u64>, pub velocity: HashMap<String,u32>, pub kill: std::collections::HashSet<String> }
impl PolicyEngine {
    pub fn new() -> Self { Self { spent: HashMap::new(), velocity: HashMap::new(), kill: std::collections::HashSet::new() } }
    pub fn evaluate(&self, policy: &Value, envelope: &Envelope, agent_did: &str, amount_cents: u64, counterparty: &str, now: &str) -> Result<(), String> {
        if self.kill.contains(agent_did) { return Err("KILL_SWITCH_ACTIVE".to_string()); }
        if let Some(a)=policy.get("agentDid").and_then(|v| v.as_str()) { if a!=agent_did { return Err("POLICY_AGENT_MISMATCH".to_string()); } }
        if canonical_hash(policy) != envelope.policy_hash { return Err("STALE_POLICY_SNAPSHOT".to_string()); }
        if envelope.expires_at < now.to_string() { return Err("ENVELOPE_EXPIRED".to_string()); }
        if let Some(bl)=policy.get("blocklist").and_then(|v| v.as_array()) { if bl.iter().any(|x| x.as_str()==Some(counterparty)) { return Err("COUNTERPARTY_BLOCKED".to_string()); } }
        if let Some(al)=policy.get("allowlist").and_then(|v| v.as_array()) { if !al.is_empty() && !al.iter().any(|x| x.as_str()==Some(counterparty)) { return Err("COUNTERPARTY_NOT_ALLOWLISTED".to_string()); } }
        if let Some(m)=policy.get("perTaskCents").and_then(|v| v.as_u64()) { if amount_cents>m { return Err("PER_TASK_EXCEEDED".to_string()); } }
        if let Some(m)=policy.get("maxSpendCents").and_then(|v| v.as_u64()) { if amount_cents>m { return Err("MAX_SPEND_EXCEEDED".to_string()); } }
        if let Some(ht)=policy.get("hitlThresholdCents").and_then(|v| v.as_u64()) { if amount_cents>=ht { return Err("HITL_REQUIRED".to_string()); } }
        Ok(())
    }
}
