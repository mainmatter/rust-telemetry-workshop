use helpers::LogOutput;
use what_to_log::TestLogger;

#[test]
fn failure() {
    let logging_buffer = TestLogger::init();
    let order_numbers = vec![3, 4, 5];

    what_to_log::get_total(&order_numbers).unwrap_err();

    // Check that the log output matches what we expect.
    let logging_output = LogOutput::new(logging_buffer.lock().unwrap().clone());
    let mut log_lines = logging_output.lines();

    log_lines
        .next_some()
        .assert_eq("START - process total price");

    log_lines.next_some().assert_eq("START - retrieve order");
    log_lines
        .next_some()
        .assert_regex_match(r"^END - retrieve order - SUCCESS - \d+ms$");

    log_lines.next_some().assert_eq("START - retrieve order");
    log_lines
        .next_some()
        .assert_regex_match(r"^END - retrieve order - ERROR - \d+ms$");

    log_lines
        .next_some()
        .assert_regex_match(r"^END - process total price - ERROR - \d+ms$");

    log_lines.end();
}
