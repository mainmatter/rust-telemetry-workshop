use log::LevelFilter;
use log_filter_koan::FilteredLogger;
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut module_filters = HashMap::new();
    module_filters.insert("log_filter_koan::one".to_string(), LevelFilter::Trace);
    FilteredLogger::init(LevelFilter::Warn, module_filters)?;

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
        let assert = Command::cargo_bin("module").unwrap().assert().success();
        let stdout = from_utf8(&assert.get_output().stdout).unwrap();

        assert_eq!(
            stdout,
            "Starting to do something!\nWorking really hard!\nOh no, it failed!\nAlmost done!\n"
        )
    }
}
