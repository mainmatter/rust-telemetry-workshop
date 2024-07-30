# The `metrics` crate

How does the `metrics` crate work?
`log` used the facade pattern, `tracing` used the facade pattern... You guessed it, `metrics`
uses the facade pattern as well!

It exposes a set of macros to create and manipulate metrics.
The metric data is then forwarded to an implementor of the `metrics::Recorder` trait,
which takes care of processing it.

It should all feel pretty familiar by now!
