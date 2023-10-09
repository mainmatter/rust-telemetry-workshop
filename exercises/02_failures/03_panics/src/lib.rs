//! # Panic handling
//!
//! There are two types of failures in Rust:
//! - recoverable errors, using the `Result` type and the `Error` trait
//! - panics, (almost) unrecoverable failures that unwind the stack
//!
//! It's time to talk about the latter!
//!
//! ## Panics
//!
//! You should trigger a panic when:
//!
//! - you encounter a situation that you can't recover from
//! - it would be unsafe to continue
//! - it's not worth the effort to perform any kind of more sophisticated error handling.
//!
//! A few examples:
//!
//! - you're missing a resource that you absolutely need to do your job
//! - an internal invariant of your application has been violated
//! - you're in testing code and you don't want to continue if a specific condition is not met
//!
//! ## Panics and telemetry
//!
//! Regardless of the source, you should build a telemetry pipeline that knows how to handle
//! panics.  
//!
//! Whenever a panic is triggered, Rust will start unwinding the stack and will eventually
//! invoke the `panic` hook for your application.  
//! The default hook will print the panic message to `stderr` and then abort the process. This
//! may be good enough if you're running your application in a terminal, but it's not going to be
//! very useful if you're running in a production environment: you want the panic information
//! to be captured by the same telemetry pipeline that you're using for everything else.
//!
//! We need a custom panic hook!
//!
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
