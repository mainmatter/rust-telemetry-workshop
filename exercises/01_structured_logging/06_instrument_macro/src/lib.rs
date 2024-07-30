//! # Exercise
//!
//! Read the `instrument` macro documentation: https://docs.rs/tracing/latest/tracing/attr.instrument.html
//! Then use it to complete the same exercise as before, instead of manually creating and entering
//! spans.
//!
//! Tip: you can use `Span::current()` to get a reference to the current span from inside
//! the function body.
mod subscriber;

pub use subscriber::init_test_subscriber;

/// Given a list of order numbers, compute the total price.
pub fn get_total(order_numbers: &[u64]) -> Result<u64, anyhow::Error> {
    let mut total = 0;
    for order_number in order_numbers {
        let order_details = get_order_details(*order_number)?;
        total += order_details.price;
    }
    Ok(total)
}

pub struct OrderDetails {
    pub order_number: u64,
    pub price: u64,
}

/// A dummy function to simulate what would normally be a database query.
fn get_order_details(order_number: u64) -> Result<OrderDetails, anyhow::Error> {
    if order_number % 4 == 0 {
        Err(anyhow::anyhow!("Failed to talk to the database"))
    } else {
        let prices = vec![999, 1089, 1029];
        Ok(OrderDetails {
            order_number,
            price: prices[order_number as usize % prices.len()],
        })
    }
}
