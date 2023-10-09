//! # Error handling
//!
//! When monitoring a system, we are primarily interested in failures.
//! We want to know when something goes wrong, and we want to know why.
//!
//! This section of the workshop focuses on failures—in particular, how to capture their
//! occurrence in our telemetry data.
//!
//! Enjoy!

#[cfg(test)]
mod tests {
    #[test]
    fn starting_block() {
        let msg = format!("I'm ready to learn about failures!");
        assert_eq!(msg, "I'm ready to learn about failures!")
    }
}
