//! # Thread local state
//!
//! How does `tracing` know which span is currently active?
//!
//! That's necessary to implement a few of the features we've seen so far:
//!
//! - `Span::current()`, which returns a handle over that span that is currently active
//! - Attaching a new span as a child of the currently active span
//!
//! The answer is **thread local state**.
//!
//! ## `thread_local!`
//!
//! Rust's standard library exposes a `thread_local!` macro that allows you to define a variable
//! that is local to the current thread.  
//! It's a bit like `static`, but it's not shared across threads.
//!
//! It comes with a caveat: if you move to a different thread, you won't be able to access the
//! value you set on the previous thread.
//!
//! ## Spawning threads breaks the hierarchy
//!
//! `tracing` uses thread local state to keep track of the currently active span.  
//! This has an interesting implication: if you spawn a thread to do some work, the spans
//! created in that thread will **not** be linked to the spans created in the parent thread.
//!
//! ```rust
//! use tracing::{info_span, info};
//!
//! let spawner_span = info_span!("spawner");
//! let _guard = spawner_span.enter();
//!
//! std::thread::spawn(|| {
//!     // This is NOT a child of `spawner_span`!
//!     let spawned_span = info_span!("spawned");
//!     // [...]
//! });
//! ```
//!
//! This is something to be aware of when you're using `tracing` in a multi-threaded environment.
//! You have three options:
//!
//! - Leave the spans unlinked. This is OK if the two unit of works are actually unrelated.
//! - Explicitly mark the `spawned` span as a child of the `spawner` span. This is desirable
//!   if the `spawner` span won't be closed until the `spawned` span is closed (e.g. if you are
//!   waiting for the new thread to finish).
//! - Explicitly mark the `spawned` span as a "follower" of the `spawner` span. This maintains a
//!   connection between the two spans, but it doesn't require the `spawner` span to be kept open.
//!   This works pretty well when spawning background tasks that might complete after the `spawner`
//!   unit of work is done.
mod subscriber;

pub use subscriber::init_test_subscriber;

/// # Exercise
///
/// Let's see how this works in practice.
///
/// Manipulate the spans we create in this function to match the output in the test below.
pub fn do_something() -> std::thread::JoinHandle<()> {
    let spawner_span = tracing::info_span!("spawner");
    let _guard = spawner_span.enter();

    let parent = spawner_span.clone();
    let handle = std::thread::spawn(move || {
        let spawned_span = tracing::info_span!(parent: parent, "spawned1");
        let _guard = spawned_span.enter();
    });

    handle.join().unwrap();

    let follows_from = spawner_span.clone();
    std::thread::spawn(move || {
        let spawned_span = tracing::info_span!("spawned2");
        spawned_span.follows_from(&follows_from);
        let _guard = spawned_span.enter();
    })
}

#[cfg(test)]
mod tests {
    use super::init_test_subscriber;

    #[test]
    fn linking() {
        let logging_buffer = init_test_subscriber();

        let handle = super::do_something();
        handle.join().unwrap();

        // Check that the log output matches what we expect.
        let logging_output = logging_buffer.log_output().unwrap();
        let mut log_lines = logging_output.lines();

        log_lines.next_some().assert_eq("spawner");
        log_lines
            .next_some()
            .assert_eq("spawned1 - parent: spawner");
        log_lines
            .next_some()
            .assert_eq("spawned2 - follows_from: spawner");
    }
}
