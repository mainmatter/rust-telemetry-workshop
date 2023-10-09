use opentelemetry::global::shutdown_tracer_provider;
use structured::init_test_subscriber;

#[tokio::test]
async fn success() {
    init_test_subscriber();
    let order_numbers = vec![1, 2, 3];

    let total = structured::get_total(&order_numbers).unwrap();

    // Check that the total is correct.
    assert_eq!(total, 3117);

    // Ensure all spans are exported
    tokio::task::spawn_blocking(|| shutdown_tracer_provider())
        .await
        .unwrap();
}
