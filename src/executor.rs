use crate::canonical::{canonical_hash, sha256_hex};
use crate::compiler::Compiled;
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct ProofStep { pub step_id: String, pub output_commitment: String }

#[derive(Debug, Clone)]
pub struct Escrow { pub id: String, pub amount_cents: u64, pub status: String, pub created_at: String }

#[derive(Debug, Clone)]
pub struct Sealed {
    pub commitment: String,
    pub proof_chain: Vec<ProofStep>,
    pub total: u64,
    pub escrow_id: String,
    pub intent_hash: String,
    pub summary_hash: String,
}

#[derive(Debug, Clone)]
pub struct Executed {
    pub total: u64,
    pub committed: String,
    pub validated: String,
    pub aggregated: String,
    pub proof_chain: Vec<ProofStep>,
    pub commitment: String,
    pub sealed: Sealed,
    pub escrow: Escrow,
}

/// Execute VIL. All inputs explicit: no wall-clock, no random.
/// source_bytes: e.g. [{"revenue":100},{"revenue":250},{"revenue":50}]
pub fn execute_ir(compiled: &Compiled, source_bytes: &[Value], source_attestation: &str, nonce: &str, now: &str) -> Result<Executed, String> {
    if source_attestation.is_empty() || nonce.is_empty() || now.is_empty() {
        return Err("VIL_EXEC_REQUIRES_EXPLICIT_INPUTS: source_attestation, nonce, now must be supplied".to_string());
    }
    let src_json = serde_json::to_string(&Value::Array(source_bytes.to_vec())).map_err(|e| e.to_string())?;
    let committed = sha256_hex(format!("{}:{}:{}", src_json, nonce, source_attestation).as_bytes());
    let validated = sha256_hex(format!("{}:validated", committed).as_bytes());
    let mut total: u64 = 0;
    for v in source_bytes {
        if let Some(r) = v.get("revenue").and_then(|x| x.as_u64()) { total = total.wrapping_add(r); }
    }
    let aggregated = sha256_hex(format!("{}:{}", validated, total).as_bytes());
    let proof_chain = vec![
        ProofStep { step_id: "fetch".to_string(), output_commitment: committed.clone() },
        ProofStep { step_id: "validate".to_string(), output_commitment: validated.clone() },
        ProofStep { step_id: "aggregate".to_string(), output_commitment: aggregated.clone() },
    ];
    let escrow_id = format!("esc_{}", &sha256_hex(format!("{}:{}", compiled.intent_hash, total).as_bytes())[..16]);
    let escrow = Escrow { id: escrow_id.clone(), amount_cents: 1, status: "HELD".to_string(), created_at: now.to_string() };
    let proof_json: Vec<Value> = proof_chain.iter().map(|s| json!({"stepId": s.step_id, "outputCommitment": s.output_commitment})).collect();
    let commitment = canonical_hash(&json!({"total": total, "proofChain": proof_json, "escrowId": escrow_id, "intentHash": compiled.intent_hash}));
    let sealed = Sealed { commitment: commitment.clone(), proof_chain: proof_chain.clone(), total, escrow_id: escrow_id.clone(), intent_hash: compiled.intent_hash.clone(), summary_hash: compiled.summary_hash.clone() };
    Ok(Executed { total, committed, validated, aggregated, proof_chain, commitment, sealed, escrow })
}
