use crate::{
    extra::Extra,
    impl_extra,
    test::{attachment::Attachment, step::Step},
};

use std::{collections::HashMap, path::PathBuf, time::Duration, vec};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod attachment;
pub mod step;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Pending,
    Skipped,
    Failed,
    Passed,
    Other,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Test {
    name: String,
    status: Status,
    duration: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suite: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_status: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub test_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filepath: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flaky: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stdout: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stderr: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<Step>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<Attachment>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    extra: HashMap<String, Value>,
}

impl Test {
    pub fn new(name: String, status: Status, duration: Duration) -> Self {
        Self {
            name,
            status,
            duration: duration.as_millis() as u64,
            start: None,
            stop: None,
            suite: None,
            message: None,
            trace: None,
            line: None,
            ai: None,
            raw_status: None,
            tags: vec![],
            test_type: None,
            filepath: None,
            retries: None,
            flaky: None,
            stdout: vec![],
            stderr: vec![],
            thread_id: None,
            browser: None,
            device: None,
            screenshot: None,
            parameters: HashMap::new(),
            steps: vec![],
            attachments: vec![],
            extra: HashMap::new(),
        }
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn suite(&self) -> &Option<String> {
        &self.suite
    }
}

impl_extra!(Test);
