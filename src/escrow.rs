use crate::executor::Escrow;
use std::collections::HashMap;

/// Escrow state machine. Pure. Release only on caller-supplied on-chain proof.
/// This crate NEVER auto-releases on a mock. No mock settlement exists here.
pub struct EscrowManager { pub map: HashMap<String, Escrow> }
impl EscrowManager {
    pub fn new() -> Self { Self { map: HashMap::new() } }
    pub fn create(&mut self, e: Escrow) -> Escrow { self.map.insert(e.id.clone(), e.clone()); e }
    pub fn release_onchain(&mut self, id: &str, onchain_tx_hash: &str, onchain_verified: bool) -> Result<Escrow, String> {
        let e = self.map.get_mut(id).ok_or_else(|| "ESCROW_NOT_FOUND".to_string())?;
        if e.status != "HELD" { return Err(format!("ESCROW_NOT_HELD:{}", e.status)); }
        if !onchain_verified || onchain_tx_hash.is_empty() { return Err("ONCHAIN_SETTLEMENT_NOT_PROVEN".to_string()); }
        e.status = "RELEASED".to_string();
        Ok(e.clone())
    }
    pub fn revert(&mut self, id: &str, _reason: &str) -> Result<Escrow, String> {
        let e = self.map.get_mut(id).ok_or_else(|| "ESCROW_NOT_FOUND".to_string())?;
        e.status = "REVERTED".to_string();
        Ok(e.clone())
    }
}
