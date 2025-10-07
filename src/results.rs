use crate::{
    environment::Environment,
    extra::Extra,
    impl_extra,
    summary::Summary,
    test::{Status, Test},
    tool::Tool,
};

use std::{
    collections::{HashMap, HashSet},
    time::SystemTime,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Results {
    tool: Tool,
    summary: Summary,
    tests: Vec<Test>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<Environment>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    extra: HashMap<String, Value>,
}

pub struct ResultsBuilder {
    tool: Tool,
    tests: Vec<Test>,
    environment: Option<Environment>,
    extra: HashMap<String, Value>,
}

impl ResultsBuilder {
    pub fn new(tool: Tool) -> Self {
        Self {
            tool,
            tests: vec![],
            environment: None,
            extra: HashMap::new(),
        }
    }

    /// Appends a Test to the contained list
    pub fn add_test(&mut self, test: Test) {
        self.tests.push(test);
    }

    /// Sets the Environment, can be None
    pub fn environment(mut self, environment: Option<Environment>) {
        self.environment = environment;
    }

    /// Builds and returns the final Results instance
    pub fn build(self, start: SystemTime, stop: SystemTime) -> Results {
        let ResultsBuilder {
            tool,
            tests,
            environment,
            extra,
        } = self;

        let mut summary = Summary::new(start, stop);

        summary.passed(
            tests
                .iter()
                .filter(|t| t.status() == Status::Passed)
                .count(),
        );
        summary.failed(
            tests
                .iter()
                .filter(|t| t.status() == Status::Failed)
                .count(),
        );
        summary.pending(
            tests
                .iter()
                .filter(|t| t.status() == Status::Pending)
                .count(),
        );
        summary.skipped(
            tests
                .iter()
                .filter(|t| t.status() == Status::Skipped)
                .count(),
        );
        summary.other(tests.iter().filter(|t| t.status() == Status::Other).count());

        let mut suites = HashSet::new();
        for t in &tests {
            if let Some(s) = t.suite() {
                suites.insert(s);
            }
        }

        let suite_count = suites.len();
        if suite_count > 0 {
            summary.suites(Some(suite_count));
        }

        Results {
            tool,
            summary,
            tests,
            environment,
            extra,
        }
    }
}

impl_extra!(Results, ResultsBuilder);

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tool::TOOL_NAME;

    use std::time::Duration;

    use serde_json::Result;

    #[test]
    fn add_passed() -> Result<()> {
        const TEST_COUNT: usize = 2;

        let tool = Tool::new("ctrf-rs", None);
        let mut builder = ResultsBuilder::new(tool);

        for t in 0..TEST_COUNT {
            builder.add_test(Test::new(
                format!("pass{t}"),
                Status::Passed,
                Duration::from_millis(0),
            ));
        }

        let time = SystemTime::now();
        let results = builder.build(time, time);

        let tool_text = serde_json::to_string::<Tool>(&results.tool)?;
        let summary_text = serde_json::to_string::<Summary>(&results.summary)?;
        let tests_text = serde_json::to_string::<Vec<Test>>(&results.tests)?;
        assert!(tool_text.contains(TOOL_NAME));
        assert!(summary_text.contains(&format!(r#""tests":{TEST_COUNT}"#)));
        assert!(summary_text.contains(&format!(r#""passed":{TEST_COUNT}"#)));
        for t in 0..TEST_COUNT {
            assert!(tests_text.contains(&format!(r#""name":"pass{t}""#)));
        }

        Ok(())
    }

    #[test]
    fn add_failed() -> Result<()> {
        const TEST_COUNT: usize = 4;

        let tool = Tool::new("ctrf-rs", None);
        let mut builder = ResultsBuilder::new(tool);

        for t in 0..TEST_COUNT {
            builder.add_test(Test::new(
                format!("fail{t}"),
                Status::Failed,
                Duration::from_millis(0),
            ));
        }

        let time = SystemTime::now();
        let results = builder.build(time, time);

        let tool_text = serde_json::to_string::<Tool>(&results.tool)?;
        let summary_text = serde_json::to_string::<Summary>(&results.summary)?;
        let tests_text = serde_json::to_string::<Vec<Test>>(&results.tests)?;
        assert!(tool_text.contains(TOOL_NAME));
        assert!(summary_text.contains(&format!(r#""tests":{TEST_COUNT}"#)));
        assert!(summary_text.contains(&format!(r#""failed":{TEST_COUNT}"#)));
        for t in 0..TEST_COUNT {
            assert!(tests_text.contains(&format!(r#""name":"fail{t}""#)));
        }

        Ok(())
    }

    #[test]
    fn add_pending() -> Result<()> {
        const TEST_COUNT: usize = 6;

        let tool = Tool::new("ctrf-rs", None);
        let mut builder = ResultsBuilder::new(tool);

        for t in 0..TEST_COUNT {
            builder.add_test(Test::new(
                format!("pending{t}"),
                Status::Pending,
                Duration::from_millis(0),
            ));
        }

        let time = SystemTime::now();
        let results = builder.build(time, time);

        let tool_text = serde_json::to_string::<Tool>(&results.tool)?;
        let summary_text = serde_json::to_string::<Summary>(&results.summary)?;
        let tests_text = serde_json::to_string::<Vec<Test>>(&results.tests)?;
        assert!(tool_text.contains(TOOL_NAME));
        assert!(summary_text.contains(&format!(r#""tests":{TEST_COUNT}"#)));
        assert!(summary_text.contains(&format!(r#""pending":{TEST_COUNT}"#)));
        for t in 0..TEST_COUNT {
            assert!(tests_text.contains(&format!(r#""name":"pending{t}""#)));
        }

        Ok(())
    }

    #[test]
    fn add_skipped() -> Result<()> {
        const TEST_COUNT: usize = 8;

        let tool = Tool::new("ctrf-rs", None);
        let mut builder = ResultsBuilder::new(tool);

        for t in 0..TEST_COUNT {
            builder.add_test(Test::new(
                format!("skipped{t}"),
                Status::Skipped,
                Duration::from_millis(0),
            ));
        }

        let time = SystemTime::now();
        let results = builder.build(time, time);

        let tool_text = serde_json::to_string::<Tool>(&results.tool)?;
        let summary_text = serde_json::to_string::<Summary>(&results.summary)?;
        let tests_text = serde_json::to_string::<Vec<Test>>(&results.tests)?;
        assert!(tool_text.contains(TOOL_NAME));
        assert!(summary_text.contains(&format!(r#""tests":{TEST_COUNT}"#)));
        assert!(summary_text.contains(&format!(r#""skipped":{TEST_COUNT}"#)));
        for t in 0..TEST_COUNT {
            assert!(tests_text.contains(&format!(r#""name":"skipped{t}""#)));
        }

        Ok(())
    }

    #[test]
    fn add_other() -> Result<()> {
        const TEST_COUNT: usize = 10;

        let tool = Tool::new("ctrf-rs", None);
        let mut builder = ResultsBuilder::new(tool);

        for t in 0..TEST_COUNT {
            builder.add_test(Test::new(
                format!("other{t}"),
                Status::Other,
                Duration::from_millis(0),
            ));
        }

        let time = SystemTime::now();
        let results = builder.build(time, time);

        let tool_text = serde_json::to_string::<Tool>(&results.tool)?;
        let summary_text = serde_json::to_string::<Summary>(&results.summary)?;
        let tests_text = serde_json::to_string::<Vec<Test>>(&results.tests)?;
        assert!(tool_text.contains(TOOL_NAME));
        assert!(summary_text.contains(&format!(r#""tests":{TEST_COUNT}"#)));
        assert!(summary_text.contains(&format!(r#""other":{TEST_COUNT}"#)));
        for t in 0..TEST_COUNT {
            assert!(tests_text.contains(&format!(r#""name":"other{t}""#)));
        }

        Ok(())
    }

    #[test]
    fn add_many() -> Result<()> {
        const PRESENT_SUITE: &str = "present";
        const ABSENT_SUITE: &str = "absent";
        const UNKNOWN_SUITE: &str = "unknown";

        let tool = Tool::new("ctrf-rs", None);
        let mut builder = ResultsBuilder::new(tool);

        const PASS_COUNT: usize = 10;
        for t in 0..PASS_COUNT {
            let mut test = Test::new(format!("pass{t}"), Status::Passed, Duration::from_millis(0));
            test.suite = Some(String::from(PRESENT_SUITE));
            builder.add_test(test);
        }

        const FAIL_COUNT: usize = 8;
        for t in 0..FAIL_COUNT {
            let mut test = Test::new(format!("fail{t}"), Status::Failed, Duration::from_millis(0));
            test.suite = Some(String::from(PRESENT_SUITE));
            builder.add_test(test);
        }

        const PENDING_COUNT: usize = 6;
        for t in 0..PENDING_COUNT {
            let mut test = Test::new(
                format!("pending{t}"),
                Status::Pending,
                Duration::from_millis(0),
            );
            test.suite = Some(String::from(ABSENT_SUITE));
            builder.add_test(test);
        }

        const SKIPPED_COUNT: usize = 4;
        for t in 0..SKIPPED_COUNT {
            let mut test = Test::new(
                format!("skipped{t}"),
                Status::Skipped,
                Duration::from_millis(0),
            );
            test.suite = Some(String::from(ABSENT_SUITE));
            builder.add_test(test);
        }

        const OTHER_COUNT: usize = 2;
        for t in 0..OTHER_COUNT {
            let mut test = Test::new(format!("other{t}"), Status::Other, Duration::from_millis(0));
            test.suite = Some(String::from(UNKNOWN_SUITE));
            builder.add_test(test);
        }

        const TOTAL_COUNT: usize =
            PASS_COUNT + FAIL_COUNT + PENDING_COUNT + SKIPPED_COUNT + OTHER_COUNT;

        let time = SystemTime::now();
        let results = builder.build(time, time);

        let tool_text = serde_json::to_string::<Tool>(&results.tool)?;
        let summary_text = serde_json::to_string::<Summary>(&results.summary)?;
        let tests_text = serde_json::to_string::<Vec<Test>>(&results.tests)?;
        assert!(tool_text.contains(TOOL_NAME));

        assert!(summary_text.contains(&format!(r#""tests":{TOTAL_COUNT}"#)));
        assert!(summary_text.contains(&format!(r#""passed":{PASS_COUNT}"#)));
        assert!(summary_text.contains(&format!(r#""failed":{FAIL_COUNT}"#)));
        assert!(summary_text.contains(&format!(r#""pending":{PENDING_COUNT}"#)));
        assert!(summary_text.contains(&format!(r#""skipped":{SKIPPED_COUNT}"#)));
        assert!(summary_text.contains(&format!(r#""other":{OTHER_COUNT}"#)));
        assert!(summary_text.contains(&format!(r#""suites":3"#)));

        for t in 0..PASS_COUNT {
            assert!(tests_text.contains(&format!(r#""name":"pass{t}""#)));
        }
        for t in 0..FAIL_COUNT {
            assert!(tests_text.contains(&format!(r#""name":"fail{t}""#)));
        }
        for t in 0..PENDING_COUNT {
            assert!(tests_text.contains(&format!(r#""name":"pending{t}""#)));
        }
        for t in 0..SKIPPED_COUNT {
            assert!(tests_text.contains(&format!(r#""name":"skipped{t}""#)));
        }
        for t in 0..OTHER_COUNT {
            assert!(tests_text.contains(&format!(r#""name":"other{t}""#)));
        }

        Ok(())
    }
}
