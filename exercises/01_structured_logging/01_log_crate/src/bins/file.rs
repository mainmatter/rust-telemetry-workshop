use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // Read the arguments that have been passed to the program.
    let args: Vec<String> = std::env::args().collect();
    // We extract the first argument as the name of the file we should emit logs to.
    let log_file_path = args
        .get(1)
        .ok_or("You need to pass a path to a log file as first argument!")?;

    // We configure the logger to emit all log records to **a file**.
    let log_file: fs_err::File = fs_err::File::create(log_file_path)?;
    log_koan::SimpleLogger::init(log_file)?;

    // We now invoke our (trivial) business logic
    log_koan::entrypoint(&args[2..])
}

#[cfg(test)]
mod tests {
    use assert_cmd::assert::Assert;
    use assert_cmd::Command;
    use std::str::from_utf8;
    use tempfile::NamedTempFile;

    /// Create a temporary file. It will be automatically deleted when the test completes.
    ///
    /// Tip: check out the `tempfile` crate to work with temporary files in tests!
    fn log_file() -> NamedTempFile {
        NamedTempFile::new().unwrap()
    }

    /// The command invocation, pre-configured to include the path to a temporary file for logging.
    fn base_command() -> (Command, NamedTempFile) {
        let log_file = log_file();
        let mut cmd = Command::cargo_bin("file").unwrap();
        cmd.arg(log_file.path());
        (cmd, log_file)
    }

    #[test]
    fn happy_case() {
        let (mut cmd, log_file) = base_command();

        cmd.arg("hello").arg("world").assert().success();

        let logs = fs_err::read_to_string(log_file.path()).unwrap();
        assert_eq!(
            &logs,
            r#"Retrieving first argument
Retrieving second argument
hello world
"#
        )
    }

    #[test]
    fn one_arg() {
        let (mut cmd, log_file) = base_command();

        let assert = cmd.arg("hello").assert().failure();
        let stderr = stderr(&assert);

        assert_eq!(
            stderr,
            "Error: \"You have only passed one argument to the program, you need another one!\"\n"
        );

        let logs = fs_err::read_to_string(log_file.path()).unwrap();
        assert_eq!(
            logs,
            r#"Retrieving first argument
Retrieving second argument
"#
        );
    }

    #[test]
    fn no_arg() {
        let (mut cmd, log_file) = base_command();

        let assert = cmd.assert().failure();
        let stderr = stderr(&assert);

        assert_eq!(
            stderr,
            "Error: \"You haven't passed any argument to the program! Two is the minimum.\"\n"
        );

        let logs = fs_err::read_to_string(log_file.path()).unwrap();
        assert_eq!(logs, "Retrieving first argument\n")
    }

    fn stderr(assert: &Assert) -> &str {
        let output = assert.get_output();
        from_utf8(&output.stderr).unwrap()
    }
}
