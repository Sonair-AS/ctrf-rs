use crate::{extra::Extra, impl_extra};

use std::{collections::HashMap, default::Default};

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// All fields are optional.  Because they number so many, normally this would lead to a builder;
/// however, in this case we just derive `Default` and let the end implementer access as needed.
#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_release: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_environment: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, Value>,
}

// Necessary or no?
impl Environment {
    pub fn new() -> Self {
        Self::default()
    }
}

impl_extra!(Environment);
