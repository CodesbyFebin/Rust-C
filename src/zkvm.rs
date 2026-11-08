use crate::canonical::{canonical_hash, sha256_hex};
use serde_json::{json, Value};
#[derive(Clone, Debug)]
pub struct LinkageProof { pub step_id: String, pub input_commitment: String, pub output_commitment: String, pub program_hash: String, pub proof: String }
pub fn prove_chain_linkage(steps: &[(String,String)], program_hash: &str) -> (Vec<LinkageProof>, String) {
    let mut proofs = Vec::new(); let mut prev = "0".repeat(64);
    for (id, out) in steps { let proof = sha256_hex(format!("{}:{}:{}:{}", id, prev, out, program_hash).as_bytes()); proofs.push(LinkageProof { step_id: id.clone(), input_commitment: prev.clone(), output_commitment: out.clone(), program_hash: program_hash.to_string(), proof }); prev = out.clone(); }
    let arr: Vec<Value> = proofs.iter().map(|p| json!({"stepId": p.step_id, "inputCommitment": p.input_commitment, "outputCommitment": p.output_commitment, "programHash": p.program_hash, "proof": p.proof})).collect();
    (proofs, canonical_hash(&Value::Array(arr)))
}
pub fn verify_chain_linkage(proofs: &[LinkageProof]) -> Result<String, String> {
    for (i,p) in proofs.iter().enumerate() {
        let e = sha256_hex(format!("{}:{}:{}:{}", p.step_id, p.input_commitment, p.output_commitment, p.program_hash).as_bytes());
        if e != p.proof { return Err(format!("CHAIN_LINKAGE_BROKEN_AT_{}", i)); }
        if i>0 && p.input_commitment != proofs[i-1].output_commitment { return Err(format!("CHAIN_LINK_BROKEN_AT_{}", i)); }
    }
    let arr: Vec<Value> = proofs.iter().map(|p| json!({"stepId": p.step_id, "inputCommitment": p.input_commitment, "outputCommitment": p.output_commitment, "programHash": p.program_hash, "proof": p.proof})).collect();
    Ok(canonical_hash(&Value::Array(arr)))
}
