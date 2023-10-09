//! # Structured output
//!
//! In all our exercises (and tests), we've been looking at the emitted diagnostics for a
//! single invocation of an instrumented function.
//! In the real world, however, we can expect to have hundreds if not thousands of invocations
//! per second, each of which will emit its own diagnostics and might result in a different
//! outcome.
//!
//! In order to make sense of all this data, we need to be able to **analyze it**.
//!
//! We might want to know, for example, how many failures we have had in the last hour. Queries
//! can also get more complex: how many failures have we had in the last hour, broken down by
//! error type and by user?
//!
//! So far we've always been looking at a single line of text which interpolates the target of
//! the span, the values of our fields and other metadata (e.g. timing) into a human-readable string.
//! That kind of representation is not very amenable to machine processing.
//!
//! In order to answer those questions, we need to be able to **structure** our output.
//!
//! The simplest data format we can use is JSON.
//!
//! # Exercise
//!
//! Change the subscriber settings to output JSON instead of plain text.
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
#[instrument("retrieve order", skip_all, fields(outcome))]
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
