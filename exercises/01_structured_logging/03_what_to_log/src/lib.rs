//! # What to log
//!
//! We have talked extensively about the mechanics of logging (how to log, how to implement a logger,
//! how to filter), but we haven't really talked about **what** to log.
//!
//! ## Why do we log?
//!
//! To determine what to log, we need to stop for a second and think about the **purpose** of logging.
//! We use logging as a way to determine, **from the outside**, what is going on **inside** our
//! applications. In particular, as a way to diagnose if our applications are not behaving as
//! expected.
//!
//! The "easy" solution would be to log **everything**, a full-fidelity representation of the
//! internal state of our applications.  
//! Unfortunately, that's usually not feasible: the cost of producing and storing that information
//! would be prohibitive.
//!
//! Logs, therefore, are necessarily a **lossy** representation.
//! We need to **choose carefully** what to log in order to maximize our chances of
//! *spotting** problems and being able to **troubleshoot** them.
//!
//! ## Unit of work
//!
//! A good rule of thumb is to view your application logic as a series of **units of work**.  
//! Each unit of work has a start and an end, and may in turn contain other sub-units of work.
//!
//! For each of those unit of work, we'll surely want to know:
//! - how long it took to complete, i.e. its **duration**
//! - whether it completed successfully or not, i.e. its **outcome**
//!
//! We'll get both of these pieces of information if we emit two log records for each unit of work:
//! one at the start and one at the end.
//!
//! ### How fine-grained should the units of work be?
//!
//! If you take this approach to the extreme, you could model each function call as a unit of work.
//! While that _may_ be useful in some scenarios, it's not a good default.  
//!
//! You should consider a unit of work as a **meaningful** piece of work, one that may
//! occupy a **significant** amount of time with respect to the duration of the over-arching
//! operation (e.g. processing an incoming request).
//!
//! Let's make an example in the context of a web server.
//! Parsing a header value is probably not a good candidate for a unit of work—it's very fast and
//! it's unlikely to vary much in duration.
//! On the contrary, parsing the body of a request is a good candidate: it's likely to take a
//! significant amount of time, and it's likely to vary in duration depending on the size of the
//! body itself and the way it is being sent to the server.
//!
//! Always take the over-arching context into account when determining what should or should not
//! be treated as a unit of work.

mod logger;

pub use logger::TestLogger;

/// Given a list of order numbers, compute the total price.
///
/// # Exercise
///
/// Add log statements to `get_total` and `get_order_details`, our two units of work, to capture
/// the data points we discussed:
/// - the start and end of each unit of work
/// - the duration of each unit of work
/// - the outcome of each unit of work
///
/// Refer to the test files for the expected output format.
pub fn get_total(order_numbers: &[u64]) -> Result<u64, anyhow::Error> {
    let instant = std::time::Instant::now();
    log::info!("START - process total price");
    let mut total = 0;
    for order_number in order_numbers {
        let order_details = get_order_details(*order_number).map_err(|e| {
            log::error!(
                "END - process total price - ERROR - {}ms",
                instant.elapsed().as_millis()
            );
            e
        })?;
        total += order_details.price;
    }
    log::info!(
        "END - process total price - SUCCESS - {}ms",
        instant.elapsed().as_millis()
    );
    Ok(total)
}

pub struct OrderDetails {
    pub order_number: u64,
    pub price: u64,
}

/// A dummy function to simulate what would normally be a database query.
fn get_order_details(order_number: u64) -> Result<OrderDetails, anyhow::Error> {
    let instant = std::time::Instant::now();
    log::info!("START - retrieve order");
    if order_number % 4 == 0 {
        log::info!(
            "END - retrieve order - ERROR - {}ms",
            instant.elapsed().as_millis()
        );
        Err(anyhow::anyhow!("Failed to talk to the database"))
    } else {
        let prices = vec![999, 1089, 1029];
        log::info!(
            "END - retrieve order - SUCCESS - {}ms",
            instant.elapsed().as_millis()
        );
        Ok(OrderDetails {
            order_number,
            price: prices[order_number as usize % prices.len()],
        })
    }
}
