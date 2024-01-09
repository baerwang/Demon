use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time;

use futures::stream::StreamExt;
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

pub async fn sitemap(site: String) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let site = site + "/sitemap.xml";

    let ua = common::user_agent::random_user_agent();

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, ua.parse().unwrap());

    let rsp = reqwest::Client::new()
        .get(site)
        .timeout(time::Duration::from_secs(5))
        .headers(headers)
        .send()
        .await?;

    if rsp.status() != reqwest::StatusCode::OK {
        return Ok(Default::default());
    }

    let txt = rsp.text().await?;
    let sitemap: Sitemap = serde_xml_rs::from_str(&txt)?;
    let values = sitemap.values();
    let loc_set: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

    let client = reqwest::Client::new();

    futures::stream::iter(values.into_iter().map(|v| {
        let ua_clone = ua.clone();
        let client_clone = client.clone();
        let loc_set_clone = Arc::clone(&loc_set);
        tokio::task::spawn(async move {
            let result = parse_sitemap(v.to_string(), ua_clone, client_clone).await;
            let mut inner_set = loc_set_clone.lock().unwrap();
            inner_set.extend(result.unwrap_or_default());
        })
    }))
    .for_each(|_| async {})
    .await;

    Ok(Arc::try_unwrap(loc_set).unwrap().into_inner().unwrap())
}

async fn parse_sitemap(
    url: String,
    ua: String,
    client: reqwest::Client,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, ua.parse().unwrap());
    let rsp = client
        .get(url)
        .timeout(time::Duration::from_secs(5))
        .headers(headers)
        .send()
        .await?;

    if rsp.status() == reqwest::StatusCode::OK {
        let txt = rsp.text().await?;
        let sitemap: Sitemap = serde_xml_rs::from_str(&txt)?;
        return Ok(sitemap.values());
    }
    Ok(Default::default())
}

#[cfg(test)]
mod tests {
    use crate::common;
    use crate::handler::sitemap::parse_sitemap;
    use crate::handler::sitemap::sitemap;

    #[tokio::test]
    async fn sitemap_test() {
        common::load("user_agent", "files/user_agent.toml");
        assert_ne!(
            sitemap("https://google.com".to_string())
                .await
                .unwrap()
                .len(),
            0
        )
    }

    #[tokio::test]
    async fn parse_sitemap_test() {
        common::load("user_agent", "files/user_agent.toml");
        let ua = common::user_agent::random_user_agent();
        let client = reqwest::Client::new();
        assert_ne!(
            parse_sitemap(
                "https://www.google.com/gmail/sitemap.xml".to_string(),
                ua.clone(),
                client.clone(),
            )
            .await
            .unwrap()
            .len(),
            0
        )
    }
}
