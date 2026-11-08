// HYGIENIC TEE: NO MOCK. No mock quote passes. Caller must supply real TEE proof.
// verify_tee_proof checks measurement == codeHash and Ed25519 signature with supplied TEE verifying key.
use ed25519_dalek::{VerifyingKey, Signature, Verifier};
use crate::canonical::canonical_stringify;
use serde_json::json;
#[derive(Clone, Debug)]
pub struct TeeProof { pub measurement: String, pub code_hash: String, pub signature_hex: String, pub verifying_key_hex: String, pub ref_: Option<String>, pub timestamp: String }
pub fn verify_tee_proof(p: &TeeProof, expected_code_hash: &str, now: &str) -> Result<(), String> {
    if p.measurement.is_empty() || p.code_hash.is_empty() || p.signature_hex.is_empty() { return Err("TEE_PROOF_MISSING_OR_INCOMPLETE".to_string()); }
    if p.code_hash != expected_code_hash { return Err("CODE_HASH_MISMATCH".to_string()); }
    if p.measurement != p.code_hash { return Err("MEASUREMENT_MISMATCH".to_string()); }
    if p.timestamp > now.to_string() { return Err("TEE_TIMESTAMP_IN_FUTURE".to_string()); }
    let vk_bytes = hex::decode(&p.verifying_key_hex).map_err(|_| "TEE_BAD_VERIFYING_KEY".to_string())?;
    let vk_arr: [u8;32] = vk_bytes.try_into().map_err(|_| "TEE_BAD_VERIFYING_KEY".to_string())?;
    let vk = VerifyingKey::from_bytes(&vk_arr).map_err(|_| "TEE_BAD_VERIFYING_KEY".to_string())?;
    let sig_bytes = hex::decode(&p.signature_hex).map_err(|_| "TEE_BAD_SIGNATURE".to_string())?;
    let sig = Signature::from_slice(&sig_bytes).map_err(|_| "TEE_BAD_SIGNATURE".to_string())?;
    let msg = json!({"measurement": p.measurement, "codeHash": p.code_hash});
    vk.verify(canonical_stringify(&msg).as_bytes(), &sig).map_err(|_| "TEE_SIGNATURE_MISMATCH".to_string())?;
    Ok(())
}
