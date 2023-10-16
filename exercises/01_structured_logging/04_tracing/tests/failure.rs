use tracing_training::init_test_subscriber;

#[test]
fn failure() {
    let logging_buffer = init_test_subscriber();
    let order_numbers = vec![3, 4, 5];

    tracing_training::get_total(&order_numbers).unwrap_err();

    // Check that the log output matches what we expect.
    let logging_output = logging_buffer.log_output().unwrap();
    let mut log_lines = logging_output.lines();

    log_lines.next_some().assert_eq("process total price: new");

    log_lines
        .next_some()
        .assert_eq("process total price:retrieve order: new");
    log_lines
        .next_some()
        .assert_eq("process total price:retrieve order: exit");

    log_lines
        .next_some()
        .assert_eq("process total price:retrieve order: new");
    log_lines
        .next_some()
        .assert_eq("process total price:retrieve order: exit");

    log_lines.next_some().assert_eq("process total price: exit");

    log_lines.end();
}
