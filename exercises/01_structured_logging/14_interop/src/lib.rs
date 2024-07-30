//! # Exercise
//!
//! We have explicitly disabled the `tracing-log` feature in `tracing-subscriber` in our
//! `Cargo.toml`.  
//! Use the `tracing-log` crate to manually re-route logs emitted via `log` to `tracing`—the
//! process you'll need to go through if you're a difference subscriber for `tracing`.
mod subscriber;

pub use subscriber::init_test_subscriber;
use tracing::instrument;

/// Given a list of order numbers, compute the total price.
#[instrument("parent")]
pub fn do_something() {
    log::info!("Hello from log!");
}

#[cfg(test)]
mod tests {
    use super::init_test_subscriber;
    use crate::do_something;

    #[test]
    fn log2tracing() {
        let logging_buffer = init_test_subscriber();

        do_something();

        // Check that the log output matches what we expect.
        let logging_output = logging_buffer.log_output().unwrap();
        let logging_output = logging_output.text();

        assert!(
            logging_output.contains("Hello from log!"),
            "No `log` events in the overall logging output:\n{}",
            logging_output
        );
    }
}
