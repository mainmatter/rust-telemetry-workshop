# Log levels

## The cost of logging

Logs are useful, but they come at a cost.\
Formatting, serializing, and writing log messages to a file or a socket takes time. It is
not uncommon to hear about applications that spend more time writing logs than doing actual
work.

As a rule of thumb, we want each log record emitted by our application to be **useful**.

## How do you define "useful"?

Easier said than done.

The usefulness of a log record can be situational: redundant under normal circumstances
but **vital** when trying to troubleshoot a particularly vicious bug.

Usefulness is also subjective: depending on your **intent**, you might or might not care
about a particular log record. This is a challenge for library authors: their software
will be used in a variety of contexts, and it's hard to predict what information will be
useful to the final application.

How do we reconcile these needs?

## Log filtering

The common solution is to use **log filters**.\
A log filter is a predicate that determines whether a log record should be emitted or not.

The filters are defined by the final binary, the application, therefore they are **tuned based
on the application's needs**.\
Library authors can instrument their code ~freely, knowing that the application owner will be
in charge of deciding what they want or don't want to see.

Filters rely primarily on two pieces of information:

- the **log level** of the record, a measure of its importance
- the **module** that emitted the record, also known as **the target**.

Filters are evaluated at runtime for each log record, therefore they aren't entirely zero-cost,
but they let us skip the most expensive parts of the logging process (formatting, serializing,
emitting), therefore amortizing the cost of "unnecessary" log records.

### Compile-time filtering

There is an exception though: you can filter out log records based on their level at compile
time, using some of the `cargo` features exposed by the `log` crate.\
The filtered log statements disappear entirely: they don't exist in the compiled binary,
therefore they don't incur any runtime cost.\
It is a great option to remove the noisiest log statements (e.g. `TRACE`-level), although
it's not a silver bullet: you can't use it to filter out log records based on their source.
