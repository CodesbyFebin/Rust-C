use crate::canonical::canonical_hash;
use crate::compiler::Compiled;
use crate::executor::{Escrow, Sealed};
use serde_json::{json, Value};

/// Verify a sealed VIL output. Hashes only — never re-executes aggregation.
pub fn verify_sealed(compiled: &Compiled, sealed: &Sealed, escrow: &Escrow) -> Result<u64, String> {
    if sealed.intent_hash != compiled.intent_hash { return Err("INTENT_HASH_MISMATCH".to_string()); }
    if sealed.summary_hash != compiled.summary_hash { return Err("SUMMARY_DIVERGED".to_string()); }
    let proof_json: Vec<Value> = sealed.proof_chain.iter().map(|s| json!({"stepId": s.step_id, "outputCommitment": s.output_commitment})).collect();
    let expected = canonical_hash(&json!({"total": sealed.total, "proofChain": proof_json, "escrowId": sealed.escrow_id, "intentHash": sealed.intent_hash}));
    if expected != sealed.commitment { return Err("SEAL_COMMITMENT_MISMATCH".to_string()); }
    if escrow.status != "HELD" && escrow.status != "RELEASED" { return Err("ESCROW_NOT_HELD".to_string()); }
    Ok(sealed.total)
}
