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
