use helpers::LogOutput;
use what_to_log::TestLogger;

#[test]
fn success() {
    let logging_buffer = TestLogger::init();
    let order_numbers = vec![1, 2, 3];

    let total = what_to_log::get_total(&order_numbers).unwrap();

    // Check that the total is correct.
    assert_eq!(total, 3117);
    // Check that the log output matches what we expect.
    let logging_output = LogOutput::new(logging_buffer.lock().unwrap().clone());
    let mut log_lines = logging_output.lines();

    log_lines
        .next_some()
        .assert_eq("START - process total price");

    for _ in 0..3 {
        log_lines.next_some().assert_eq("START - retrieve order");
        log_lines
            .next_some()
            .assert_regex_match(r"^END - retrieve order - SUCCESS - \d+ms$");
    }

    log_lines
        .next_some()
        .assert_regex_match(r"^END - process total price - SUCCESS - \d+ms$");

    log_lines.end();
}
