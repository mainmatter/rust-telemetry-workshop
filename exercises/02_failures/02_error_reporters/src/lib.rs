//! # Error report types
//!
//! In the previous exercise I mentioned that not all error types actually implement the `Error`
//! trait.
//! The primary exceptions to this rule are **error report types**.
//!
//! An error report type is a type which is used to **report** about an error, with the potential
//! addition of some extra context. They're often used as **opaque error types**, i,.e. a way
//! to hide the actual type of the underlying error from the user when it is not relevant or
//! it would be a breach of semantics to expose it.
//!
//! Their common characteristic is that they want to implement a conversion from an arbitrary
//! error type (`E: Error`) into themselves.
//! This, in turn, means that they can't implement the `Error` trait themselves, as that would
//! result in a conflicting `From` implementation (see https://github.com/dtolnay/anyhow/issues/25#issuecomment-544140480).
//!
//! Examples in the ecosystem include:
//!
//! - `Box<dyn Error>`, the OG opaque error type
//! - `anyhow::Error`, a more ergonomic version of `Box<dyn Error>`
//! - `eyre::Report`, a fancier version of `anyhow::Error`
//!
//! and many more.
//!
//! It is fairly common to upcast all errors to one of these types at the boundary of your
//! application, and then use that opaque error type for all your error handling (and reporting)
//! needs.
//! When that's the case, you need to carefully study the documentation of the error report
//! type you're using: does it expose a `source` method, even if it doesn't implement the `Error`
//! trait? Does its `Debug` implementation already expose the entire source chain (e.g. `anyhow`)?
//!
//! By adapting your strategy to the specific type you're working with you can avoid emitting
//! duplicate information, or missing out on important details.
#[cfg(test)]
mod tests {
    #[test]
    fn continued() {
        // No exercise here, it was all just a big FYI!
        let msg = format!("I'm ready to __!");
        assert_eq!(msg, "I'm ready to continue!")
    }
}
