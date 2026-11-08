use crate::canonical::sha256_hex;
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct CompiledRustC { pub fn_name: String, pub code_hash: String, pub wasm: Vec<u8>, pub source: String, pub state_name: String, pub input_name: String, pub lets: Vec<(String,String)>, pub tail: String }
pub fn compile_rustc(source: &str) -> Result<CompiledRustC, String> {
    if source.contains("f32") || source.contains("f64") { return Err("Rust-C: floats banned".to_string()); }
    if source.contains("std::time") || source.contains("SystemTime") || source.contains("rand") || source.contains("println!") || source.contains("std::io") { return Err("Rust-C: time / IO / randomness banned".to_string()); }
    // fn <name>(<state>: u32, <input>: u32) -> u32
    let fn_pos = source.find("fn ").ok_or("Rust-C: expected fn <name>(state: u32, input: u32) -> u32")?;
    let rest = &source[fn_pos+3..];
    let name_end = rest.find(|c: char| c=='(' || c.is_whitespace()).ok_or("Rust-C: parse fn name")?;
    let fn_name = rest[..name_end].trim().to_string();
    let p_start = rest.find('(').ok_or("Rust-C: parse params")?;
    let p_end = rest.find(')').ok_or("Rust-C: parse params end")?;
    let params = &rest[p_start+1..p_end];
    let parts: Vec<&str> = params.split(',').collect();
    if parts.len()!=2 { return Err("Rust-C: expected 2 params".to_string()); }
    let state_name = parts[0].split(':').next().unwrap_or("").trim().to_string();
    let input_name = parts[1].split(':').next().unwrap_or("").trim().to_string();
    // body between { and last }
    let b_start = source.find('{').ok_or("Rust-C: no body")?;
    let b_end = source.rfind('}').ok_or("Rust-C: no body end")?;
    let body = &source[b_start+1..b_end];
    let mut lets = Vec::new();
    let mut tail = String::new();
    for stmt in body.split(';') {
        let s = stmt.trim();
        if s.is_empty() { continue; }
        if s.starts_with("let ") {
            let after = &s[4..];
            let eq = after.find('=').ok_or("Rust-C: let without =")?;
            let lhs = after[..eq].trim();
            let var = lhs.split(':').next().unwrap_or("").trim().to_string();
            let expr = after[eq+1..].trim().to_string();
            lets.push((var, expr));
        } else { tail = s.to_string(); }
    }
    if tail.is_empty() { tail = fn_name.clone(); }
    let mut wasm = vec![0,97,115,109,1,0,0,0];
    wasm.extend_from_slice(&hex::decode(sha256_hex(source.as_bytes())).unwrap());
    Ok(CompiledRustC { fn_name, code_hash: sha256_hex(source.as_bytes()), wasm, source: source.to_string(), state_name, input_name, lets, tail })
}
struct ExprParser { chars: Vec<char>, pos: usize, env: HashMap<String,u32> }
impl ExprParser {
    fn skip(&mut self) { while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() { self.pos+=1; } }
    fn peek(&mut self) -> Option<char> { self.skip(); self.chars.get(self.pos).cloned() }
    fn parse_expr(&mut self) -> Result<u32,String> { let mut v = self.parse_term()?; loop { match self.peek() { Some('+') => { self.pos+=1; let r=self.parse_term()?; v=v.wrapping_add(r); } Some('-') => { self.pos+=1; let r=self.parse_term()?; v=v.wrapping_sub(r); } _ => break } } Ok(v) }
    fn parse_term(&mut self) -> Result<u32,String> { let mut v = self.parse_factor()?; loop { match self.peek() { Some('*') => { self.pos+=1; let r=self.parse_factor()?; v=v.wrapping_mul(r); } Some('/') => { self.pos+=1; let r=self.parse_factor()?; v=if r==0 {0} else {v/r}; } Some('%') => { self.pos+=1; let r=self.parse_factor()?; v=if r==0 {0} else {v%r}; } _ => break } } Ok(v) }
    fn parse_factor(&mut self) -> Result<u32,String> {
        self.skip();
        if self.peek()==Some('(') { self.pos+=1; let v=self.parse_expr()?; self.skip(); if self.peek()==Some(')') { self.pos+=1; } return Ok(v); }
        // number or ident
        let mut name = String::new();
        while let Some(c)=self.chars.get(self.pos).cloned() { if c.is_alphanumeric() || c=='_' { name.push(c); self.pos+=1; } else { break; } }
        if name.is_empty() { return Err(format!("Rust-C: cannot parse expr at {}", self.pos)); }
        if name.chars().next().unwrap().is_ascii_digit() { return Ok(name.parse::<u32>().map_err(|e| e.to_string())?); }
        // method: .wrapping_add( expr )
        self.skip();
        if self.peek()==Some('.') {
            self.pos+=1;
            let mut meth = String::new();
            while let Some(c)=self.chars.get(self.pos).cloned() { if c.is_alphanumeric() || c=='_' { meth.push(c); self.pos+=1; } else { break; } }
            self.skip();
            if self.peek()==Some('(') { self.pos+=1; let arg=self.parse_expr()?; self.skip(); if self.peek()==Some(')') { self.pos+=1; }
                let base = self.env.get(&name).cloned().ok_or(format!("Rust-C: unknown var {}", name))?;
                return Ok(match meth.as_str() { "wrapping_add"=>base.wrapping_add(arg), "wrapping_sub"=>base.wrapping_sub(arg), "wrapping_mul"=>base.wrapping_mul(arg), _=>return Err(format!("Rust-C: unknown method {}", meth)) });
            }
        }
        self.env.get(&name).cloned().ok_or(format!("Rust-C: unknown var {}", name))
    }
}
impl CompiledRustC {
    pub fn step(&self, state: u32, input: u32) -> u32 {
        let mut env = HashMap::new();
        env.insert(self.state_name.clone(), state);
        env.insert(self.input_name.clone(), input);
        for (var, expr) in &self.lets {
            let mut p = ExprParser { chars: expr.chars().collect(), pos: 0, env: env.clone() };
            if let Ok(v)=p.parse_expr() { env.insert(var.clone(), v); }
        }
        if let Some(v)=env.get(&self.tail) { return *v; }
        let mut p = ExprParser { chars: self.tail.chars().collect(), pos: 0, env };
        p.parse_expr().unwrap_or(0)
    }
}
