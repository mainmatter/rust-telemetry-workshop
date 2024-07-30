# `tracing` subscribers

So far we've focused on the instrumentation side of `tracing`—spans, fields, etc.
But we haven't talked about how to **consume** the data we're producing.

# `Subscriber`

To see `tracing` in action we've already had to play with different processors:
interpolated text to the console, JSON, and OpenTelemetry.  

They are all examples of `tracing` **subscribers**—i.e. implementors of the
[`Subscriber`](https://docs.rs/tracing/latest/tracing/subscriber/trait.Subscriber.html)
trait.  

A subscriber is **much more complex** than what we have previously seen in this workshop,
the `Record` trait from the `log` crate.\
`tracing` offers a richer data model, therefore pushing more responsibilities to the subscriber.
For example: each span has to be assigned a unique ID, which is used to link spans together in a
hierarchical structure.\
Complexity is further compounded by the fact that `tracing` is designed to be extremely
low-overhead, to maximise its applicability in production environments.

## `Registry`

It's rare to find yourself implementing the `Subscriber` trait from scratch.  
More often than not, you'll rely on [`Registry`](https://docs.rs/tracing_subscriber/latest/tracing_subscriber/registry/struct.Registry.html)
as your foundation.\
It takes care of all the complicated bits (e.g. span ID generation and management) and
exposes a simple(r) interface for you to implement: the
[`Layer`](https://docs.rs/tracing_subscriber/latest/tracing_subscriber/layer/trait.Layer.html) trait.

## `Layer`

Even better: you don't even have to implement `Layer` yourself (unless you want to).  
You can **combine** multiple layers together using the `with` method exposed by
the `SubscriberExt` trait.\
You just stack multiple layers from the ecosystem on top of each other,
downstream of a `Registry`, and you're good to go.
