use serde_json::{json, Value};
#[derive(Clone, Debug)]
pub struct AtpPacket { pub version: String, pub intent_hash: String, pub nonce: String, pub at: String, pub payload: Value, pub attestation: Value }
pub fn make_atp_packet(intent_hash: &str, payload: Value, zk_chain_hash: &str, vil_commitment: &str, at: &str, nonce: &str) -> AtpPacket {
    AtpPacket { version: "atp/1".to_string(), intent_hash: intent_hash.to_string(), nonce: nonce.to_string(), at: at.to_string(), payload, attestation: json!({"zkChainHash": zk_chain_hash, "vilCommitment": vil_commitment}) }
}
pub fn verify_atp_packet(p: &AtpPacket, expected_intent: &str) -> Result<(), String> {
    if p.version!="atp/1" { return Err("ATP_VERSION".to_string()); }
    if p.intent_hash!=expected_intent { return Err("ATP_INTENT_MISMATCH".to_string()); }
    if p.attestation.get("zkChainHash").is_none() { return Err("ATP_MISSING_ZK_CHAIN".to_string()); }
    Ok(())
}
