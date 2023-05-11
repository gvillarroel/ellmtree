#[warn(dead_code)]
use serde_json::Value;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub name: String,
    pub template: Option<String>,
    pub tools: Option<HashMap<String, Profile>>,
    pub config: Value,
}