mod subscriber;

pub use subscriber::init_test_subscriber;
use tokio::task::yield_now;
use tracing::Span;

/// # Exercise
///
/// In the test below, attach a span to the invocations of `do_something` so that the output
/// matches the expected one.
pub async fn do_something(id: u16) {
    // We give a chance to the runtime to pause this future.
    // `.await` points are where the runtime gets back into the driver's seat
    // when it comes to async functions.
    yield_now().await;
    Span::current().record("caller_id", id);
    yield_now().await;
}

#[cfg(test)]
mod tests {
    use super::init_test_subscriber;
    use crate::do_something;

    #[tokio::test]
    /// We spawn a bunch of futures and check that we don't have any cross-task interference
    /// when it comes to our spans (i.e. a future setting the value of a field in a span
    /// that belongs to a different future).
    async fn futures() {
        let logging_buffer = init_test_subscriber();

        let n_futures = 10;

        let mut join_set = tokio::task::JoinSet::new();
        for i in 0..n_futures {
            let future = do_something(i);
            let span = tracing::info_span!("Task", caller_id = tracing::field::Empty);
            // TODO: attach the span to the future!
            join_set.spawn(future);
        }
        // Let's wait for all tasks to complete.
        while let Some(_) = join_set.join_next().await {}

        // Check that the log output matches what we expect.
        let logging_output = logging_buffer.log_output().unwrap();
        let logging_output = logging_output.text();

        for i in 0..n_futures {
            assert!(
                logging_output.contains(&format!("caller_id={i}")),
                "No log for caller id {} in the overall logging output:\n{}",
                i,
                logging_output
            );
        }
    }
}
