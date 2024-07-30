# Structured output

In all our exercises (and tests), we've been looking at the emitted diagnostics for a
single invocation of an instrumented function.
In the real world, however, we can expect to have hundreds if not thousands of invocations
per second, each of which will emit its own diagnostics and might result in a different
outcome.

In order to make sense of all this data, we need to be able to **analyze it**.

We might want to know, for example, how many failures we have had in the last hour. Queries
can also get more complex: how many failures have we had in the last hour, broken down by
error type and by user?

So far we've always been looking at a single line of text which interpolates the target of
the span, the values of our fields and other metadata (e.g. timing) into a human-readable string.
That kind of representation is not very amenable to machine processing.

In order to answer those questions, we need to be able to **structure** our output.

The simplest data format we can use is JSON.
