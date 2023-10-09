//! # The lifecycle of a span
//!
//! All the instrumentation work we have done so far has happened in a
//! **synchronous** context.  
//! We have only used async Rust on the processing side, to use non-blocking IO to ship telemetry
//! data to a sink over the network (i.e. Honeycomb's API).  
//!
//! But what if we wanted to **instrument asynchronous code**?  
//!
//! It's trickier!  
//! There are some subtleties to be aware of, at the intersection between the lifecycle of a
//! `tracing` span, work-stealing async runtimes and `tracing`'s internal state management.
//!
//! It's a lot to cover in a single exercise, so we'll break it down into smaller chunks.  
//! Let's start with the lifecycle of a span.
//!
//! # A span was born—what then?
//!
//! You've seen in the first `tracing` exercises that there are two distinct phases in the lifecycle
//! of a span:
//!
//! - **creation**, i.e. what happens when `tracing::<level>_span!` is called;
//! - **entering**, i.e. what happens when you can `.enter()` on a span, getting back a RAII guard.
//!
//! A span can only be created **once**, but it can be entered **multiple times**.
//!
//! ```rust
//! use tracing::info_span;
//!
//! let span = info_span!("my span");
//! {
//!     let _guard = span.enter();
//!     // Do something
//! }
//! {
//!     // We re-enter the same span!
//!     let _guard2 = span.enter();
//!     // Do something else
//! }
//! ```
//!
//! Why would you do that, you may be wondering?  
//! Well, your span may be tracking a piece of work that can be **paused** (async, remember?).
//!
//! Creation and entering have two matching phases: **closing** and **exiting**.  
//!
//! A span is **exited** when the guard object returned by `.enter()` is dropped.  
//! A span is **closed**, instead, when there are no more handles to it. It's the subscriber's
//! responsibility to do this bookkeeping.
//!
//! When a span is entered into a single time, the closing and exiting phases usually* happen one
//! after the other. This is what happened so far in our exercises.
//!
//! * You can technically clone the span handle and hold on to it, although I would wonder why.
mod subscriber;

pub use subscriber::init_test_subscriber;

/// # Exercise
///
/// Let's see how this works in practice.
///
/// Manipulate the span we create in this function to match the output in the test below.
pub fn do_something() {
    let span = tracing::info_span!("My unit of work");
    for i in 0..3 {
        let _guard = span.enter();
    }
}

#[cfg(test)]
mod tests {
    use super::init_test_subscriber;
    use serde_json::json;

    #[test]
    fn failure() {
        let logging_buffer = init_test_subscriber();

        super::do_something();

        // Check that the log output matches what we expect.
        let logging_output = logging_buffer.log_output().unwrap();
        let mut log_lines = logging_output.lines();

        log_lines
            .next_some()
            .assert_json_include(json!({"message":"new"}));

        for _ in 0..3 {
            log_lines
                .next_some()
                .assert_json_include(json!({"message":"enter"}));
            log_lines
                .next_some()
                .assert_json_include(json!({"message":"exit"}));
        }

        log_lines
            .next_some()
            .assert_json_include(json!({"message":"close"}));

        log_lines.end();
    }
}
