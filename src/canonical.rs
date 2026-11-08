use sha2::{Digest, Sha256};
use serde_json::Value;
pub fn sha256_hex(data: &[u8]) -> String { let mut h = Sha256::new(); h.update(data); hex::encode(h.finalize()) }
pub fn canonical_stringify(v: &Value) -> String { serde_json::to_string(v).expect("canonical") }
pub fn canonical_hash(v: &Value) -> String { sha256_hex(canonical_stringify(v).as_bytes()) }
