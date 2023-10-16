use tracing_training::init_test_subscriber;

#[test]
fn success() {
    let logging_buffer = init_test_subscriber();
    let order_numbers = vec![1, 2, 3];

    let total = tracing_training::get_total(&order_numbers).unwrap();

    // Check that the total is correct.
    assert_eq!(total, 3117);
    // Check that the log output matches what we expect.
    let logging_output = logging_buffer.log_output().unwrap();
    let mut log_lines = logging_output.lines();

    log_lines.next_some().assert_eq("process total price: new");

    for _ in 0..3 {
        log_lines
            .next_some()
            .assert_eq("process total price:retrieve order: new");
        log_lines
            .next_some()
            .assert_eq("process total price:retrieve order: exit");
    }

    log_lines.next_some().assert_eq("process total price: exit");

    log_lines.end();
}
