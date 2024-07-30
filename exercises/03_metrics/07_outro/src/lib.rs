fn affirmation() -> &'static str {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::affirmation;

    #[test]
    fn the_end() {
        assert_eq!(affirmation(), "I know how to instrument my Rust applications!")
    }
}
