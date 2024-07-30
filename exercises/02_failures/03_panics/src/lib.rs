//! # Exercise
//!
//! Install a custom panic hook that will emit a `tracing` event whenever a panic is triggered.

mod subscriber;

fn panicky_function() {
    panic!("I'm panicking! Log me!");
}

#[cfg(test)]
mod tests {
    use crate::panicky_function;
    use crate::subscriber::init_test_subscriber;
    use std::panic::catch_unwind;

    #[test]
    fn error_span() {
        let logging_buffer = init_test_subscriber();

        // We catch the panic so that it doesn't abort the test.
        let _ = catch_unwind(panicky_function);

        // Check that the log output matches what we expect.
        let logging_output = logging_buffer.log_output().unwrap();
        let logging_output = logging_output.text();

        assert!(
            logging_output.contains("I'm panicking! Log me!"),
            "The logging output is missing the expected panic message:\n{}",
            logging_output
        );
    }
}
