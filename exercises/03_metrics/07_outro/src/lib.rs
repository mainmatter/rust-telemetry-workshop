fn affirmation() -> &'static str {
    "I know how to instrument my Rust applications!"
}

#[cfg(test)]
mod tests {
    use crate::affirmation;

    #[test]
    fn the_end() {
        assert_eq!(affirmation(), "I know how to instrument my Rust applications!")
    }
}
