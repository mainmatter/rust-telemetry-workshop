# Ergonomics

Before I stated that we shouldn't treat each function as its own unit of work—it's way
too verbose and it leads to a lot of noise in the logs.\
Nonetheless, it is often the case that a unit of work maps to **a certain** function. The work
starts when that function is invoked and ends when it returns. That was indeed the case in the
previous exercise.

When it happens, we can use the [`instrument`](https://docs.rs/tracing/latest/tracing/attr.instrument.html) macro
to avoid having to manually create a span inside the function body.
