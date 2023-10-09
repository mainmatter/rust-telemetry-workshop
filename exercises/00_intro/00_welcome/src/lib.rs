//! Exercises will include `TODO`, `todo!()` or `__` markers to draw your attention to the lines
//! where you need to write code.
//! You'll need to replace these markers with your own code to complete the exercise.
//! Sometimes it'll be enough to write a single line of code, other times you'll have to write
//! longer sections.
//!
//! If you get stuck for more than 10 minutes on an exercise, grab a trainer! We're here to help!
//! You can also find solutions to all exercises in the `solutions` git branch.

fn greeting() -> &'static str {
    "I'm ready to go!"
}

// Your solutions will be automatically verified by a set of tests.
// You can run these tests directly by invoking the `cargo test` command in your terminal,
// from the root of this exercise's directory. That's what the `wr` command does for you
// under the hood.
//
// ⚠️ **DO NOT MODIFY THE TESTS** ⚠️
// They are there to help you validate your solutions. You should only change the code that's being
// tested, not the tests themselves.
#[cfg(test)]
mod tests {
    use crate::greeting;

    #[test]
    fn starting_block() {
        let msg = greeting();
        assert_eq!(msg, "I'm ready to go!")
    }
}
