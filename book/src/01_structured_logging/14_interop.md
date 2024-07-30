# Interoperability

You have seen first-hand how much `tracing` brings to the table.
Nonetheless, migrating an entire ecosystem takes time: you can still find crates that rely
on `log` for instrumentation.

We need a way to bridge the gap between the two worlds: you should be able to use a single
pipeline to process telemetry coming from both sources.

# `tracing-log`

The `tracing-log` crate can act as such a bridge: it provides you with a `log`-compatible
processor which redirects all `log` events to `tracing`.  

Be mindful: if you're using `tracing_subscriber::fmt` as your `tracing` subscriber, it'll
automatically install this bridge for you unless you explicitly disable the `tracing-log`
feature.
