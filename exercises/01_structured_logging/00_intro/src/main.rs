//! # Logs
//!
//! We will start our observability journey with logs.
//!
//! Logs are the most common kind of telemetry data.
//! Even developers who have never heard of observability have an intuitive understanding of
//! the usefulness of logs: logs are what you look at when stuff goes south to understand what is
//! happening, crossing your fingers extra hard hoping you captured enough information to
//! troubleshoot effectively.
//!
//! What are logs though?
//! The format varies, depending on the epoch, the platform and the technologies you are using.
//! The simplest starting point: a bunch of text pushed to `stdout`, with a line break to separate
//! the current record from the next one. For example:
//!
//! ```text
//! The application is starting on port 8080
//! Handling a request to /index
//! Returned a 200 OK
//! ```
//!
//! Three perfectly-valid log records for a web server.
//! Enough to understand, from the outside, what is happening.

/// # Exercise
///
/// The standard library contains the most basic logging framework you can think of: [`println!`].
/// Add `println!` statements to the little CLI below as required to get the tests to pass.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the arguments that have been passed to the program.
    let args: Vec<String> = std::env::args().collect();

    println!("Retrieving first argument");
    let Some(a) = args.get(1) else {
        return Err("You haven't passed any argument to the program! Two is the minimum.".into());
    };
    println!("Retrieving second argument");
    let Some(b) = args.get(2) else {
        return Err(
            "You have only passed one argument to the program, you need another one!".into(),
        );
    };

    println!("{} {}", a, b);

    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_cmd::assert::Assert;
    use assert_cmd::Command;
    use std::str::from_utf8;

    /// We invoke the binary as if it was installed on the system.
    ///
    /// We are interacting with the program in the same way a *user* would—no magic hooks,
    /// no special privileges. Just looking at whatever diagnostic information the program
    /// is emitting to `stdout` and `stderr`.
    ///
    /// Tip: the `assert_cmd` crate is your friend when it comes to black-box testing of CLIs.
    fn command() -> Command {
        Command::cargo_bin("intro").unwrap()
    }

    #[test]
    fn happy_case() {
        let assert = command().arg("hello").arg("world").assert().success();
        let stdout = stdout(&assert);

        assert_eq!(
            stdout,
            r#"Retrieving first argument
Retrieving second argument
hello world
"#
        )
    }

    #[test]
    fn one_arg() {
        let assert = command().arg("hello").assert().failure();
        let stdout = stdout(&assert);
        let stderr = stderr(&assert);

        assert_eq!(
            stdout,
            r#"Retrieving first argument
Retrieving second argument
"#
        );
        // The error message returned by the `main` function is automatically
        // printed to `stderr` in Rust programs.
        assert_eq!(
            stderr,
            r#"Error: "You have only passed one argument to the program, you need another one!"
"#
        )
    }

    #[test]
    fn no_arg() {
        let assert = command().assert().failure();
        let stdout = stdout(&assert);
        let stderr = stderr(&assert);

        assert_eq!(stdout, "Retrieving first argument\n");
        assert_eq!(
            stderr,
            r#"Error: "You haven't passed any argument to the program! Two is the minimum."
"#
        )
    }

    fn stdout(assert: &Assert) -> &str {
        let output = assert.get_output();
        from_utf8(&output.stdout).unwrap()
    }

    fn stderr(assert: &Assert) -> &str {
        let output = assert.get_output();
        from_utf8(&output.stderr).unwrap()
    }
}
