# The lifecycle of a span

All the instrumentation work we have done so far has happened in a
**synchronous** context.\
We have only used async Rust on the processing side, to use non-blocking IO to ship telemetry
data to a sink over the network (i.e. Honeycomb's API).

But what if we wanted to **instrument asynchronous code**?

It's trickier!\
There are some subtleties to be aware of, at the intersection between the lifecycle of a
`tracing` span, work-stealing async runtimes and `tracing`'s internal state management.

It's a lot to cover in a single exercise, so we'll break it down into smaller chunks.\
Let's start with the lifecycle of a span.

# A span was born—what then?

You've seen in the first `tracing` exercises that there are two distinct phases in the lifecycle
of a span:

- **creation**, i.e. what happens when `tracing::<level>_span!` is called;
- **entering**, i.e. what happens when you can `.enter()` on a span, getting back a RAII guard.

A span can only be created **once**, but it can be entered **multiple times**.

```rust
use tracing::info_span;

let span = info_span!("my span");
{
    let _guard = span.enter();
    // Do something
}
{
    // We re-enter the same span!
    let _guard2 = span.enter();
    // Do something else
}
```

Why would you do that, you may be wondering?\
Well, your span may be tracking a piece of work that can be **paused** (async, remember?).

Creation and entering have two matching phases: **closing** and **exiting**.

A span is **exited** when the guard object returned by `.enter()` is dropped.\
A span is **closed**, instead, when there are no more handles to it. It's the subscriber's
responsibility to do this bookkeeping.

When a span is entered into a single time, the closing and exiting phases usually happen one
after the other. This is what happened so far in our exercises.
