#[cfg(test)]
mod tests {
    use demon::handler;
    use handler::robots::parse_robots;

    fn load() {
        let buf = std::env::current_dir()
            .unwrap()
            .join("files/user_agent.toml");
        std::env::set_var("user_agent", buf);
    }

    #[test]
    fn parse_robots_test() {
        load();
        assert_ne!(
            parse_robots("https://www.dvwa.co.uk".to_string())
                .unwrap()
                .len(),
            0
        )
    }
}
