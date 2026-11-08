use crate::canonical::{canonical_hash, sha256_hex};
use serde_json::{json, Value};
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct SipSnapshot { pub agent_id: String, pub version: u64, pub prev_root: String, pub state_hash: String, pub new_root: String, pub state: Value, pub at: String }
pub struct SipStore { pub states: HashMap<String, SipSnapshot>, pub history: HashMap<String, Vec<SipSnapshot>> }
impl SipStore {
    pub fn new() -> Self { Self { states: HashMap::new(), history: HashMap::new() } }
    pub fn update(&mut self, agent_id: &str, state: Value, at: &str) -> SipSnapshot {
        let prev = self.states.get(agent_id);
        let prev_root = prev.map(|s| s.new_root.clone()).unwrap_or_else(|| "0".repeat(64));
        let version = prev.map(|s| s.version+1).unwrap_or(1);
        let state_hash = canonical_hash(&state);
        let new_root = sha256_hex(format!("{}:{}", prev_root, state_hash).as_bytes());
        let snap = SipSnapshot { agent_id: agent_id.to_string(), version, prev_root, state_hash, new_root, state, at: at.to_string() };
        self.states.insert(agent_id.to_string(), snap.clone());
        self.history.entry(agent_id.to_string()).or_default().push(snap.clone());
        snap
    }
    pub fn verify_chain(&self, agent_id: &str) -> Result<String, String> {
        let hist = self.history.get(agent_id).ok_or("NO_HISTORY")?;
        if hist.is_empty() { return Err("NO_HISTORY".to_string()); }
        for (i,s) in hist.iter().enumerate() {
            if s.version != (i+1) as u64 { return Err("VERSION_GAP".to_string()); }
            if i>0 && s.prev_root != hist[i-1].new_root { return Err("PREV_ROOT_BROKEN".to_string()); }
            if s.new_root != sha256_hex(format!("{}:{}", s.prev_root, s.state_hash).as_bytes()) { return Err("ROOT_MISMATCH".to_string()); }
            if s.state_hash != canonical_hash(&s.state) { return Err("STATE_HASH_MISMATCH".to_string()); }
        }
        let _ = json!(null);
        Ok(hist.last().unwrap().new_root.clone())
    }
}
