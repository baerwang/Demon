#[cfg(test)]
mod tests {
    use demon::{common, handler};
    use handler::robots::robots;

    #[test]
    fn robots_test() {
        common::load("user_agent", "files/user_agent.toml");
        assert_ne!(
            robots("https://www.dvwa.co.uk".to_string()).unwrap().len(),
            0
        )
    }
}
