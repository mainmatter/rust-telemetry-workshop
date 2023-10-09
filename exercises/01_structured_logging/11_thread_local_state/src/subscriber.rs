use helpers::MockWriter;
use std::io::Write;
use tracing::{Id, Subscriber};
use tracing_subscriber::layer::{Context, SubscriberExt};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;

pub fn init_test_subscriber() -> MockWriter {
    let writer = MockWriter::new();
    let writer2 = writer.clone();
    Registry::default()
        .with(MockLayer { writer: writer2 })
        .init();
    writer
}

struct MockLayer {
    writer: MockWriter,
}

impl<S> tracing_subscriber::Layer<S> for MockLayer
where
    S: Subscriber + for<'span> tracing_subscriber::registry::LookupSpan<'span>,
{
    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).unwrap();
        let name = span.name();
        let parent = span
            .parent()
            .map(|p| format!(" - parent: {}", p.name()))
            .unwrap_or_default();
        let follows_from_id = span.extensions().get::<Id>().cloned();
        let follows_from = follows_from_id
            .map(|p| {
                let p = ctx.span(&p).unwrap();
                format!(" - follows_from: {}", p.name())
            })
            .unwrap_or_default();
        let mut buffer = self.writer.buf().unwrap();
        writeln!(&mut buffer, "{name}{parent}{follows_from}").unwrap();
    }

    fn on_follows_from(&self, span: &Id, follows: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(span).unwrap();
        span.extensions_mut().insert(follows.clone());
    }
}
