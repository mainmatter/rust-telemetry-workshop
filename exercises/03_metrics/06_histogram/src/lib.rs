use std::thread;
use std::time::Duration;

static INVOCATION_DURATION_SECONDS: std::sync::Once = std::sync::Once::new();
const INVOCATION_DURATION_NAME: &str = "invocation_duration_seconds";

fn do_something(t: Duration) {
    INVOCATION_DURATION_SECONDS.call_once(|| {
        metrics::describe_histogram!(
            INVOCATION_DURATION_NAME,
            metrics::Unit::Seconds,
            "How long it took to invoke `do_something`"
        );
    });
    thread::sleep(t);
    metrics::histogram!(INVOCATION_DURATION_NAME).record(t)
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
