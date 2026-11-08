use crate::canonical::{canonical_hash, sha256_hex};
use serde_json::{json, Value};
use std::fmt;

#[derive(Debug, Clone)]
pub struct VilError { pub code: String, pub message: String }
impl VilError {
    pub fn new(code: &str, message: &str) -> Self { Self { code: code.to_string(), message: message.to_string() } }
}
impl fmt::Display for VilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}: {}", self.code, self.message) }
}
impl std::error::Error for VilError {}

#[derive(Debug, Clone)]
pub struct Parsed {
    pub intent: String,
    pub params: String,
    pub effects: String,
    pub trust: String,
    pub max_cost: f64,
    pub body: String,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct Compiled {
    pub intent: String,
    pub params: String,
    pub effects: String,
    pub trust: String,
    pub max_cost: f64,
    pub body: String,
    pub summary: String,
    pub intent_hash: String,
    pub summary_hash: String,
}

pub fn parse_vil(src: &str) -> Result<Parsed, VilError> {
    if !src.contains("intent ") { return Err(VilError::new("VIL_NO_INTENT", "Missing intent")); }
    // intent <name>(<params>)
    let intent_start = src.find("intent ").ok_or_else(|| VilError::new("VIL_PARSE", "Cannot find intent"))? + "intent ".len();
    let rest = &src[intent_start..];
    let name_end = rest.find(|c: char| c == '(' || c.is_whitespace()).ok_or_else(|| VilError::new("VIL_PARSE", "Cannot parse intent name"))?;
    let intent = rest[..name_end].trim().to_string();
    let paren_start = rest.find('(').ok_or_else(|| VilError::new("VIL_PARSE", "Cannot parse params"))?;
    let paren_end = rest.find(')').ok_or_else(|| VilError::new("VIL_PARSE", "Cannot parse params end"))?;
    let params = rest[paren_start + 1..paren_end].trim().to_string();
    // effects: [...]
    let effects = if let Some(s) = src.find("effects:") {
        let after = &src[s + "effects:".len()..];
        if let Some(lb) = after.find('[') {
            if let Some(rb) = after.find(']') {
                after[lb + 1..rb].trim().to_string()
            } else { String::new() }
        } else { String::new() }
    } else { String::new() };
    // trust: <line>
    let trust = if let Some(s) = src.find("trust:") {
        let after = &src[s + "trust:".len()..];
        let line_end = after.find('\n').unwrap_or(after.len());
        after[..line_end].trim().to_string()
    } else { String::new() };
    // max_cost: <number>
    let mut max_cost = 0.0f64;
    if let Some(s) = src.find("max_cost:") {
        let after = src[s + "max_cost:".len()..].trim_start();
        let mut num = String::new();
        for c in after.chars() {
            if c.is_ascii_digit() || c == '.' { num.push(c); } else { break; }
        }
        if let Ok(v) = num.parse::<f64>() { max_cost = v; }
    }
    let body = if let Some(b) = src.find('{') { src[b..].to_string() } else { String::new() };
    Ok(Parsed { intent, params, effects, trust, max_cost, body, source: src.to_string() })
}

pub fn generate_summary(p: &Parsed) -> String {
    format!(
        "Intent {} reads {} under trust {} with max cost ${}. Output is sealed and escrowed; settlement releases only on verification.",
        p.intent,
        if p.effects.is_empty() { "no declared effects".to_string() } else { p.effects.clone() },
        if p.trust.is_empty() { "unspecified".to_string() } else { p.trust.clone() },
        p.max_cost
    )
}

pub fn compile_vil(src: &str) -> Result<Compiled, VilError> {
    let p = parse_vil(src)?;
    if src.contains("settle(") { return Err(VilError::new("SETTLE_BANNED", "settle() banned — use escrow()")); }
    if !src.contains("escrow(") { return Err(VilError::new("ESCROW_REQUIRED", "VIL 0.2 requires escrow()")); }
    if !src.contains("Committed<") { return Err(VilError::new("COMMIT_REQUIRED", "VIL 0.2 requires Committed<RawBytes> from fetch() inside TEE")); }
    if !src.contains("Validated<") { return Err(VilError::new("PROOF_CARRYING_TYPE_VIOLATION", "Missing Validated<T>")); }
    if src.contains("aggregate(committed") || src.contains("aggregate( committed") { return Err(VilError::new("PROOF_CARRYING_TYPE_VIOLATION", "aggregate() on Committed<RawBytes> — must aggregate Validated")); }
    if src.contains("aggregate(raw") { return Err(VilError::new("PROOF_CARRYING_TYPE_VIOLATION", "aggregate() on RawBytes")); }
    if p.effects.is_empty() { return Err(VilError::new("EFFECTS_REQUIRED", "Effects required")); }
    if p.trust.is_empty() { return Err(VilError::new("TRUST_REQUIRED", "Trust required in signature")); }
    let summary = generate_summary(&p);
    let intent_hash = sha256_hex(canonical_hash(&json!({
        "intent": p.intent, "params": p.params, "effects": p.effects, "trust": p.trust, "maxCost": p.max_cost, "body": p.body, "summary": summary
    })).as_bytes());
    let summary_hash = sha256_hex(summary.as_bytes());
    let _ = Value::Null;
    Ok(Compiled { intent: p.intent, params: p.params, effects: p.effects, trust: p.trust, max_cost: p.max_cost, body: p.body, summary, intent_hash, summary_hash })
}
