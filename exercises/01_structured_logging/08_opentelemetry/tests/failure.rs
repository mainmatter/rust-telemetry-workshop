use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry_training::init_test_subscriber;
use std::time::Duration;

#[tokio::test]
async fn failure() {
    init_test_subscriber();
    let order_numbers = vec![3, 4, 5];

    opentelemetry_training::get_total(&order_numbers).unwrap_err();

    // Wait for the batch exporter to export all spans before the test is finished
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Ensure all spans are exported
    tokio::task::spawn_blocking(|| shutdown_tracer_provider())
        .await
        .unwrap();
}
