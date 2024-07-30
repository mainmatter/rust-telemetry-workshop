pub fn do_something(i: u64) {
    let label_value = if i % 2 == 0 { "even" } else { "odd" };
    metrics::counter!("invocations", "type" => label_value).increment(1)
}

#[cfg(test)]
mod tests {
    use crate::do_something;
    use std::net::{Ipv4Addr, SocketAddr};

    /// # Exercise
    ///
    /// Initialize a pull-based Prometheus recorder, listening on the address specified as input.
    fn init_test_recorder(socket_addr: SocketAddr) {
        todo!()
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
