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

impl Balance {
    pub fn new() -> Self {
        // TODO: register a `balance` metric
        Self(0)
    }

    pub fn increment(&mut self, by: u32) {
        self.0 += by as i64;
        // TODO: record the new balance value
    }

    pub fn decrement(&mut self, by: u32) {
        self.0 -= by as i64;
        // TODO: record the new balance value
    }
}

#[cfg(test)]
mod tests {
    use crate::Balance;
    use helpers::init_test_recorder;
    use metrics::Unit;
    use metrics_util::debugging::DebugValue;
    use metrics_util::MetricKind;

    #[test]
    fn gauges() {
        let snapshotter = init_test_recorder();

        let mut balance = Balance::new();
        for i in 0..7 {
            if i % 2 == 0 {
                balance.decrement(i);
            } else {
                balance.increment(i);
            }
        }

        let metrics = snapshotter.snapshot().into_vec();
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
