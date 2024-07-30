# The `log` crate

## Coupling

`println!` and its sibling `eprintln!` can get you started, but they won't get you very far.

Our ultimate goal is to monitor our applications in production environments. Each production
environment is its own little world, with its own constraints and requirements. In particular,
expectations around log data vary wildly: some environments expect logs to be written to
`stdout`, others to `stderr`, others to a file, others to a socket, and so on.

`println!` breaks down quickly in the face of such requirements: it **couples together** the
information that we want to log (i.e. the message) with the way we want to log it (i.e. the
destination, `stdout`).\
To change the destination, we would have to change the code that produces the log message.
That's a problem: that log message might be coming from a third-party library,
or from a part of the codebase that we don't own.  \
Having to fork those modules to accommodate the logging requirements of the final application
is a non-starter.

## Facade

We need to **decouple**.\
On one side, we will have an **instrumentation API**, used to emit log messages from the
application and its dependencies.\
On the other side, we will have the **processing code**, the logic in charge of deciding
what to do with the log messages that have been produced.

If you are familiar with the Gang of Four's Design Patterns, you might have recognized this as
an instance of the **facade pattern**.

## The `log` crate

The facade pattern only works if the ecosystem, as a whole, standardizes around the same facade.
You can then use a single pipeline to collect and process instrumentation data coming from
both first party code and third party dependencies.

In the early days of the Rust ecosystem, the `log` crate arose as the de-facto standard
logging facade for Rust applications.\
Let's have a look at it!

