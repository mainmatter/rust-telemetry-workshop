mod subscriber;

pub use subscriber::init_test_subscriber;

/// # Exercise
///
/// Let's see how this works in practice.
///
/// Manipulate the span we create in this function to match the output in the test below.
pub fn do_something() {
    let span = tracing::info_span!("My unit of work");
    todo!()
}

#[cfg(test)]
mod tests {
    use super::init_test_subscriber;
    use serde_json::json;

    #[test]
    fn failure() {
        let logging_buffer = init_test_subscriber();

        super::do_something();

        // Check that the log output matches what we expect.
        let logging_output = logging_buffer.log_output().unwrap();
        let mut log_lines = logging_output.lines();

        log_lines
            .next_some()
            .assert_json_include(json!({"message":"new"}));

        for _ in 0..3 {
            log_lines
                .next_some()
                .assert_json_include(json!({"message":"enter"}));
            log_lines
                .next_some()
                .assert_json_include(json!({"message":"exit"}));
        }

        log_lines
            .next_some()
            .assert_json_include(json!({"message":"close"}));

        log_lines.end();
    }
}
