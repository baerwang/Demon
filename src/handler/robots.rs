use std::collections::HashSet;
use std::time;

use reqwest::header::{HeaderMap, USER_AGENT};

use crate::common;

pub fn robots(site: String) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let site = site + "/robots.txt";

    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        common::user_agent::random_user_agent().parse().unwrap(),
    );

    let rsp = reqwest::blocking::Client::new()
        .get(site)
        .timeout(time::Duration::from_secs(5))
        .headers(headers)
        .send();

    if rsp.as_ref().unwrap().status() != reqwest::StatusCode::OK {
        return Ok(HashSet::new());
    }

    let txt = rsp?.text()?;
    let values: HashSet<String> = txt
        .lines()
        .flat_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts.len() {
                2 if parts[0] == "Allow:" || parts[0] == "Disallow:" => Some(parts[1].to_string()),
                _ => None,
            }
        })
        .collect();
    Ok(values)
}

#[cfg(test)]
mod tests {
    use crate::common;
    use crate::handler::robots::robots;

    #[test]
    fn robots_test() {
        common::load("user_agent", "files/user_agent.toml");
        assert_ne!(
            robots("https://www.dvwa.co.uk".to_string()).unwrap().len(),
            0
        )
    }
}
