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
