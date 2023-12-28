use crate::common::user_agent::{gen_chrome_ua, gen_edge_ua, gen_firefox_ua};
use crate::common::{read_versions_from_file, read_versions_from_file_inner};
use once_cell::sync::Lazy;

pub static UA_GENS: Lazy<Vec<fn() -> String>> =
    Lazy::new(|| vec![gen_firefox_ua, gen_chrome_ua, gen_edge_ua]);

pub static FF_VERSIONS: Lazy<Vec<f64>> = Lazy::new(|| {
    read_versions_from_file_inner("ff_versions", std::env::var("user_agent").unwrap().as_str())
        .unwrap_or_else(|err| {
            eprintln!("Error reading versions file: {}", err);
            Vec::new()
        })
});

pub static CHROME_VERSIONS: Lazy<Vec<String>> =
    Lazy::new(|| read_versions_from_file("chrome_versions", "user_agent"));

pub static EDGE_VERSIONS: Lazy<Vec<String>> =
    Lazy::new(|| read_versions_from_file("edge_versions", "user_agent"));

pub static OS_STRINGS: Lazy<Vec<String>> =
    Lazy::new(|| read_versions_from_file("os", "user_agent"));

pub static PHONE_PREFIX: Lazy<Vec<String>> =
    Lazy::new(|| read_versions_from_file("phone_prefix", "form"));

pub static NAMES_EN: Lazy<Vec<String>> = Lazy::new(|| read_versions_from_file("names_en", "form"));

pub static SURNAMES_EN: Lazy<Vec<String>> =
    Lazy::new(|| read_versions_from_file("surnames_en", "form"));

pub static NAMES: Lazy<Vec<String>> = Lazy::new(|| read_versions_from_file("names", "form"));

pub static SURNAMES: Lazy<Vec<String>> = Lazy::new(|| read_versions_from_file("surnames", "form"));

pub static EMAIL_SUFFIX: Lazy<Vec<String>> =
    Lazy::new(|| read_versions_from_file("email_suffix", "form"));
