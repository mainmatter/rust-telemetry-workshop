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
