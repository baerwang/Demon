use std::fs;

use once_cell::sync::Lazy;
use toml::Value;

use crate::common::user_agent::{gen_chrome_ua, gen_edge_ua, gen_firefox_ua};

pub mod user_agent;

pub static UA_GENS: Lazy<Vec<fn() -> String>> =
    Lazy::new(|| vec![gen_firefox_ua, gen_chrome_ua, gen_edge_ua]);

pub static FF_VERSIONS: Lazy<Vec<f64>> = Lazy::new(|| {
    read_versions_from_file("ff_versions", std::env::var("user_agent").unwrap().as_str())
        .unwrap_or_else(|err| {
            eprintln!("Error reading versions file: {}", err);
            Vec::new()
        })
});

pub static CHROME_VERSIONS: Lazy<Vec<String>> = Lazy::new(|| {
    read_versions_from_file(
        "chrome_versions",
        std::env::var("user_agent").unwrap().as_str(),
    )
    .unwrap_or_else(|err| {
        eprintln!("Error reading versions file: {}", err);
        Vec::new()
    })
});

pub static EDGE_VERSIONS: Lazy<Vec<String>> = Lazy::new(|| {
    read_versions_from_file(
        "edge_versions",
        std::env::var("user_agent").unwrap().as_str(),
    )
    .unwrap_or_else(|err| {
        eprintln!("Error reading versions file: {}", err);
        Vec::new()
    })
});

pub static OS_STRINGS: Lazy<Vec<String>> = Lazy::new(|| {
    read_versions_from_file("os", std::env::var("user_agent").unwrap().as_str()).unwrap_or_else(
        |err| {
            eprintln!("Error reading versions file: {}", err);
            Vec::new()
        },
    )
});

fn read_versions_from_file<T>(
    key: &str,
    filename: &str,
) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
{
    let file_content = fs::read_to_string(filename)?;

    let toml_value: Value = toml::from_str(&file_content)?;

    let versions = match toml_value.get(key) {
        Some(Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().and_then(|s| s.parse::<T>().ok()))
            .collect::<Vec<T>>(),
        _ => Vec::new(),
    };

    Ok(versions)
}
