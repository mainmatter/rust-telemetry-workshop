//! Welcome to this workshop!
//! We'll be diving into the available telemetry solutions for Rust applications.
//! 
//! The workshop is structured as a series of exercises. 
//! Each exercise is a standalone Rust project. An exercise is solved
//! when it compiles and all its tests pass.
//! You will have to replace intentional blanks (e.g. `____`) or fill in
//! stubbed out functionality (e.g. `todo!()`) with your own code.
//! 
//! The exercises are ordered, building on top of each other to form a 
//! coherent learning path. 
//! 
//! Enjoy!

#[cfg(test)]
mod tests {
    #[test]
    fn starting_block() {
        let msg = format!("I'm ready to __!");
        assert_eq!(msg, "I'm ready to go!")
    }
}
