/// # Exercise
///
/// Add a `type` label to our `invocations` counter.
/// It should be set to `odd` if `i` is odd, to `even` otherwise.
pub fn do_something(i: u64) {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::do_something;
    use helpers::init_test_recorder;
    use metrics_util::debugging::DebugValue;

    #[test]
    fn labels() {
        let snapshotter = init_test_recorder();

        for i in 0..7 {
            do_something(i);
        }

        let metrics = snapshotter.snapshot().into_vec();
        assert_eq!(metrics.len(), 2);

        for (key, _, _, value) in metrics {
            let key = key.key();
            let DebugValue::Counter(value) = value else {
                unreachable!()
            };
            let labels: Vec<_> = key.labels().collect();
            assert_eq!(labels.len(), 1);
            let label = labels[0];
            assert_eq!(label.key(), "type");
            if label.value() == "odd" {
                assert_eq!(value, 3);
            } else if label.value() == "even" {
                assert_eq!(value, 4);
            } else {
                unreachable!()
            }
        }
    }
}
