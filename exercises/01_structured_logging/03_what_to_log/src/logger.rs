use log::{LevelFilter, Metadata, Record};
use std::fmt::Write;
use std::sync::{Arc, Mutex};

/// A logger implementation that appends the log message to an in-memory buffer.
/// This is useful for testing purposes, as it allows us to assert on the log output.
pub struct TestLogger {
    sink: Arc<Mutex<String>>,
}

impl TestLogger {
    /// Initialize the logger and return a handle to the in-memory logging buffer
    /// that can be used to assert on the log output.
    pub fn init() -> Arc<Mutex<String>> {
        let logging_buffer = Arc::new(Mutex::new(String::new()));
        let logger = Self {
            sink: logging_buffer.clone(),
        };
        log::set_boxed_logger(Box::new(logger)).expect("Failed to set logger");
        log::set_max_level(LevelFilter::Trace);
        logging_buffer
    }
}

impl log::Log for TestLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let mut sink = self.sink.lock().unwrap();
        writeln!(&mut sink, "{}", record.args()).unwrap();
    }

    fn flush(&self) {}
}
