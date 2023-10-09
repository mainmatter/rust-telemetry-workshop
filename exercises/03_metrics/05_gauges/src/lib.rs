//! # Gauges
//!
//! We have seen the end-to-end journey of a metric series. It's time to take a step back
//! and expand our toolset beyond counters.
//!
//! We'll start with gauges.
//! Counters are expected to be positive and monotonically increasing: that's not the case for
//! gauges.
//! Gauges are designed to represent the current value of some property of interest—the number of
//! concurrent requests being handled by your API, the number of idle connections in your
//! database connection pool or the available disk space on your server.
//! The value can increase or decrease over time, therefore counters are ill-suited for these
//! kinds of measurements.
//!
//! The value of a gauge is expected to be a float, rather than an integer.
//! Apart from that, everything else we learned about counters applies to gauges as well
//! (description, units, labels).

pub struct Balance(i64);

const BALANCE_METRIC: &str = "balance";

impl Balance {
    pub fn new() -> Self {
        metrics::describe_gauge!(BALANCE_METRIC, metrics::Unit::Count, "The current balance");
        Self(0)
    }

    pub fn increment(&mut self, by: u32) {
        self.0 += by as i64;
        metrics::gauge!(BALANCE_METRIC, self.0 as f64);
    }

    pub fn decrement(&mut self, by: u32) {
        self.0 -= by as i64;
        metrics::gauge!(BALANCE_METRIC, self.0 as f64);
    }
}

#[cfg(test)]
mod tests {
    use crate::Balance;
    use metrics::Unit;
    use metrics_util::debugging::{DebugValue, DebuggingRecorder, Snapshotter};
    use metrics_util::MetricKind;

    fn init_test_recorder() {
        DebuggingRecorder::per_thread().install().unwrap();
    }

    #[test]
    fn gauges() {
        init_test_recorder();

        let mut balance = Balance::new();
        for i in 0..7 {
            if i % 2 == 0 {
                balance.decrement(i);
            } else {
                balance.increment(i);
            }
        }

        let metrics = Snapshotter::current_thread_snapshot().unwrap().into_vec();
        assert_eq!(metrics.len(), 1);
        let (metric_key, unit, description, value) = &metrics[0];
        assert_eq!(metric_key.kind(), MetricKind::Gauge);
        let DebugValue::Gauge(value) = value else {
            unreachable!()
        };
        assert_eq!(value.into_inner(), -3.0);

        assert_eq!(metric_key.key().name(), "balance");
        // Unfortunately you can't register custom units in `metrics`. No euros for us here!
        assert_eq!(unit.unwrap(), Unit::Count);
        assert_eq!(
            description.as_ref().unwrap().as_ref(),
            "The current balance"
        )
    }
}
