//! Test helpers used in the workshop exercises.
use assert_json_diff::{CompareMode, Config};
use metrics_util::debugging::{DebuggingRecorder, Snapshotter};
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};

/// Assert that the right-hand expression matches the regex specified as first argument.
#[macro_export]
macro_rules! assert_regex {
    ($left:expr, $right:expr $(,)?) => {
        let raw_re = $left;
        let re = ::regex::Regex::new(raw_re).unwrap();
        let actual = $right;
        assert!(
            re.is_match(actual),
            "`{raw_re}` did non match on `{actual}`"
        )
    };
}

/// Use a vector of bytes behind a Arc<Mutex> as writer in order to inspect the tracing output
/// for testing purposes.
#[derive(Clone)]
pub struct MockWriter {
    buf: Arc<Mutex<Vec<u8>>>,
}

impl MockWriter {
    pub fn new() -> Self {
        Self {
            buf: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn map_error<Guard>(err: TryLockError<Guard>) -> std::io::Error {
        match err {
            TryLockError::WouldBlock => std::io::Error::from(std::io::ErrorKind::WouldBlock),
            TryLockError::Poisoned(_) => std::io::Error::from(std::io::ErrorKind::Other),
        }
    }

    pub fn buf(&self) -> std::io::Result<MutexGuard<'_, Vec<u8>>> {
        self.buf.try_lock().map_err(Self::map_error)
    }

    pub fn log_output(&self) -> std::io::Result<LogOutput> {
        let buf = self.buf()?;
        let buf = String::from_utf8(buf.clone())
            .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidData))?;
        Ok(LogOutput(Arc::new(buf)))
    }
}

impl std::io::Write for MockWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf()?.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buf()?.flush()
    }
}

/// A wrapper around the log output of a test.
/// It is designed to ease asserting what the test output should look like, with decent errors
/// when it's not what we expected.
#[derive(Clone)]
pub struct LogOutput(Arc<String>);

impl LogOutput {
    pub fn new(s: String) -> Self {
        Self(Arc::new(s))
    }

    /// Iterator over logging output, line by line.
    pub fn lines(&self) -> LogLines {
        LogLines {
            output: self.clone(),
            lines: self.0.lines(),
        }
    }

    pub fn text(&self) -> &str {
        self.0.as_str()
    }
}

pub struct LogLines<'a> {
    lines: std::str::Lines<'a>,
    output: LogOutput,
}

impl<'a> LogLines<'a> {
    /// Advance the iterator and panic if we've arrived at the end.
    #[track_caller]
    pub fn next_some(&mut self) -> LogLine<'a> {
        let next = self.next();
        if next.is_none() {
            panic!(
                "There is no further log output, although at least another line was expected.\n\
                Full log output:\n{}",
                self.output.0
            );
        }
        next.unwrap()
    }

    /// Panic if we've not arrived at the end of the iterator.
    #[track_caller]
    pub fn end(&mut self) {
        if let Some(next) = self.next() {
            panic!(
                "We expect no further log output, but there is at least one more line:\n{}\n\
                Full log output:\n{}",
                next.line, self.output.0
            );
        }
    }
}

impl<'a> Iterator for LogLines<'a> {
    type Item = LogLine<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|l| LogLine {
            line: l,
            output: self.output.clone(),
        })
    }
}

pub struct LogLine<'a> {
    line: &'a str,
    output: LogOutput,
}

impl<'a> LogLine<'a> {
    pub fn assert_eq(&self, other: &str) {
        assert_eq!(
            self.line, other,
            "A log line didn't match what we expected. Full log output:\n{}",
            self.output.0
        )
    }

    pub fn assert_regex_match(&self, regex: &str) {
        assert_regex!(regex, self.line);
    }

    pub fn assert_json_include(&self, expected: serde_json::Value) {
        let v = match serde_json::Value::from_str(self.line) {
            Ok(v) => v,
            Err(e) => {
                panic!(
                    "Failed to parse log line as JSON: {}\n\n\
                    Log line:\n{}\n\n\
                    Full log output:\n{}",
                    e, self.line, self.output.0
                )
            }
        };
        if let Err(e) = assert_json_diff::assert_json_matches_no_panic(
            &v,
            &expected,
            Config::new(CompareMode::Inclusive),
        ) {
            panic!(
                "The JSON log record doesn't include the expected fields: {}\n\n\
                Log record:\n{}\n\n\
                Full log output:\n{}",
                e, v, self.output.0
            )
        }
    }

    pub fn text(&self) -> &str {
        self.line
    }
}

pub fn init_test_recorder() -> Snapshotter {
    // The `metrics-util` crate provides utilities to easily manipulate metrics
    // in our testing code.
    let recorder = DebuggingRecorder::new();
    let snapshotter = recorder.snapshotter();
    recorder.install().unwrap();
    snapshotter
}