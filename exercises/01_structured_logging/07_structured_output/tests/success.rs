use serde_json::json;
use structured::init_test_subscriber;

#[test]
fn success() {
    let logging_buffer = init_test_subscriber();
    let order_numbers = vec![1, 2, 3];

    let total = structured::get_total(&order_numbers).unwrap();

    // Check that the total is correct.
    assert_eq!(total, 3117);
    // Check that the log output matches what we expect.
    let logging_output = logging_buffer.log_output().unwrap();
    let mut log_lines = logging_output.lines();

    log_lines.next_some().assert_json_include(
        json!({"message":"new","span":{"name":"process total price"},"spans":[]}),
    );

    for _ in 0..3 {
        log_lines
            .next_some()
            .assert_json_include(
                json!({"message":"new","span":{"name":"retrieve order"},"spans":[{"name":"process total price"}]})
            );
        log_lines
            .next_some()
            .assert_json_include(
                json!({"message":"exit","span":{"name":"retrieve order","outcome":"success"},"spans":[{"name":"process total price"}]})
            );
    }

    log_lines
        .next_some()
        .assert_json_include(
            json!({"message":"exit","span":{"name":"process total price","outcome":"success"},"spans":[]})
        );

    log_lines.end();
}
