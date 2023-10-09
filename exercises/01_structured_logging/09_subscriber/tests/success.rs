use opentelemetry::global::shutdown_tracer_provider;
use serde_json::json;
use subscriber::init_test_subscriber;

#[tokio::test]
async fn success() {
    let logging_buffer = init_test_subscriber();
    let order_numbers = vec![1, 2, 3];

    let total = subscriber::get_total(&order_numbers).unwrap();

    // Check that the total is correct.
    assert_eq!(total, 3117);

    // Check that the log output matches what we expect.
    let logging_output = logging_buffer.log_output().unwrap();
    let mut log_lines = logging_output.lines();

    log_lines.next_some().assert_json_include(
        json!({"message":"new","span":{"name":"process total price"},"spans":[]}),
    );
    log_lines.next_some().assert_json_include(
        json!({"message":"exit","span":{"name":"process total price","outcome":"success"},"spans":[]})
    );
    log_lines.end();

    // Ensure all spans are exported
    tokio::task::spawn_blocking(|| shutdown_tracer_provider())
        .await
        .unwrap();
}
