use opentelemetry_training::init_test_subscriber;

#[tokio::test]
async fn failure() {
    let provider = init_test_subscriber();
    let order_numbers = vec![3, 4, 5];

    opentelemetry_training::get_total(&order_numbers).unwrap_err();

    // Ensure all spans are exported
    tokio::task::spawn_blocking(move || provider.shutdown())
        .await
        .unwrap()
        .expect("Error shutting down Open Telemetry tracing");
}
