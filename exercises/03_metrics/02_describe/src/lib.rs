//! # Describe
//!
//! Let's break down what happened in the last exercise.
//! You created a counter: a **monotonically increasing** value.  
//! Once it has been created, its value can only be incremented.
//!
//! You can create a counter using the `counter!` macro.
//! You can then call `increment` on the `Counter` returned by the macro to increment its value.
//!
//! `Counter` exposes another method, `absolute`, which is a bit special: it's primarily designed to **initialize**
//! a counter sequence, specifying a non-zero initial value.
//! Beware: recorders will enforce the monotonicity property for counters, so you can't rely on `absolute`
//! to artificially decrement an existing counter.
//!
//! When using `counter!`, the `Recorder` implementation is expected to:
//!
//! - Create a new counter with the name you specified, if one doesn't exist;
//! - Retrieve the counter, if one exists.
//!
//! `UPSERT` behaviour, for the SQL-minded among us.
//!
//! # Metadata
//!
//! It can be useful to add metadata to your counters (and metrics in general).  
//! You can rely on the `describe_*` macros offered by the `metrics!` crate: they let you add
//! a unit (e.g. capture that the counter value represents bytes or seconds) and a description
//! (which recorders can then choose to expose when metrics are exported/analyzes).

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
