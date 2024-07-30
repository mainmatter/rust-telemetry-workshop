# Instrumenting asynchronous code

We're finally ready to tackle the big boss: instrumenting asynchronous code.

Let's start by brushing up on Rust's asynchronous programming model. If you're familiar
with the details, feel free to skim through!

## `Future`s

The cornerstone of Rust's asynchronous programming model is the `Future` trait.

```rust,ignore
pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

We refer to an implementor of the `Future` trait as a future.
A future represents a computation that will **eventually** produce a value (of type `Output`).
Futures are designed to be lazy—if you want them to make progress on whatever job they're
doing, you need to repeatedly call their `poll` method until they're done, i.e. `Poll::Ready`
is returned.

## Async runtimes

The component in charge of polling a future is known as an **asynchronous runtime**.
Rust's standard library does not include a "default" asynchronous runtime. You need to bring
your own, usually as a third-party dependency.

When it comes to `tracing`, there are two key things to keep in mind when it comes to
asynchronous runtimes:

- When a certain future can't make any more progress (i.e. it returns `Poll::Pending`) the
  runtime will usually try to run (on that very thread!) another future that has been queued up.
  This is what allows concurrency to be highly efficient!
- Many popular runtimes (e.g. `tokio` or `async-std`) are built around the concept of
  multi-threaded **work-stealing**. The next time a future is polled, it may be
  **on a different thread**.
  This model comes with some advantages: you don't need to worry too much about balancing your
  work across threads, the runtime will transparently take care of it for you.\
  It also introduces some challenges: you can't rely on thread-local state to keep track of
  values that the future cares about, because the next time it's polled it may be on a different
  thread where that state is not available (or it's set to a different value).

# `Instrumented`

As much as possible, we want our instrumentation code to behave correctly no matter what runtime
our code is being executed on.\
Let's assume that a future is a unit of work we want to track. We want to know how much time
it spent doing work (i.e. inside `poll`), as well as how long it takes in terms of wall
clock time.\
Based on what we discussed so far, it follows that:

- Any `tracing`-specific state that is associated to our future (i.e. its span handle)
  must be stored in the future itself.
- Whenever our future is polled, we need to re-enter its span.
- Whenever our future returns from `poll`, we need to exit its span.

This is the exact strategy implemented in the `tracing` crate via the `Instrumented` future:

```rust,ignore
// I have spared you the Pin-related stuff here, for simplicity.
// You can see the full implementation with all the details in `tracing`'s source code.
pub struct Instrumented<T> {
    inner: T,
    span: Span,
}

impl<T: Future> Future for Instrumented<T> {
    type Output = T::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let _enter = self.span.enter();
        self.inner.poll(cx)
    }
}
```

It wraps around the future you want to instrument and bundles it together with its tracking span.
Every time `poll` is called, we enter the span and then exit it when it returns.
One of those cases where the actual implementation is very simple compared to the rationale
that justifies it!
