use crate::{extra::Extra, impl_extra, results::Results};

use std::{
    collections::HashMap,
    io::{Read, Write},
    str::FromStr,
    time::SystemTime,
};

use semver::Version;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_json::{Result, Value};
use uuid::Uuid;

pub const REPORT_FORMAT: &str = "CTRF";
pub const SPEC_VERSION: Version = Version::new(0, 0, 0);

/// Top-level element for a CTRF report.
/// Corresponds to the spec's ["Root"](https://ctrf.io/docs/specification/root) object.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    #[serde(deserialize_with = "deserialize_format")]
    report_format: String,
    spec_version: Version,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_by: Option<String>,
    results: Results,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, Value>,
}

impl Report {
    /// Creates an instance of a CTRF report
    pub fn new(
        report_id: Option<Uuid>,
        timestamp: Option<SystemTime>,
        generated_by: Option<String>,
        results: Results,
    ) -> Self {
        Report {
            report_format: String::from(REPORT_FORMAT),
            spec_version: SPEC_VERSION,
            report_id,
            timestamp: timestamp.map(|ts| format!("{ts:?}")),
            generated_by,
            results,
            extra: HashMap::new(),
        }
    }

    /// Deserialize a `Report` instance from bytes of JSON text
    pub fn from_slice(s: &[u8]) -> Result<Self> {
        serde_json::from_slice(s)
    }

    /// Interpret a `serde_json::Value` as a `Report` instance
    pub fn from_value(v: Value) -> Result<Self> {
        serde_json::from_value(v)
    }

    /// Deserialize a `Report` instance from an I/O stream of JSON text
    pub fn from_reader(r: impl Read) -> Result<Self> {
        serde_json::from_reader(r)
    }

    /// Borrows the contained Results
    pub fn results(&self) -> &Results {
        &self.results
    }

    /// Outputs the report as a String of JSON
    pub fn to_string(&self) -> Result<String> {
        serde_json::to_string(self)
    }

    /// Outputs the report as a pretty-printed String of JSON
    pub fn to_string_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Outputs the report as a JSON byte vector
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self)
    }

    /// Outputs the report as a pretty-printed JSON byte vector
    pub fn to_vec_pretty(&self) -> Result<Vec<u8>> {
        serde_json::to_vec_pretty(self)
    }

    /// Outputs the report as JSON to the provided I/O stream
    pub fn to_writer(&self, writer: impl Write) -> Result<()> {
        serde_json::to_writer(writer, self)
    }

    /// Outputs the report as pretty-printed JSON to the provided I/O stream
    pub fn to_writer_pretty(&self, writer: impl Write) -> Result<()> {
        serde_json::to_writer_pretty(writer, self)
    }
}

impl FromStr for Report {
    type Err = serde_json::Error;

    /// Deserialize a `Report` instance from a string of JSON text
    fn from_str(s: &str) -> Result<Self> {
        serde_json::from_str(s)
    }
}

fn deserialize_format<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    if s == REPORT_FORMAT {
        Ok(s)
    } else {
        Err(D::Error::custom(format!(
            "unrecognized report format '{s}'"
        )))
    }
}

impl_extra!(Report);

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{results::ResultsBuilder, tool::Tool};

    use std::time::{Duration, UNIX_EPOCH};

    const TEMPLATE_JSON: &str = r#"{
  "reportFormat": "CTRF",
  "specVersion": "0.0.0",
  "results": {
    "tool": {
      "name": "ctrf-rs"
    },
    "summary": {
      "tests": 0,
      "passed": 0,
      "failed": 0,
      "pending": 0,
      "skipped": 0,
      "other": 0,
      "start": START,
      "stop": STOP
    },
    "tests": []
  }
}"#;

    #[test]
    fn create_empty_report() {
        let time = SystemTime::now();
        let results = ResultsBuilder::new(Tool::new(None)).build(time, time);
        let report = Report::new(None, None, None, results);

        assert_eq!(report.report_format, REPORT_FORMAT);
        assert_eq!(report.spec_version, SPEC_VERSION);
        assert_eq!(report.report_id, None);
        assert_eq!(report.timestamp, None);
        assert_eq!(report.generated_by, None);
    }

    #[test]
    fn create_report_with_id() {
        let time = SystemTime::now();
        let results = ResultsBuilder::new(Tool::new(None)).build(time, time);
        let id = Some(Uuid::new_v4());
        let report = Report::new(id, None, None, results);

        assert_eq!(report.report_format, REPORT_FORMAT);
        assert_eq!(report.spec_version, SPEC_VERSION);
        assert_eq!(report.report_id, id);
        assert_eq!(report.timestamp, None);
        assert_eq!(report.generated_by, None);
    }

    #[test]
    fn create_report_with_timestamp() {
        let time = SystemTime::now();
        let results = ResultsBuilder::new(Tool::new(None)).build(time, time);
        let ts = Some(time);
        let report = Report::new(None, ts, None, results);

        assert_eq!(report.report_format, REPORT_FORMAT);
        assert_eq!(report.spec_version, SPEC_VERSION);
        assert_eq!(report.report_id, None);
        assert_eq!(report.timestamp, Some(format!("{time:?}")));
        assert_eq!(report.generated_by, None);
    }

    #[test]
    fn create_report_with_generated_by() {
        let time = SystemTime::now();
        let results = ResultsBuilder::new(Tool::new(None)).build(time, time);
        let gen_by = Some(String::from("ctrf-rs"));
        let report = Report::new(None, None, gen_by, results);

        assert_eq!(report.report_format, REPORT_FORMAT);
        assert_eq!(report.spec_version, SPEC_VERSION);
        assert_eq!(report.report_id, None);
        assert_eq!(report.timestamp, None);
        assert_eq!(report.generated_by, Some(String::from("ctrf-rs")));
    }

    #[test]
    fn serialize_to_string() {
        let time = SystemTime::now();
        let results = ResultsBuilder::new(Tool::new(None)).build(time, time);
        let report = Report::new(None, None, None, results);

        assert_eq!(report.report_format, REPORT_FORMAT);
        assert_eq!(report.spec_version, SPEC_VERSION);

        let report_text = report.to_string().expect("report generation failed");
        let exp_text = r#"{"reportFormat":"CTRF","specVersion":"0.0.0","results":{"tool":{"name":"ctrf-rs"},"summary":{"tests":0,"passed":0,"failed":0,"pending":0,"skipped":0,"other":0,"start":START,"stop":STOP},"tests":[]}}"#;
        let time_str = time
            .duration_since(UNIX_EPOCH)
            .expect("time conversion error")
            .as_millis()
            .to_string();

        assert_eq!(
            report_text,
            exp_text
                .replace("START", &time_str)
                .replace("STOP", &time_str)
        );
    }

    #[test]
    fn serialize_to_string_pretty() {
        let time = SystemTime::now();
        let results = ResultsBuilder::new(Tool::new(None)).build(time, time);
        let report = Report::new(None, None, None, results);

        assert_eq!(report.report_format, REPORT_FORMAT);
        assert_eq!(report.spec_version, SPEC_VERSION);

        let report_text = report.to_string_pretty().expect("report generation failed");
        let time_str = time
            .duration_since(UNIX_EPOCH)
            .expect("time conversion error")
            .as_millis()
            .to_string();

        assert_eq!(
            report_text,
            TEMPLATE_JSON
                .replace("START", &time_str)
                .replace("STOP", &time_str)
        );
    }

    // TODO: serialize full report

    #[test]
    fn deserialize_happy_path() -> Result<()> {
        let time = 1234567890000_u64;
        let time_str = time.to_string();
        let json = TEMPLATE_JSON
            .replace("START", &time_str)
            .replace("STOP", &time_str);

        let report = Report::from_str(&json)?;

        assert_eq!(report.report_format, REPORT_FORMAT);
        assert_eq!(report.spec_version, SPEC_VERSION);

        let time_sys = SystemTime::UNIX_EPOCH + Duration::from_millis(time);
        let results = ResultsBuilder::new(Tool::new(None)).build(time_sys, time_sys);
        let report_exp = Report::new(None, None, None, results);

        assert_eq!(report, report_exp);

        Ok(())
    }

    #[test]
    fn deserialize_bad_format() {
        let time = 1234567890000_u64;
        let time_str = time.to_string();
        let bad_format = "INVALID";
        let json = TEMPLATE_JSON
            .replace("START", &time_str)
            .replace("STOP", &time_str)
            .replace("CTRF", bad_format);

        let report_result = Report::from_str(&json);
        let exp_msg = format!("unrecognized report format '{bad_format}'");

        match report_result {
            Ok(_) => panic!("report deserialization should have failed"),
            Err(e) => {
                if !e.to_string().contains(&exp_msg) {
                    panic!(
                        "deserialization result did not contain expected message \"{}\"",
                        exp_msg
                    );
                }
            }
        }
    }

    // TODO: deserialize full JSON
}
