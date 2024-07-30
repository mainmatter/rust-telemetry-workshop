# Instrumenting asynchronous code - Part 2

`Instrumented` is neat, but ergonomics are a bit poor: in order to attach a span to an
invocation of an async function we need to write a bunch of wrapping glue code.

Wrapping glue code... we've complained about it before, haven't we?
Yes! When attaching a span to a synchronous function invocation.

`#[tracing::instrument]` is here to rescue us, **again**!
The macro is smart enough to detect that the function it is applied to is asynchronous:
it will automatically generate the required glue code and attach the span using `.instrument`.
