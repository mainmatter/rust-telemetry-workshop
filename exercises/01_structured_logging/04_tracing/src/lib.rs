//! # The `tracing` crate
//!
//! ## `log` is not the right abstraction
//!
//! In the previous exercise, we laid out the key abstraction we need to model our applications:
//! **units of work**.  
//! We've also tried to leverage the `log` crate to properly track them, but you may have noticed
//! that it's not a perfect fit.  
//!
//! The `log` crate is structured around the concept of **log events**.  
//! There is no duration. There is also no **relationship** between one event and the other,
//! while we have seen how our units of work are naturally organised in a **hierarchical** fashion—
//! a unit of work may in turn contain multiple sub-units of work, and so on.
//!
//! We need to change our instrumentation library to one that is more suited to our needs: the
//! `tracing` crate.
//!
//! ## `tracing::Span`
//!
//! The `tracing` crate models units of work as **spans**.  
//! A span is a unit of work that has a **start** and an **end**.
//! A span can also have a parent, which naturally introduces that hierarchical relationship we
//! were looking for.
//!
//! The richer data model translates into a more complex interface, both on the instrumentation
//! side (i.e. the code that emits spans) and on the consumer side (i.e. the code that processes
//! spans).  
//! This is one of the reasons I chose to talk about `log` first: it's a gentler introduction
//! to the overall facade pattern and by probing its limitations you (hopefully) have a better
//! understanding of the rationale behind `tracing`'s additional complexity.
//!
//! # Exercise
//!
//! We'll ignore the consumer side for now and focus on the instrumentation side.  
//! We'll redo the previous exercise using the `tracing` crate in order to get familiar with the
//! basics.
mod subscriber;

pub use subscriber::init_test_subscriber;

/// Given a list of order numbers, compute the total price.
///
/// # Exercise
///
/// Wrap `get_total` and `get_order_details`, our two units of work, in a `tracing::Span`.
/// We don't care about capturing the outcome of each unit of work (for now).
///
/// Refer to the test files for the expected output format.
pub fn get_total(order_numbers: &[u64]) -> Result<u64, anyhow::Error> {
    // Tip: use `tracing::info_span!` to create a new span.
    // You'll have to learn about the *RAII guard* pattern!
    let span = tracing::info_span!("process total price");
    let _guard = span.enter();
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
    let span = tracing::info_span!("retrieve order");
    let _guard = span.enter();
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
