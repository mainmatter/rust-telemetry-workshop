use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Read the arguments that have been passed to the program.
    let args: Vec<String> = std::env::args().collect();

    // We configure the logger to emit all log records to **stdout**
    log_koan::SimpleLogger::init(std::io::stdout())?;

    // We now invoke our (trivial) business logic
    log_koan::entrypoint(&args[1..])
}

#[cfg(test)]
mod tests {
    use assert_cmd::assert::Assert;
    use assert_cmd::Command;
    use std::str::from_utf8;

    fn command() -> Command {
        Command::cargo_bin("stdout").unwrap()
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

        // The error message returned by the `main` function is automatically
        // printed to `stderr` in Rust programs.
        assert_eq!(
            stderr,
            r#"Error: "You have only passed one argument to the program, you need another one!"
"#
        );
        assert_eq!(
            stdout,
            r#"Retrieving first argument
Retrieving second argument
"#
        );
    }

    #[test]
    fn no_arg() {
        let assert = command().assert().failure();
        let stdout = stdout(&assert);
        let stderr = stderr(&assert);

        assert_eq!(
            stderr,
            r#"Error: "You haven't passed any argument to the program! Two is the minimum."
"#
        );
        assert_eq!(stdout, "Retrieving first argument\n");
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
