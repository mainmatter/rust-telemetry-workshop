use std::thread;
use std::time::Duration;

fn do_something(t: Duration) {
    thread::sleep(t);
    // TODO: register how long it takes to run this function using an `invocation_duration_seconds`
    //   histogram.
}

#[cfg(test)]
mod tests {
    use crate::do_something;
    use helpers::init_test_recorder;
    use metrics::Unit;
    use metrics_util::MetricKind;
    use std::time::Duration;

    #[test]
    fn histogram() {
        let snapshotter = init_test_recorder();

        for i in 0..7 {
            do_something(Duration::from_millis(i * 5));
        }

        let metrics = snapshotter.snapshot().into_vec();
        assert_eq!(metrics.len(), 1);
        let (metric_key, unit, description, value) = &metrics[0];
        assert_eq!(metric_key.kind(), MetricKind::Histogram);
        assert_eq!(metric_key.key().name(), "invocation_duration_seconds");
        assert_eq!(unit.unwrap(), Unit::Seconds);
    }
}
