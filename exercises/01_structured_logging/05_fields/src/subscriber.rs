use helpers::MockWriter;
use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_test_subscriber() -> MockWriter {
    let writer = MockWriter::new();
    let writer2 = writer.clone();
    tracing_subscriber::fmt()
        .without_time()
        .with_writer(move || writer.clone())
        .with_ansi(false)
        .with_level(false)
        .with_target(false)
        .with_span_events(FmtSpan::NEW | FmtSpan::EXIT)
        .compact()
        .init();
    writer2
}
