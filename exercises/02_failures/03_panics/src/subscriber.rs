use helpers::MockWriter;
use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_test_subscriber() -> MockWriter {
    let writer = MockWriter::new();
    let writer2 = writer.clone();
    tracing_subscriber::fmt()
        .with_writer(move || writer.clone())
        .with_span_events(FmtSpan::FULL)
        .compact()
        .with_ansi(false)
        .init();

    // TODO: Install the custom panic hook here
    // You can try to write one on your own, or you can choose to lean on
    // the `tracing-panic` crate
    std::panic::set_hook(Box::new(tracing_panic::panic_hook));

    writer2
}
