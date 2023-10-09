//! # Labels
//!
//! Not all metric samples are equal.
//! You start tracking the number of requests received by your API. But knowing
//! that number on its own is not enough: your API offers multiple endpoints, where is that
//! traffic actually going?
//!
//! You could solve the issue by using a separate metric for each endpoint.
//! It would work!
//! But it'd make it more difficult to manipulate those metrics in your tool of choice:
//! it's much easier to look for a standard "request_count_total" than
//! a bespoke "request_subscribe_count_total". You might also want to visualize them all
//! in a single dashboard, using a separate colour for each endpoint: having to add each
//! endpoint manually would be tedious.
//!
//! The metric world came up with a solution: when you want to capture a certain dimension,
//! but be able to break it down further, you add **labels**.
//!
//! Labels are a set of key-value pairs that you specify when interacting with a metric.
//! Under the hood, it behaves exactly as having separate metric series: recorders will track each
//! combination of (metric name, [(label keys, label value)]) as its own unique metric.  
//! But we retain the semantic understanding that they are all "subsets" of an overall measurement.
//!
//! # Cardinality
//!
//! Be **extremely** careful with labels!  
//! You should only rely on labels for tracking dimensions that have a well-known and **bounded**
//! cardinality (the set of possible values)—e.g. you don't want to add "user_id" as a label on
//! your metrics!  
//! Each unique label value creates its own metric series, significantly increasing the amount of
//! memory and work necessary to keep track of those metrics in your application.  
//!
//! The number of metric series you are effectively creating/storing/exporting scales as the
//! **product** of the cardinality of each label:
//!
//! # metrics series = { # unique values for label 1 } x .. x { # unique values for label N }
//!
//! That grows fast.
//! If you don't keep an eye on label cardinality you're likely to blow up whatever tool you are
//! exporting metric data to.
//! If it doesn't fail over, you can expect to be punished at the end of the next billing cycle.
//!
//! You've been warned!

/// # Exercise
///
/// Add a `type` label to our `invocations` counter.
/// It should be set to `odd` if `i` is odd, to `even` otherwise.
pub fn do_something(i: u64) {
    let label_value = if i % 2 == 0 { "even" } else { "odd" };
    metrics::increment_counter!("invocations", "type" => label_value);
}

#[cfg(test)]
mod tests {
    use crate::do_something;
    use metrics_util::debugging::{DebugValue, DebuggingRecorder, Snapshotter};

    fn init_test_recorder() {
        DebuggingRecorder::per_thread().install().unwrap();
    }

    #[test]
    fn labels() {
        init_test_recorder();

        for i in 0..7 {
            do_something(i);
        }

        let metrics = Snapshotter::current_thread_snapshot().unwrap().into_vec();
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
