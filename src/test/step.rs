use crate::{extra::Extra, impl_extra, test::Status};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    /// Required
    pub name: String,
    /// Required
    pub status: Status,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, Value>,
}

impl_extra!(Step);
