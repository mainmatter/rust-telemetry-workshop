//! # Enriching spans with structured fields
//!
//! The test assertions in the previous exercise have highlighted some of the `tracing` advantages
//! we talked about—e.g. the hierarchical relationship between spans which our subscriber translated
//! into a prefix-based structure for the log output.  
//! We haven't fully replicated our `log` version though:
//!
//! 1. we're not capturing the outcome of each unit of work
//! 2. we're not capturing the duration of each unit of work
//!
//! 2. is easily achieved by tweaking the configuration for our subscriber (`.with_timer(Uptime)`),
//! so let's focus on 1., the outcome.  
//! To pull it off in an idiomatic way, we need to learn about **span fields**.
//!
//! ## Span fields
//!
//! `tracing` allows us to attach key-value pairs to spans, which we refer to as **span fields**.  
//! The syntax looks like this:
//!
//! ```rust
//! tracing::info_span!("my special task", foo = 42, bar = "hello");
//! ```
//!
//! You can also change the value of a field after the span has been created using `.record`:
//!
//! ```rust
//! let span = tracing::info_span!("my special task", foo = 42);
//! span.record("foo", 43);
//! ```
//!
//! The way fields are handled depends on the subscriber we're using, just like the span data itself.
//!
//! ## Fields must be defined upfront
//!
//! It's important to point out one key limitation of span fields: they must be known when the
//! span is created.  
//! In other words:
//!
//! ```rust
//! // Don't do this in prod.
//! let span = tracing::info_span!("my special task");
//! span.record("foo", 43);
//! ```
//!
//! won't work because the field `foo` is not defined when the span is created. No error will be
//! raised, but the field will be ignored at runtime—that `record` call will have no effect
//! whatsoever.  
//!
//! You may be wondering: what if I don't know the value of a field upfront?
//! Good question! You can use the `tracing::field::Empty` as value for it when defining the span:
//!
//! ```rust
//! // This works!
//! let span = tracing::info_span!("my special task", foo = tracing::field::Empty);
//! span.record("foo", 43);
//! ```
//!
//! This limitation stems from `tracing`'s commitment to low overhead: knowing the fields upfront
//! allows `tracing` to avoid a few heap allocations in the hot path of your application.
mod subscriber;

pub use subscriber::init_test_subscriber;

/// Given a list of order numbers, compute the total price.
///
/// # Exercise
///
/// The same exercise as before, with one twist: we want to capture the outcome of each unit of
/// work this time!
///
/// Refer to the test files for the expected output format.
pub fn get_total(order_numbers: &[u64]) -> Result<u64, anyhow::Error> {
    let span = tracing::info_span!("process total price", outcome = tracing::field::Empty);
    let _guard = span.enter();
    let mut total = 0;
    for order_number in order_numbers {
        let order_details = get_order_details(*order_number).map_err(|e| {
            span.record("outcome", "failure");
            e
        })?;
        total += order_details.price;
    }
    span.record("outcome", "success");
    Ok(total)
}

pub struct OrderDetails {
    pub order_number: u64,
    pub price: u64,
}

/// A dummy function to simulate what would normally be a database query.
fn get_order_details(order_number: u64) -> Result<OrderDetails, anyhow::Error> {
    let span = tracing::info_span!("retrieve order", outcome = tracing::field::Empty);
    let _guard = span.enter();
    if order_number % 4 == 0 {
        span.record("outcome", "failure");
        Err(anyhow::anyhow!("Failed to talk to the database"))
    } else {
        let prices = vec![999, 1089, 1029];
        span.record("outcome", "success");
        Ok(OrderDetails {
            order_number,
            price: prices[order_number as usize % prices.len()],
        })
    }
}
