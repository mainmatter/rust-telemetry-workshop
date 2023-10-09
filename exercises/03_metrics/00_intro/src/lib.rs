//! # Metrics
//!
//! Early detection is a key capability if you're operating a highly available service:
//! you need to quickly spot if something is wrong in order for your first-response team
//! to engage.
//!
//! How does it work, in practice?
//!
//! # SLAs, SLOs and SLIs
//!
//! It's a common industry practice to define a set of **service level indicators** (SLIs)
//! for your service. They capture a set of **measurable** dimensions that highly correlate with your
//! users having a good experience.
//! For a payment platform, for example, you might monitor the success rate of payments,
//! the settlement time (i.e. how long it takes for the counterpart to receive the funds),
//! notification latency (i.e. how long it takes for the user to be notified of the outcome), etc.
//!
//! Once you've identified a good set of SLIs, you need to set targets.
//! If those targets are part of a contract with your users, they are usually called **service level
//! agreement** (SLAs).
//! If they are instead "self-imposed", they're called **service level objectives** (SLOs).
//! It's common to have both in a mature business, with each SLO being stricter than its SLA
//! counterpart.
//!
//! When those targets are breached, an alert is triggered and one or more operators have to
//! engage, investigate and/or try to bring the system back to a working state.
//!
//! # Measuring
//!
//! Let's stress it again: your SLIs need to be **measurable**. But how does that actually work?
//!
//! There are two common strategies:
//!
//! - You "metricise" your structured logs. E.g. you compute your API error rate by counting the number
//!   of log records where `span.kind` is set to `server` and status code is not in the 2xx/3xx
//!   range.
//! - You pre-compute the relevant metrics in your application and then export the result to a
//!   dedicated system. E.g. every time you finish handling a request, you bump a counter based
//!   on the returned status code.
//!
//! The first strategy is more flexible (e.g. you can define new alerts without having to touch
//! the application, assuming your structured logs are rich enough), but it requires you to
//! ship more data to your telemetry system where it will be processed further to output
//! the number you're looking for.
//! Depending on your scale, this may be either too slow, too wasteful or too expensive.
//!
//! The opposite applies to pre-aggregated metrics: they are cheaper to ship and store, but
//! they are rather inflexible.
//!
//! My recommendation is to follow a hybrid strategy:
//! - Collect structured logs (with a smart sampling strategy) to ask arbitrary questions about
//!   your systems and dive deep.
//! - Pre-compute all SLIs and other relevant quantities that you **know** you need to monitor.
//!
//! It's also worth pointing out that some quantities do not neatly map to logs and are instead
//! well-suited to be collected as metrics with a time-based frequency. E.g. the balance of your
//! company's operating account.
//!
//! # Next
//!
//! This part of the workshop will focus on the `metrics` crate, our tool of choice for
//! collecting and exporting pre-aggregated metrics in our Rust applications.
//!
//! Get ready!

#[cfg(test)]
mod tests {
    #[test]
    fn starting_block() {
        let msg = format!("I'm ready to learn about metrics!");
        assert_eq!(msg, "I'm ready to learn about metrics!")
    }
}
