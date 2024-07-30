mod subscriber;

pub use subscriber::init_test_subscriber;

/// # Exercise
///
/// Let's see how this works in practice.
///
/// Manipulate the spans we create in this function to match the output in the test below.
pub fn do_something() -> std::thread::JoinHandle<()> {
    let spawner_span = tracing::info_span!("spawner");

    let handle = std::thread::spawn(move || {
        let spawned_span = tracing::info_span!(parent: parent, "spawned1");
    });

    handle.join().unwrap();

    std::thread::spawn(move || {
        let spawned_span = tracing::info_span!("spawned2");
    })
}

#[cfg(test)]
mod tests {
    use super::init_test_subscriber;

    #[test]
    fn linking() {
        let logging_buffer = init_test_subscriber();

        let handle = super::do_something();
        handle.join().unwrap();

        // Check that the log output matches what we expect.
        let logging_output = logging_buffer.log_output().unwrap();
        let mut log_lines = logging_output.lines();

        log_lines.next_some().assert_eq("spawner");
        log_lines
            .next_some()
            .assert_eq("spawned1 - parent: spawner");
        log_lines
            .next_some()
            .assert_eq("spawned2 - follows_from: spawner");
    }
}
