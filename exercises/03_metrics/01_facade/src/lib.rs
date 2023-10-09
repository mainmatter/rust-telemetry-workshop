//! # Exercise
//!
//! Let's dip out toes into the world of `metrics` with a simple counter: we will track how
//! many times a certain function is invoked.
//!
//! Tip: take a look at the code in charge of installing a `Recorder` to get acquainted with the
//! underlying machinery.

pub fn do_something() {
    // TODO: increment a counter named "invocations" here
    metrics::counter!("invocations").increment(1);
}

#[cfg(test)]
mod tests {
    use crate::do_something;
    use metrics_util::debugging::{DebugValue, DebuggingRecorder, Snapshotter};
    use metrics_util::MetricKind;

    fn init_test_recorder() -> Snapshotter {
        // The `metrics-util` crate provides utilities to easily manipulate metrics
        // in our testing code.
        let recorder = DebuggingRecorder::new();
        let snapshotter = recorder.snapshotter();
        recorder.install().unwrap();
        snapshotter
    }

    #[test]
    fn increments() {
        let snapshotter = init_test_recorder();
        let n_invocations = 7;

        for _ in 0..n_invocations {
            do_something();
        }

        // We can get a handle to a "snapshot", the set of metrics
        // that have been registered and recorded against the test recorder.
        let metrics = snapshotter.snapshot().into_vec();
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
