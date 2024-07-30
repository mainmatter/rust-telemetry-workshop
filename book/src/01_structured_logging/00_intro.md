# Structured logging

We will start our observability journey with logs.

Logs are the most common kind of telemetry data.\
Even developers who have never heard of observability have an intuitive understanding of
the usefulness of logs: logs are what you look at when stuff goes south to understand what is
happening, crossing your fingers extra hard hoping you captured enough information to
troubleshoot effectively.

What are logs though?
The format varies, depending on the epoch, the platform and the technologies you are using.
The simplest starting point: a bunch of text pushed to `stdout`, with a line break to separate
the current record from the next one. For example:

```text
The application is starting on port 8080
Handling a request to /index
Returned a 200 OK
```

Three perfectly-valid log records for a web server. Enough to understand, from the outside, what is happening.

## `println!`

The standard library contains the most basic logging framework you can think of: [`println!`](https://doc.rust-lang.org/std/macro.println.html).
`println!` is a macro that writes a formatted string to the standard output.

```rust
let name = "Alice";
// Prints: Hello, Alice!
println!("Hello, {name}!");
```