//! # The `metrics` crate
//!
//! How does the `metrics` crate work?
//! `log` used the facade pattern, `tracing` used the facade pattern... You guessed it, `metrics`
//! uses the facade pattern as well!
//!
//! It exposes a set of macros to create and manipulate metrics.
//! The metric data is then forwarded to an implementor of the `metrics::Recorder` trait,
//! which takes care of processing it.
//!
//! It should all feel pretty familiar by now!
//!
//! # Exercise
//!
//! Let's dip out toes into the world of `metrics` with a simple counter: we will track how
//! many times a certain function is invoked.
//!
//! Tip: take a look at the code in charge of installing a `Recorder` to get acquainted with the
//! underlying machinery.

pub fn do_something() {
    // TODO: increment a counter named "invocations" here
    metrics::increment_counter!("invocations");
}

#[cfg(test)]
mod tests {
    use crate::do_something;
    use metrics_util::debugging::{DebugValue, DebuggingRecorder, Snapshotter};
    use metrics_util::MetricKind;

    fn init_test_recorder() {
        // The `metrics-util` crate provides utilities to easily manipulate metrics
        // in our testing code.
        // The `DebuggingRecorder` lets us attach a recorder to a single thread rather than a
        // "global" one, which allows us not to have to split out tests across multiple test
        // binaries to achieve isolation (what we did for all our `tracing` and `log` tests).
        DebuggingRecorder::per_thread().install().unwrap();
    }

    #[test]
    fn increments() {
        init_test_recorder();
        let n_invocations = 7;

        for _ in 0..n_invocations {
            do_something();
        }

        // We can get a handle to a "snapshot", the set of metrics
        // that have been registered and recorded against the per-thread debugging recorder.
        let metrics = Snapshotter::current_thread_snapshot().unwrap().into_vec();
        assert_eq!(metrics.len(), 1);
        let (metric_key, _, _, metric_value) = &metrics[0];

        assert_eq!(metric_key.kind(), MetricKind::Counter);
        assert_eq!(metric_key.key().name(), "invocations");
        let DebugValue::Counter(metric_value) = metric_value else {
            unreachable!()
        };
        assert_eq!(*metric_value, n_invocations);
    }
}
