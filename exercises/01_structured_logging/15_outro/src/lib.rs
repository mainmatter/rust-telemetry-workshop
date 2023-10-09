/// Congrats, you just made it to the end of our structure logging journey!
///
/// You should now have a good understanding of the purpose of structured logging,
/// as well as the tools required to effectively instrument your applications.
/// If you have any questions around `log` or `tracing`, this is a good time to pull me over
/// and ask them!
#[cfg(test)]
mod tests {
    #[test]
    fn starting_block() {
        let msg = format!("I'm ready to learn about errors!");
        assert_eq!(msg, "I'm ready to learn about errors!")
    }
}
