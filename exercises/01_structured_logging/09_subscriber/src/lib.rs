//! # `tracing` subscribers
//!
//! So far we've focused on the instrumentation side of `tracing`—spans, fields, etc.
//! But we haven't talked about how to **consume** the data we're producing.
//!
//! # `Subscriber`
//!
//! To see `tracing` in action we've already had to play with different processors:
//! interpolated text to the console, JSON, OpenTelemetry.  
//!
//! They are all examples of `tracing` **subscribers**—i.e. implementors of the
//! [`Subscriber`](https://docs.rs/tracing/latest/tracing/subscriber/trait.Subscriber.html)
//! trait.  
//!
//! A subscriber is **much more complex** than what we have previously seen in this workshop,
//! the `Record` trait from the `log` crate.  
//! `tracing` offers a richer data model, therefore pushing more responsibilities to the subscriber.
//! For example: each span has to be assigned a unique ID, which is used to link spans together in a
//! hierarchical structure.  
//! Complexity is further compounded by the fact that `tracing` is designed to be extremely
//! low-overhead, to maximise its applicability in production environments.
//!
//! ## `Registry`
//!
//! It's rare to find yourself implementing the `Subscriber` trait from scratch.  
//! More often than not, you'll rely on [`Registry`](https://docs.rs/tracing_subscriber/latest/tracing_subscriber/registry/struct.Registry.html)
//! as your foundation.  
//! It takes care of all the complicated bits (e.g. span ID generation and management) and
//! exposes a simple(r) interface for you to implement: the
//! [`Layer`](https://docs.rs/tracing_subscriber/latest/tracing_subscriber/layer/trait.Layer.html) trait.
//!
//! ## `Layer`
//!
//! Even better: you don't even have to implement `Layer` yourself (unless you want to).  
//! You can **combine** multiple layers together using the `with` method exposed by
//! the `SubscriberExt` trait.  
//! You just stack multiple layers from the ecosystem on top of each other,
//! downstream of a `Registry`, and you're good to go.
//!
//! # Exercise
//!
//! Build a `tracing` subscriber that:
//!
//! - Emits JSON-structured logs to an in-memory buffer
//! - Exports telemetry data in OpenTelemetry format to Honeycomb
//! - Only captures spans that are level `INFO` or above
//!
//! You can look at the subscribers we built in the previous exercises for inspiration!
mod subscriber;

pub use subscriber::init_test_subscriber;
use tracing::{instrument, Span};

/// Given a list of order numbers, compute the total price.
#[instrument("process total price", skip_all, fields(outcome))]
pub fn get_total(order_numbers: &[u64]) -> Result<u64, anyhow::Error> {
    let mut total = 0;
    for order_number in order_numbers {
        let order_details = get_order_details(*order_number).map_err(|e| {
            Span::current().record("outcome", "failure");
            e
        })?;
        total += order_details.price;
    }
    Span::current().record("outcome", "success");
    Ok(total)
}

pub struct OrderDetails {
    pub order_number: u64,
    pub price: u64,
}

/// A dummy function to simulate what would normally be a database query.
#[instrument("retrieve order", level = tracing::Level::TRACE, skip_all, fields(outcome))]
fn get_order_details(order_number: u64) -> Result<OrderDetails, anyhow::Error> {
    if order_number % 4 == 0 {
        Span::current().record("outcome", "failure");
        Err(anyhow::anyhow!("Failed to talk to the database"))
    } else {
        let prices = vec![999, 1089, 1029];
        Span::current().record("outcome", "success");
        Ok(OrderDetails {
            order_number,
            price: prices[order_number as usize % prices.len()],
        })
    }
}
