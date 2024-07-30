fn affirmation() -> &'static str {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::affirmation;

    #[test]
    fn starting_block() {
        assert_eq!(affirmation(), "I'm ready to learn about failures!")
    }
}
