use crate::{extra::Extra, impl_extra};

use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result summary element for a CTRF report.
/// Corresponds to the spec's ["Summary"](https://ctrf.io/docs/specification/summary) object.
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    tests: usize,
    passed: usize,
    failed: usize,
    pending: usize,
    skipped: usize,
    other: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    suites: Option<usize>,
    start: u64,
    stop: u64,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    extra: HashMap<String, Value>,
}

impl Summary {
    /// Creates a report Summary instance
    pub fn new(start: SystemTime, stop: SystemTime) -> Self {
        Self {
            tests: 0,
            passed: 0,
            failed: 0,
            pending: 0,
            skipped: 0,
            other: 0,
            suites: None,
            start: start.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
            stop: stop.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
            extra: HashMap::new(),
        }
    }

    pub fn all_passed(&self) -> bool {
        self.passed == self.tests
    }

    /// Sets the count of passed tests and updates the overall total
    pub fn passed(&mut self, count: usize) {
        self.passed = count;

        self.update_tests();
    }

    /// Sets the count of failed tests and updates the overall total
    pub fn failed(&mut self, count: usize) {
        self.failed = count;

        self.update_tests();
    }

    /// Sets the count of pending tests and updates the overall total
    pub fn pending(&mut self, count: usize) {
        self.pending = count;

        self.update_tests();
    }

    /// Sets the count of skipped tests and updates the overall total
    pub fn skipped(&mut self, count: usize) {
        self.skipped = count;

        self.update_tests();
    }

    /// Sets the count of other tests and updates the overall total
    pub fn other(&mut self, count: usize) {
        self.other = count;

        self.update_tests();
    }

    /// Sets the number of Suites, can be None
    pub fn suites(&mut self, suites: Option<usize>) {
        self.suites = suites;
    }

    /// Updates the total test count
    fn update_tests(&mut self) {
        self.tests = self.passed + self.failed + self.pending + self.skipped + self.other;
    }
}

impl_extra!(Summary);

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::SystemTime;

    #[test]
    fn add_passed() {
        const PASSED_COUNT: usize = 5;
        let time = SystemTime::now();
        let mut summary = Summary::new(time, time);

        summary.passed(PASSED_COUNT);

        assert_eq!(summary.passed, PASSED_COUNT);
        assert_eq!(summary.tests, PASSED_COUNT);
    }

    #[test]
    fn add_failed() {
        const FAILED_COUNT: usize = 20;
        let time = SystemTime::now();
        let mut summary = Summary::new(time, time);

        summary.passed(FAILED_COUNT);

        assert_eq!(summary.passed, FAILED_COUNT);
        assert_eq!(summary.tests, FAILED_COUNT);
    }

    #[test]
    fn add_pending() {
        const PENDING_COUNT: usize = 10;
        let time = SystemTime::now();
        let mut summary = Summary::new(time, time);

        summary.passed(PENDING_COUNT);

        assert_eq!(summary.passed, PENDING_COUNT);
        assert_eq!(summary.tests, PENDING_COUNT);
    }

    #[test]
    fn add_skipped() {
        const SKIPPED_COUNT: usize = 2;
        let time = SystemTime::now();
        let mut summary = Summary::new(time, time);

        summary.passed(SKIPPED_COUNT);

        assert_eq!(summary.passed, SKIPPED_COUNT);
        assert_eq!(summary.tests, SKIPPED_COUNT);
    }

    #[test]
    fn add_other() {
        const OTHER_COUNT: usize = 50;
        let time = SystemTime::now();
        let mut summary = Summary::new(time, time);

        summary.passed(OTHER_COUNT);

        assert_eq!(summary.passed, OTHER_COUNT);
        assert_eq!(summary.tests, OTHER_COUNT);
    }

    #[test]
    fn add_all_types() {
        const PASSED_COUNT: usize = 5;
        const FAILED_COUNT: usize = 40;
        const PENDING_COUNT: usize = 300;
        const SKIPPED_COUNT: usize = 2000;
        const OTHER_COUNT: usize = 10000;
        let time = SystemTime::now();
        let mut summary = Summary::new(time, time);

        summary.passed(PASSED_COUNT);
        summary.failed(FAILED_COUNT);
        summary.skipped(SKIPPED_COUNT);
        summary.pending(PENDING_COUNT);
        summary.other(OTHER_COUNT);

        assert_eq!(summary.passed, PASSED_COUNT);
        assert_eq!(summary.failed, FAILED_COUNT);
        assert_eq!(summary.skipped, SKIPPED_COUNT);
        assert_eq!(summary.pending, PENDING_COUNT);
        assert_eq!(summary.other, OTHER_COUNT);
        assert_eq!(
            summary.tests,
            PASSED_COUNT + FAILED_COUNT + PENDING_COUNT + SKIPPED_COUNT + OTHER_COUNT,
        );
    }

    #[test]
    fn revise_value() {
        const PASSED_COUNT: usize = 5;
        const FAILED_COUNT: usize = 40;
        const PENDING_COUNT: usize = 300;
        const SKIPPED_COUNT: usize = 2000;
        const OTHER_COUNT: usize = 10000;
        let time = SystemTime::now();
        let mut summary = Summary::new(time, time);

        summary.passed(PASSED_COUNT);
        summary.failed(FAILED_COUNT);
        summary.skipped(SKIPPED_COUNT);
        summary.pending(PENDING_COUNT);
        summary.other(OTHER_COUNT);

        assert_eq!(summary.passed, PASSED_COUNT);
        assert_eq!(summary.failed, FAILED_COUNT);
        assert_eq!(summary.skipped, SKIPPED_COUNT);
        assert_eq!(summary.pending, PENDING_COUNT);
        assert_eq!(summary.other, OTHER_COUNT);
        assert_eq!(
            summary.tests,
            PASSED_COUNT + FAILED_COUNT + PENDING_COUNT + SKIPPED_COUNT + OTHER_COUNT,
        );

        const NEW_PASSED: usize = 24681;
        summary.passed(NEW_PASSED);

        assert_eq!(summary.passed, NEW_PASSED);
        assert_eq!(summary.failed, FAILED_COUNT);
        assert_eq!(summary.skipped, SKIPPED_COUNT);
        assert_eq!(summary.pending, PENDING_COUNT);
        assert_eq!(summary.other, OTHER_COUNT);
        assert_eq!(
            summary.tests,
            NEW_PASSED + FAILED_COUNT + PENDING_COUNT + SKIPPED_COUNT + OTHER_COUNT,
        );
    }

    #[test]
    fn add_suites() {
        const SUITES: Option<usize> = Some(16);
        let time = SystemTime::now();
        let mut summary = Summary::new(time, time);

        summary.suites(SUITES);

        assert_eq!(summary.suites, SUITES)
    }
}
