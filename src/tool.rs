use crate::{extra::Extra, impl_extra};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const TOOL_NAME: &str = "ctrf-rs";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    extra: HashMap<String, Value>,
}

impl Tool {
    pub fn new(name: &str, version: Option<String>) -> Self {
        Self {
            name: String::from(name),
            version,
            extra: HashMap::new(),
        }
    }
}

impl_extra!(Tool);
