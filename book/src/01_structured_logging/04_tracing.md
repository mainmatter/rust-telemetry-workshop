# The `tracing` crate

## `log` is not the right abstraction

In the previous exercise, we laid out the key abstraction we need to model our applications:
**units of work**.\
We've also tried to leverage the `log` crate to properly track them, but you may have noticed
that it's not a perfect fit.

The `log` crate is structured around the concept of **log events**.\
There is no duration. There is also no **relationship** between one event and the other,
while we have seen how our units of work are naturally organised in a **hierarchical** fashion—
a unit of work may in turn contain multiple sub-units of work, and so on.

We need to change our instrumentation library to one that is more suited to our needs: the
`tracing` crate.

## `tracing::Span`

The `tracing` crate models units of work as **spans**.\
A span is a unit of work that has a **start** and an **end**.
A span can also have a parent, which naturally introduces that hierarchical relationship we
were looking for.

The richer data model translates into a more complex interface, both on the instrumentation
side (i.e. the code that emits spans) and on the consumer side (i.e. the code that processes
spans).\
This is one of the reasons I chose to talk about `log` first: it's a gentler introduction
to the overall facade pattern and by probing its limitations you (hopefully) have a better
understanding of the rationale behind `tracing`'s additional complexity.
