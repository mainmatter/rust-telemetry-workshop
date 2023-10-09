use helpers::MockWriter;
use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_test_subscriber() -> MockWriter {
    let writer = MockWriter::new();
    let writer2 = writer.clone();
    tracing_subscriber::fmt()
        .with_writer(move || writer2.clone())
        // We want to see a log record for each span lifecycle stage.
        .with_span_events(FmtSpan::FULL)
        .json()
        .flatten_event(true)
        .init();
    writer
}
