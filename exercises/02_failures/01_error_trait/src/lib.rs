//! # The `Error` trait
//!
//! In Rust, errors are expected to implement the `Error` trait (there are some exceptions,
//! but we'll ignore them for now).
//!
//! The `Error` trait is defined in the standard library, and looks like this:
//!
//! ```rust
//! use std::fmt::{Debug, Display};
//!
//! pub trait Error: Debug + Display {
//!     fn source(&self) -> Option<&(dyn Error + 'static)>;
//! }
//! ```
//!
//! Let's unpack the definition:
//!
//! - All errors must implement the `Debug` trait. This representation is primarily intended
//!   for **operators**. It is likely to expose internal details of the error and the system it
//!   was emitted from.
//!   In most cases, you can just use `#[derive(Debug)]` for your error type.
//! - All errors must implement the `Display` trait. This representation is primarily designed
//!   for **users**. It should be understandable by a person that is not familiar (nor has access)
//!   to the internals of the system.
//! - An errors may travel through multiple "layers" in your application. E.g. a failure to execute
//!   a query might arise from a network error, which in turn might be caused by a DNS resolution
//!   failure.
//!   Each additional semantic layer is often represented as a wrapper over the original error.
//!   The `source` method allows you to **walk the chain of errors** and inspect each layer.
//!
//! What does this mean for us?
//!
//! From a telemetry perspective, we should be careful and try to capture as much context
//! as possible!  
//! The `Display` representation is likely to omit details which are going to be necessary
//! to troubleshoot.
//! The `Debug` representation will be more verbose, but it might still miss
//! some crucial details which are only available in the `source` chain—e.g. it might say
//! "I failed to run `SELECT * FROM TABLE`" but it won't tell you "Timed out while trying to
//! connect to XYZ.com".
//!
//! My suggestion is to be slightly wasteful and capture all three representations.
//! They're not going to be 100% orthogonal, but you'll be maximising your chances at capturing
//! all the information you need.
//!
//! # Exercise
//!
//! Fill out the missing details in `telemetry_wrapper` to capture the information we just discussed.

use std::error::Error;
use std::fmt::{Debug, Write};
use std::fmt::{Display, Formatter};
use std::io::ErrorKind;
use tracing::field::Empty;
use tracing::Span;

mod subscriber;

/// Another interesting question when it comes to errors: who should log them?
///
/// The answer depends on what you care about and the facilities available to you.
/// In an API, my recommendation is to have a top-level middleware which takes care of logging
/// all "fatal" errors—i.e. errors that will be converted directly into an error HTTP response for
/// your caller.
///
/// This `telemetry_wrapper` function is meant to play such a role: it wraps the underlying
/// computation and takes care of logging any errors that arise from it.
#[tracing::instrument(
    "my_task", 
    // The precise naming choice here is not important, it's likely to be determined by
    // the conventions and constraints of your team/org. 
    // What matters is capturing the enough information!
    fields(error.msg = Empty, error.debug = Empty, error.source_chain = Empty)
)]
fn telemetry_wrapper() -> Result<(), OpaqueError> {
    let outcome = fallible_operation();
    if let Err(e) = &outcome {
        Span::current().record("error.msg", tracing::field::display(e));
        Span::current().record("error.debug", tracing::field::debug(e));
        Span::current().record("error.source_chain", source_chain(e));
    }
    outcome
}

/// Return a string representation of the source chain of the given error.
fn source_chain<E>(e: E) -> String
where
    E: Error,
{
    let mut result = String::new();
    let mut source = e.source();
    while let Some(e) = source {
        // We use the debug representation here to maximise the amount of information we capture
        // about errors in the chain.
        writeln!(&mut result, "{:?}", e).unwrap();
        source = e.source();
    }
    result
}

fn fallible_operation() -> Result<(), OpaqueError> {
    let root_error = std::io::Error::new(
        ErrorKind::ConnectionRefused,
        "Failed to connect to 127.0.0.1:4236",
    );
    let intermediate_error = DatabaseError {
        source: root_error,
        query: "SELECT * FROM table",
    };
    Err(OpaqueError {
        source: intermediate_error,
    })
}

struct DatabaseError {
    source: std::io::Error,
    query: &'static str,
}

impl Debug for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to execute: `{}`", self.query)
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to execute a database query")
    }
}

impl Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug)]
struct OpaqueError {
    source: DatabaseError,
}

impl Display for OpaqueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // This is our top-level error and we don't talk about databases with our end users,
        // it's an implementation detail of the system.
        write!(
            f,
            "The service is temporarily unavailable, please try again later"
        )
    }
}

impl Error for OpaqueError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

#[cfg(test)]
mod tests {
    use super::telemetry_wrapper;
    use crate::subscriber::init_test_subscriber;

    #[test]
    fn error_span() {
        let logging_buffer = init_test_subscriber();

        let error = telemetry_wrapper().unwrap_err();

        // Check that the log output matches what we expect.
        let logging_output = logging_buffer.log_output().unwrap();
        let logging_output = logging_output.text();

        assert!(
            logging_output.contains(
                "error.msg=The service is temporarily unavailable, please try again later"
            ),
            "The logging output is missing the expected error.msg:\n{}",
            logging_output
        );
        // Depending on how each error type implements `Debug` and `Display`, we might get
        // different levels of overlap between `error.debug` and `error.source_chain`.
        assert!(
            logging_output.contains(
                r#"error.debug=OpaqueError { source: Failed to execute: `SELECT * FROM table` }"#
            ),
            "The logging output is missing the expected error.debug:\n{}",
            logging_output
        );
        assert!(logging_output.contains(r#"error.source_chain="Failed to execute: `SELECT * FROM table`\nCustom { kind: ConnectionRefused, error: \"Failed to connect to 127.0.0.1:4236\" }"#),
            "The logging output is missing the expected error.source_chain:\n{}",
            logging_output
        );
    }
}
