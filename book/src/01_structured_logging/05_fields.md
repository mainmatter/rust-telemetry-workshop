# Enriching spans with structured fields

The test assertions in the previous exercise have highlighted some of the `tracing` advantages
we talked about—e.g. the hierarchical relationship between spans which our subscriber translated
into a prefix-based structure for the log output.\
We haven't fully replicated our `log` version though:

1. we're not capturing the outcome of each unit of work
2. we're not capturing the duration of each unit of work

2 is easily achieved by tweaking the configuration for our subscriber (`.with_timer(Uptime)`),
so let's focus on 1., the outcome.  
To pull it off in an idiomatic way, we need to learn about **span fields**.

## Span fields

`tracing` allows us to attach key-value pairs to spans, which we refer to as **span fields**.  
The syntax looks like this:

```rust
tracing::info_span!("my special task", foo = 42, bar = "hello");
```

You can also change the value of a field after the span has been created using `.record`:

```rust
let span = tracing::info_span!("my special task", foo = 42);
span.record("foo", 43);
```

The way fields are handled depends on the subscriber we're using, just like the span data itself.

## Fields must be defined upfront

It's important to point out one key limitation of span fields: they must be known when the
span is created. In other words:

```rust
// Don't do this in prod.
let span = tracing::info_span!("my special task");
span.record("foo", 43);
```

won't work because the field `foo` is not defined when the span is created. No error will be
raised, but the field will be ignored at runtime—that `record` call will have no effect
whatsoever.  

You may be wondering: what if I don't know the value of a field upfront?\
Good question! You can use the `tracing::field::Empty` as value for it when defining the span:

```rust
// This works!
let span = tracing::info_span!("my special task", foo = tracing::field::Empty);
span.record("foo", 43);
```

This limitation stems from `tracing`'s commitment to low overhead: knowing the fields upfront
allows `tracing` to avoid a few heap allocations in the hot path of your application.
