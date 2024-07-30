/// There is no need to describe the same metric series multiple times.
///
/// You can either perform this task in some kind of "initialization" phase for your application,
/// or rely on something like `Once`.
/// Deduping "manually" is usually cheaper than delegating the dedup work to the recorder, since it
/// might have to perform more processing before realising that there is nothing to be done (e.g.
/// manipulate the metric name, acquire a lock over the entire metric store, etc.).
static REGISTER_INVOCATIONS: std::sync::Once = std::sync::Once::new();

static COUNTER_NAME: &str = "invocations";

pub fn do_something() {
    REGISTER_INVOCATIONS.call_once(|| {
        // TODO: Set `Count` as the unit for "invocations" and "The number of times `do something`
        //   has been invoked" as its description.
        todo!()
    });
    metrics::counter!(COUNTER_NAME).increment(1);
}

#[cfg(test)]
mod tests {
    use crate::{do_something, COUNTER_NAME};
    use helpers::init_test_recorder;
    use metrics::Unit;
    use metrics_util::MetricKind;

    #[test]
    fn describe() {
        let snapshotter = init_test_recorder();

        for _ in 0..7 {
            do_something();
        }

        let metrics = snapshotter.snapshot().into_vec();
        assert_eq!(metrics.len(), 1);
        let (metric_key, unit, description, _) = &metrics[0];

        assert_eq!(metric_key.kind(), MetricKind::Counter);
        assert_eq!(metric_key.key().name(), COUNTER_NAME);
        assert_eq!(unit.unwrap(), Unit::Count);
        assert_eq!(
            description.as_ref().unwrap().as_ref(),
            "The number of times `do something` has been invoked"
        )
    }
}
