use std::fs;

use toml::Value;

pub mod constant;
pub mod filter;
pub mod form;
pub mod user_agent;
pub mod util;

pub fn load(key: &str, file: &str) {
    let buf = std::env::current_dir().unwrap().join(file);
    std::env::set_var(key, buf);
}

fn read_versions_from_file_inner<T>(
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

fn read_versions_from_file(key: &str, filename: &str) -> Vec<String> {
    match read_versions_from_file_inner(key, std::env::var(filename).unwrap().as_str()) {
        Ok(versions) => versions,
        Err(err) => {
            log::error!("Error reading versions file: {}", err);
            Vec::new()
        }
    }
}
