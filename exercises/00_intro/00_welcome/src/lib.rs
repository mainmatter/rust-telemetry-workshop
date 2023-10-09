/// Welcome to this workshop!
///
/// # What is this about
///
/// You'll be learning how to build **observable** applications in Rust.
///
/// > Observability is about being able to ask arbitrary questions about your environment without
/// > —and this is the key part—having to know ahead of time what you wanted to ask.
/// >
/// > Honeycomb
///
/// I'll take you on a journey through the Rust ecosystem, exploring the available telemetry
/// solutions, learning how to combine them together to build a coherent and comprehensive toolkit.
///
/// # How it works
///
/// The workshop is structured as a series of workshop-runner.
/// Each koan is a standalone Rust project.
/// A koan is solved when the project compiles and all its tests pass.
/// You might have to replace intentional blanks (e.g. `____`) or fill in
/// stubbed out functionality with your own code.
///
/// The workshop-runner are ordered, building on top of each other to form a
/// coherent learning path.
///
/// Enjoy!
#[cfg(test)]
mod tests {
    #[test]
    fn starting_block() {
        let msg = format!("I'm ready to go!");
        assert_eq!(msg, "I'm ready to go!")
    }
}
