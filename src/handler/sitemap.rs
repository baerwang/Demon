use std::collections::HashSet;
use std::time;

use reqwest::header::{HeaderMap, USER_AGENT};
use serde_derive::Deserialize;

use crate::common;

#[derive(Debug, Deserialize)]
pub struct Sitemap {
    sitemap: Option<Vec<LocUrl>>,
    url: Option<Vec<LocUrl>>,
}

impl Sitemap {
    fn values(self) -> Vec<String> {
        self.sitemap
            .into_iter()
            .chain(self.url)
            .flat_map(|items| items.into_iter())
            .map(|v| v.loc)
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct LocUrl {
    loc: String,
}

pub fn sitemap(site: String) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let site = site + "/sitemap.xml";

    let ua = common::user_agent::random_user_agent();

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, ua.parse().unwrap());

    let rsp = reqwest::blocking::Client::new()
        .get(site)
        .timeout(time::Duration::from_secs(5))
        .headers(headers)
        .send();

    if let Ok(r) = rsp {
        let txt = r.text()?;
        let sitemap: Sitemap = serde_xml_rs::from_str(&txt)?;
        let values = sitemap.values();
        let mut loc_set: HashSet<String> = HashSet::with_capacity(values.len());

        let client = reqwest::blocking::Client::new();
        values.iter().for_each(|v| {
            loc_set.extend(
                parse_sitemap(v.to_string(), ua.clone(), client.clone()).unwrap_or_default(),
            )
        });

        return Ok(loc_set);
    }

    Ok(Default::default())
}

fn parse_sitemap(
    url: String,
    ua: String,
    client: reqwest::blocking::Client,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, ua.parse().unwrap());
    let rsp = client
        .get(url)
        .timeout(time::Duration::from_secs(5))
        .headers(headers)
        .send();

    if let Ok(r) = rsp {
        if r.status() != reqwest::StatusCode::OK {
            let txt = r.text()?;
            let sitemap: Sitemap = serde_xml_rs::from_str(&txt)?;
            return Ok(sitemap.values());
        }
    }

    Ok(Default::default())
}

#[cfg(test)]
mod tests {
    use crate::common;
    use crate::handler::sitemap::parse_sitemap;
    use crate::handler::sitemap::sitemap;

    #[test]
    fn sitemap_test() {
        common::load("user_agent", "files/user_agent.toml");
        assert_ne!(sitemap("https://google.com".to_string()).unwrap().len(), 0)
    }

    #[test]
    fn parse_sitemap_test() {
        common::load("user_agent", "files/user_agent.toml");
        let ua = common::user_agent::random_user_agent();
        let client = reqwest::blocking::Client::new();
        assert_ne!(
            parse_sitemap(
                "https://www.google.com/gmail/sitemap.xml".to_string(),
                ua.clone(),
                client.clone(),
            )
            .unwrap()
            .len(),
            0
        )
    }
}
