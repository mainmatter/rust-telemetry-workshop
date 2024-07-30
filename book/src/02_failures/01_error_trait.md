# The `Error` trait

In Rust, errors are expected to implement the `Error` trait (there are some exceptions,
but we'll ignore them for now).

The `Error` trait is defined in the standard library, and looks like this:

```rust
use std::fmt::{Debug, Display};

pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)>;
}
```

Let's unpack the definition:

- All errors must implement the `Debug` trait. This representation is primarily intended
  for **operators**. It is likely to expose internal details of the error and the system it
  was emitted from.
  In most cases, you can just use `#[derive(Debug)]` for your error type.
- All errors must implement the `Display` trait. This representation is primarily designed
  for **users**. It should be understandable by a person that is not familiar (nor has access)
  to the internals of the system.
- An errors may travel through multiple "layers" in your application. E.g. a failure to execute
  a query might arise from a network error, which in turn might be caused by a DNS resolution
  failure.
  Each additional semantic layer is often represented as a wrapper over the original error.
  The `source` method allows you to **walk the chain of errors** and inspect each layer.

What does this mean for us?

From a telemetry perspective, we should be careful and try to capture as much context
as possible!\
The `Display` representation is likely to omit details which are going to be necessary
to troubleshoot.
The `Debug` representation will be more verbose, but it might still miss
some crucial details which are only available in the `source` chain—e.g. it might say
"I failed to run `SELECT * FROM TABLE`" but it won't tell you "Timed out while trying to
connect to XYZ.com".

My suggestion is to be slightly wasteful and capture all three representations.
They're not going to be 100% orthogonal, but you'll be maximising your chances at capturing
all the information you need.
