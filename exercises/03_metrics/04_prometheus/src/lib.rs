//! # Prometheus
//!
//! Honeycomb is my go-to recommendations when working with structured logs.
//! What about metrics?
//!
//! The most common option, by far, is Prometheus.
//! It's a timeseries database that has seen incredible adoption and it's now deployed in production
//! environments at all levels of scale.
//! It is often used in combination with:
//!
//! - Grafana, a dashboard tool to explore and visualize metric data
//! - Alertmanager, a system to trigger alerts based on metric queries (e.g. alert me when the %
//!   of failed requests exceeds 5% for more than 15 minutes)
//!
//! Explaining the ins and outs of these systems is outside the scope of this workshop—it'd have
//! to be a whole workshop on its own!
//!
//! But we can try to set it up once, to get a feel for what they look like.
//!
//! # Push vs Pull
//!
//! When it comes to metrics, there are two mechanisms for exporting data: push-based or pull-based.
//!
//! In the push-based model it's the application's job to send metric data to your storage system
//! of choice. This can be done, for example, over the network or by appending the data to a file.
//! The application is also in control of the exporting schedule.
//!
//! In the pull-based model, instead, the application is passive: it exposes an endpoint for
//! retrieving the current value of all recorded metrics. It is the job of an external system
//! to call (or "scrape") that endpoint to collect the data, usually on a schedule (15s is a
//! common choice for Prometheus).
//!
//! ## Prometheus
//!
//! The recommended Prometheus configuration is pull-based.
//! You can also choose to push metrics though, using their push gateway.
//!
//! In both cases, you can rely on the `metrics-exporter-prometheus` crate.  
//! It provides a recorder implementation that can be configured to either listen to incoming
//! HTTP requests or push data on a schedule.
//!
//! # What about OpenTelemetry?
//!
//! The OpenTelemetry specification covers all kinds of telemetry data, including metrics.
//! Nonetheless, the metric story (and the associated tooling) is relatively recent: you might want
//! to do due some due diligence before relying on it.

pub fn do_something(i: u64) {
    let label_value = if i % 2 == 0 { "even" } else { "odd" };
    metrics::increment_counter!("invocations", "type" => label_value)
}

#[cfg(test)]
mod tests {
    use crate::do_something;
    use std::net::{Ipv4Addr, SocketAddr};

    /// # Exercise
    ///
    /// Initialize a pull-based Prometheus recorder, listening on the address specified as input.
    fn init_test_recorder(socket_addr: SocketAddr) {
        metrics_exporter_prometheus::PrometheusBuilder::new()
            .with_http_listener(socket_addr)
            .install()
            .unwrap();
    }

    #[test]
    fn prometheus() {
        let listener_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 9091);
        init_test_recorder(listener_addr);

        for i in 0..7 {
            do_something(i);
        }

        let metrics_endpoint = format!("http://{}:{}", listener_addr.ip(), listener_addr.port());
        let response = ureq::get(&metrics_endpoint).call().unwrap();
        let body = response.into_string().unwrap();
        // This is what metrics look like when exported in Prometheus' format!
        // You can clearly see how each combination of metric name and labels value is, under the
        // hood, its own metric series.
        assert!(body.contains(r#"invocations{type="even"} 4"#));
        assert!(body.contains(r#"invocations{type="odd"} 3"#));
    }
}
