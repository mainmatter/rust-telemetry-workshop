//! # Interoperability
//!
//! You have seen first-hand how much `tracing` brings to the table.
//! Nonetheless, migrating an entire ecosystem takes time: you can still find crates that rely
//! on `log` for instrumentation.
//!
//! We need a way to bridge the gap between the two worlds: you should be able to use a single
//! pipeline to process telemetry coming from both sources.
//!
//! # `tracing-log`
//!
//! The `tracing-log` crate can act as such a bridge: it provides you with a `log`-compatible
//! processor which redirects all `log` events to `tracing`.  
//!
//! Be mindful: if you're using `tracing_subscriber::fmt` as your `tracing` subscriber, it'll
//! automatically install this bridge for you unless you explicitly disable the `tracing-log`
//! feature.
//!
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
