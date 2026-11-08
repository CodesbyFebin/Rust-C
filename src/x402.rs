use crate::canonical::sha256_hex;
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct Escrow { pub id: String, pub amount_cents: u64, pub payee: String, pub idempotency_key: String, pub status: String, pub created_at: String }
pub fn fee_cents(a: u64) -> u64 { let f = a/200; std::cmp::min(f, if a<500 {100} else {1000}) }
pub fn verify_facilitator_formula(tx_hash: &str, proof_hash: &str, amount: u64, payee: &str, idem: &str) -> (bool, bool) {
    let e = sha256_hex(format!("{}:{}:{}:{}", tx_hash, amount, payee, idem).as_bytes());
    (e==proof_hash, false)
}
pub struct X402Escrow { pub map: HashMap<String, Escrow> }
impl X402Escrow {
    pub fn new() -> Self { Self { map: HashMap::new() } }
    pub fn create(&mut self, e: Escrow) -> Escrow { self.map.insert(e.id.clone(), e.clone()); e }
    pub fn release_onchain(&mut self, id: &str, tx: &str, verified: bool) -> Result<Escrow,String> {
        let e = self.map.get_mut(id).ok_or("ESCROW_NOT_FOUND")?;
        if e.status!="HELD" { return Err(format!("ESCROW_NOT_HELD:{}", e.status)); }
        if !verified || tx.is_empty() { return Err("ONCHAIN_SETTLEMENT_NOT_PROVEN".to_string()); }
        e.status="RELEASED".to_string(); Ok(e.clone())
    }
}
