use log::LevelFilter;
use log_filter_koan::FilteredLogger;
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    FilteredLogger::init(LevelFilter::Debug, HashMap::new())?;

    log_filter_koan::one::work();
    log_filter_koan::two::work();

    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::str::from_utf8;

    #[test]
    fn logs() {
        let assert = Command::cargo_bin("min_level").unwrap().assert().success();
        let stdout = from_utf8(&assert.get_output().stdout).unwrap();

        assert_eq!(
            stdout,
            "Working really hard!\nOh no, it failed!\nTime to do some work!\nAlmost done!\n"
        )
    }
}
