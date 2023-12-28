#[cfg(test)]
mod tests {
    use demon::{common, handler};
    use handler::robots::parse_robots;

    #[test]
    fn parse_robots_test() {
        common::load("user_agent", "files/user_agent.toml");
        assert_ne!(
            parse_robots("https://www.dvwa.co.uk".to_string())
                .unwrap()
                .len(),
            0
        )
    }
}
