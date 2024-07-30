# Describe

Let's break down what happened in the last exercise.
You created a counter: a **monotonically increasing** value.  
Once it has been created, its value can only be incremented.

You can create a counter using the `counter!` macro.
You can then call `increment` on the `Counter` returned by the macro to increment its value.

`Counter` exposes another method, `absolute`, which is a bit special: it's primarily designed to **initialize**
a counter sequence, specifying a non-zero initial value.
Beware: recorders will enforce the monotonicity property for counters, so you can't rely on `absolute`
to artificially decrement an existing counter.

When using `counter!`, the `Recorder` implementation is expected to:

- Create a new counter with the name you specified, if one doesn't exist;
- Retrieve the counter, if one exists.

`UPSERT` behaviour, for the SQL-minded among us.

# Metadata

It can be useful to add metadata to your counters (and metrics in general).  
You can rely on the `describe_*` macros offered by the `metrics!` crate: they let you add
a unit (e.g. capture that the counter value represents bytes or seconds) and a description
(which recorders can then choose to expose when metrics are exported/analyzes).

