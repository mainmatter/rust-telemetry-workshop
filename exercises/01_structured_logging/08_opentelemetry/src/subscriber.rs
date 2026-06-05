use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::{WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::trace::SdkTracerProvider;
use tonic::metadata::MetadataMap;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;

pub fn init_test_subscriber() -> SdkTracerProvider {
    let provider = init_tracer_provider();
    let tracer = provider.tracer("rust-telemetry-workshop");
    let otel = tracing_opentelemetry::layer().with_tracer(tracer);

    // Here we are using the `Layer` trait from the `tracing-subscriber` crate to combine together
    // multiple pieces of functionality into a single subscriber.
    // We'll talk more about layers later in the workshop.
    Registry::default().with(otel).init();

    provider
}

pub fn init_tracer_provider() -> SdkTracerProvider {
    let honeycomb_key =
        std::env::var("HONEYCOMB_API_KEY").expect("`HONEYCOMB_API_KEY` must be set");
    let mut map = MetadataMap::with_capacity(1);
    map.insert("x-honeycomb-team", honeycomb_key.try_into().unwrap());

    // Correctly configuring your exporter is a bit of a black art and highly-dependent on the
    // specifics of your deployment environment.
    // We won't go into the details here, but you can read more about it in the OpenTelemetry
    // documentation (or grab me after the workshop to talk about it).
    // At a super high-level: you want batching and you want a sensible sampling strategy,
    // but beyond that it's hard to give general advice.
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint("https://api.honeycomb.io/api/traces")
        .with_timeout(std::time::Duration::from_secs(5))
        .with_metadata(map)
        .build()
        .expect("Error setting up Open Telemetry exporter");

    opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .build()
}
