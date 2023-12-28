use std::collections::HashSet;
use std::time;

use reqwest::header::{HeaderMap, USER_AGENT};

use crate::common;

pub fn parse_robots(site: String) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
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
    assert_eq!(rsp.as_ref().unwrap().status(), 200);
    let txt = rsp?.text()?;
    let allow_values: HashSet<String> = txt
        .lines()
        .flat_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts.len() {
                2 if parts[0] == "Allow:" || parts[0] == "Disallow:" => Some(parts[1].to_string()),
                _ => None,
            }
        })
        .collect();
    Ok(allow_values)
}
