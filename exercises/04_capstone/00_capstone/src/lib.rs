//! # Combining everything together
//!
//! We've covered a lot of ground together: structured logging, failure handling, metrics.
//! I've tried to break each topic down into small bites, empowering you to build up your
//! knowledge incrementally.
//!
//! It's time to put everything together!
//! Pick your web framework of choice—I recommend either `actix-web` or `axum`.
//!
//! You have to:
//! - Configure a `tracing` subscriber that exports data to both Honeycomb and stdout, in JSON format
//! - Configure a suitable panic hook
//! - Configure a `metric` recorder that exposes metric data at `/metrics`, using a different port
//!   than your API endpoints
//! - Add one or more middleware to:
//!   - Create a top-level INFO span for each incoming request
//!   - Track the number of concurrent requests using a gauge
//!   - Track request duration using a histogram
//!   - Track the number of handled requests
//!   All metrics should include success/failure as a label.
//!
//! Bonus points if you:
//! - devise a mechanism for your request handlers to get a handle to the root span, in order to
//!   populate span fields with context from your domain layer
//! - allow changing the set of filtered/included `tracing` spans at runtime
//!
//! I don't have a suite of tests for you here, but please call me in when you're done—I want to
//! see what you come with!

#[cfg(test)]
mod tests {
    #[test]
    fn the_end() {
        let msg = format!("I know how to instrument my Rust applications!");
        assert_eq!(msg, "I know how to instrument my Rust applications!")
    }
}
