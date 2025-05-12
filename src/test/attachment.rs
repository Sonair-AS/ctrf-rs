use crate::{extra::Extra, impl_extra};

use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    /// Required
    pub name: String,
    /// Required
    pub content_type: String,
    /// Required
    pub path: PathBuf,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, Value>,
}

impl_extra!(Attachment);
