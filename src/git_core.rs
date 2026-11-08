use crate::canonical::{canonical_hash, sha256_hex};
use serde_json::{json, Value};
use std::collections::HashMap;

pub fn blob_hash(content: &[u8]) -> String {
    let mut data = format!("blob {}\0", content.len()).into_bytes();
    data.extend_from_slice(content);
    sha256_hex(&data)
}
pub fn tree_hash(entries: &[(String, String)]) -> String {
    let mut s = entries.to_vec();
    s.sort_by(|a, b| a.0.cmp(&b.0));
    let arr: Vec<Value> = s.iter().map(|(n, h)| json!({"name": n, "hash": h})).collect();
    canonical_hash(&Value::Array(arr))
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Commit {
    pub hash: String,
    pub tree: String,
    pub parent: Option<String>,
    pub message: String,
    pub task_id: Option<String>,
    pub input_hash: Option<String>,
    pub code_hash: Option<String>,
    pub output_hash: Option<String>,
    pub agent_id: Option<String>,
    pub proof: Option<Value>,
    pub at: String,
    pub signature: String,
}

pub struct GitCore {
    pub objects: HashMap<String, Value>,
    pub commits: Vec<Commit>,
    pub head: Option<String>,
}
impl GitCore {
    pub fn new() -> Self { Self { objects: HashMap::new(), commits: Vec::new(), head: None } }
    pub fn write_blob(&mut self, content: &[u8]) -> String {
        let h = blob_hash(content);
        self.objects.insert(h.clone(), Value::String(String::from_utf8_lossy(content).to_string()));
        h
    }
    pub fn write_tree(&mut self, files: &[(String, String)]) -> (String, Vec<(String, String)>) {
        let mut entries = Vec::new();
        for (name, content) in files {
            let h = self.write_blob(content.as_bytes());
            entries.push((name.clone(), h));
        }
        let h = tree_hash(&entries);
        self.objects.insert(h.clone(), Value::Array(entries.iter().map(|(n, hh)| json!({"name": n, "hash": hh})).collect()));
        (h, entries)
    }
    #[allow(clippy::too_many_arguments)]
    pub fn commit(&mut self, files: &[(String, String)], message: &str, parent: Option<String>, task_id: Option<String>, input_hash: Option<String>, code_hash: Option<String>, output_hash: Option<String>, agent_id: Option<String>, proof: Option<Value>, at: &str) -> Result<Commit, String> {
        if at.is_empty() { return Err("GIT_CORE_REQUIRES_EXPLICIT_TIME".to_string()); }
        let (tree, _) = self.write_tree(files);
        let payload = json!({
            "tree": tree, "parent": parent, "message": message,
            "taskId": task_id, "inputHash": input_hash, "codeHash": code_hash,
            "outputHash": output_hash, "agentId": agent_id, "proof": proof, "at": at,
        });
        let hash = canonical_hash(&payload);
        let signature = sha256_hex(format!("{}:{}", hash, agent_id.clone().unwrap_or_default()).as_bytes());
        let c = Commit { hash: hash.clone(), tree, parent, message: message.to_string(), task_id, input_hash, code_hash, output_hash, agent_id, proof, at: at.to_string(), signature };
        self.commits.push(c.clone());
        self.head = Some(hash);
        Ok(c)
    }
    pub fn verify_chain(&self) -> Result<(), String> {
        for (i, c) in self.commits.iter().enumerate() {
            let payload = json!({
                "tree": c.tree, "parent": c.parent, "message": c.message,
                "taskId": c.task_id, "inputHash": c.input_hash, "codeHash": c.code_hash,
                "outputHash": c.output_hash, "agentId": c.agent_id, "proof": c.proof, "at": c.at,
            });
            if canonical_hash(&payload) != c.hash { return Err(format!("COMMIT_HASH_MISMATCH_AT_{}", i)); }
            if i > 0 && c.parent != Some(self.commits[i-1].hash.clone()) { return Err(format!("PARENT_LINK_BROKEN_AT_{}", i)); }
            if i == 0 && c.parent.is_some() { return Err("GENESIS_MUST_HAVE_NULL_PARENT".to_string()); }
        }
        Ok(())
    }
}
